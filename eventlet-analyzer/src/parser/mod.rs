use crate::{EventletUsage, UsageType};
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use url::Url;

lazy_static! {
    /// Regex patterns for different eventlet usage types
    static ref USAGE_PATTERNS: HashMap<UsageType, Regex> = {
        let mut patterns = HashMap::new();

        patterns.insert(
            UsageType::Import,
            Regex::new(r"import\s+eventlet|from\s+eventlet").unwrap()
        );

        patterns.insert(
            UsageType::MonkeyPatch,
            Regex::new(r"eventlet\.monkey_patch\(\)").unwrap()
        );

        patterns.insert(
            UsageType::Spawn,
            Regex::new(r"eventlet\.spawn\(").unwrap()
        );

        patterns.insert(
            UsageType::Listen,
            Regex::new(r"eventlet\.listen\(").unwrap()
        );

        patterns.insert(
            UsageType::Wsgi,
            Regex::new(r"eventlet\.wsgi").unwrap()
        );

        patterns.insert(
            UsageType::Executor,
            Regex::new(r#"executor\s*=\s*['"]eventlet['"]"#).unwrap()
        );

        patterns.insert(
            UsageType::Sleep,
            Regex::new(r"eventlet\.sleep\(").unwrap()
        );

        patterns.insert(
            UsageType::Pool,
            Regex::new(r"eventlet\.(GreenPool|pool)").unwrap()
        );

        patterns
    };

    /// Regex for parsing Beagle output lines
    static ref BEAGLE_LINE_REGEX: Regex = Regex::new(
        r"^(https://[^#]+)#n(\d+)\s*:\s*(.+)$"
    ).unwrap();
}

/// Parse eventlet usage from a single line of Beagle output
fn parse_beagle_line(line: &str) -> Result<EventletUsage> {
    let captures = BEAGLE_LINE_REGEX
        .captures(line.trim())
        .context("Failed to parse Beagle output line format")?;

    let file_url = captures[1].to_string();
    let line_number: u32 = captures[2].parse()
        .context("Failed to parse line number")?;
    let code_snippet = captures[3].to_string();

    // Extract project and repository information from URL
    let url = Url::parse(&file_url)
        .context("Failed to parse file URL")?;

    let path_segments: Vec<&str> = url.path_segments()
        .context("Invalid URL path")?
        .collect();

    // Handle opendev.org URLs: /openstack/project-name/src/branch/master/...
    let (repository, project_name, file_path) = if url.host_str() == Some("opendev.org") {
        if path_segments.len() < 2 {
            anyhow::bail!("OpenDev URL path too short to extract project info");
        }
        let org = path_segments[0]; // "openstack"
        let project = path_segments[1]; // project name
        let repo = format!("{}/{}", org, project);
        let remaining_path = if path_segments.len() > 5 {
            // Skip "src", "branch", "master" parts
            path_segments[5..].join("/")
        } else {
            "".to_string()
        };
        (repo, project.to_string(), remaining_path)
    } else {
        // Handle GitHub URLs: /org/project/...
        if path_segments.len() < 2 {
            anyhow::bail!("GitHub URL path too short to extract project info");
        }
        let repo = format!("{}/{}", path_segments[0], path_segments[1]);
        let project = path_segments[1].to_string();
        let remaining_path = path_segments[2..].join("/");
        (repo, project, remaining_path)
    };

    // Determine usage type from code snippet
    let usage_type = determine_usage_type(&code_snippet);

    Ok(EventletUsage {
        file_url,
        line_number,
        project_name,
        repository,
        usage_type,
        code_snippet,
        file_path,
    })
}

/// Determine the type of eventlet usage from code snippet
fn determine_usage_type(code_snippet: &str) -> UsageType {
    for (usage_type, pattern) in USAGE_PATTERNS.iter() {
        if pattern.is_match(code_snippet) {
            return usage_type.clone();
        }
    }

    // If no specific pattern matches, create an Other variant
    UsageType::Other(extract_usage_pattern(code_snippet))
}

/// Extract a representative pattern for unknown usage types
fn extract_usage_pattern(code_snippet: &str) -> String {
    // Look for eventlet method calls
    if let Some(caps) = Regex::new(r"eventlet\.(\w+)").unwrap().captures(code_snippet) {
        return format!("eventlet.{}", &caps[1]);
    }

    // Fallback to first 50 characters
    code_snippet.chars().take(50).collect()
}

/// Parse a Beagle results file and extract all eventlet usages
pub fn parse_beagle_file<P: AsRef<Path>>(file_path: P) -> Result<Vec<EventletUsage>> {
    let content = fs::read_to_string(file_path.as_ref())
        .with_context(|| format!("Failed to read file: {:?}", file_path.as_ref()))?;

    let usages: Result<Vec<EventletUsage>> = content
        .par_lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_beagle_line)
        .collect();

    usages
}

/// Parse multiple Beagle files in parallel
pub fn parse_multiple_files<P: AsRef<Path> + Sync>(file_paths: &[P]) -> Result<HashMap<String, Vec<EventletUsage>>> {
    let results: Result<Vec<(String, Vec<EventletUsage>)>> = file_paths
        .par_iter()
        .map(|path| {
            let path_str = path.as_ref().to_string_lossy().to_string();
            let usages = parse_beagle_file(path)?;
            Ok((path_str, usages))
        })
        .collect();

    results.map(|vec| vec.into_iter().collect())
}

/// Filter usages based on criteria (exclude tests, docs, release notes)
pub fn filter_usages(
    usages: Vec<EventletUsage>,
    exclude_tests: bool,
    exclude_docs: bool
) -> Vec<EventletUsage> {
    usages
        .into_par_iter()
        .filter(|usage| {
            // Filter tests if requested
            let is_test = exclude_tests && (
                usage.file_path.contains("/test") ||
                usage.file_path.contains("/tests") ||
                usage.file_path.ends_with("_test.py") ||
                usage.file_path.ends_with("test_.py")
            );

            // Filter documentation and release notes if requested
            let is_docs = exclude_docs && (
                usage.file_path.contains("/doc") ||
                usage.file_path.contains("/docs") ||
                usage.file_path.contains("/releasenotes") ||
                usage.file_path.contains("/release-notes") ||
                usage.file_path.contains("/release_notes") ||
                usage.file_path.contains("/locale/") ||
                usage.file_path.contains("/translations/") ||
                usage.file_path.contains("/po/") ||
                usage.file_path.ends_with(".po") ||
                usage.file_path.ends_with(".pot") ||
                usage.file_path.ends_with(".rst") ||
                usage.file_path.ends_with(".md") ||
                usage.file_path.contains("README") ||
                usage.file_path.contains("CHANGELOG") ||
                usage.file_path.contains("HISTORY") ||
                usage.file_path.contains("/notes/") ||
                // Specific patterns for release notes
                usage.file_path.contains("releasenotes/notes/") ||
                usage.file_path.contains("releasenotes/source/")
            );

            !is_test && !is_docs
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_beagle_line() {
        let line = "https://github.com/openstack/aodh/blob/master/aodh/service.py#45 : import eventlet";
        let usage = parse_beagle_line(line).unwrap();

        assert_eq!(usage.project_name, "aodh");
        assert_eq!(usage.repository, "openstack/aodh");
        assert_eq!(usage.line_number, 45);
        assert_eq!(usage.usage_type, UsageType::Import);
    }

    #[test]
    fn test_determine_usage_type() {
        assert_eq!(determine_usage_type("import eventlet"), UsageType::Import);
        assert_eq!(determine_usage_type("eventlet.monkey_patch()"), UsageType::MonkeyPatch);
        assert_eq!(determine_usage_type("eventlet.spawn(func)"), UsageType::Spawn);
        assert_eq!(determine_usage_type("executor='eventlet'"), UsageType::Executor);
    }
}