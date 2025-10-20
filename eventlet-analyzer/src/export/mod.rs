use crate::{AnalysisSummary, ComparisonResult, EventletUsage};
use anyhow::{Context, Result};
use serde_json;
use std::fs;
use std::path::Path;

/// Export formats supported by the analyzer
#[derive(Debug, Clone)]
pub enum ExportFormat {
    Json,
    JsonPretty,
    Csv,
    Yaml,
    Hugo,
    Jekyll,
}

impl ExportFormat {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "json" => Ok(ExportFormat::Json),
            "json-pretty" | "pretty" => Ok(ExportFormat::JsonPretty),
            "csv" => Ok(ExportFormat::Csv),
            "yaml" | "yml" => Ok(ExportFormat::Yaml),
            "hugo" => Ok(ExportFormat::Hugo),
            "jekyll" => Ok(ExportFormat::Jekyll),
            _ => Err(anyhow::anyhow!("Unsupported export format: {}", s)),
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            ExportFormat::Json | ExportFormat::JsonPretty => "json",
            ExportFormat::Csv => "csv",
            ExportFormat::Yaml => "yaml",
            ExportFormat::Hugo | ExportFormat::Jekyll => "md",
        }
    }
}

/// Export analysis summary to specified format
pub fn export_summary<P: AsRef<Path>>(
    summary: &AnalysisSummary,
    format: &ExportFormat,
    output_path: P,
) -> Result<()> {
    let content = match format {
        ExportFormat::Json => export_json(summary, false)?,
        ExportFormat::JsonPretty => export_json(summary, true)?,
        ExportFormat::Csv => export_csv_summary(summary)?,
        ExportFormat::Yaml => export_yaml(summary)?,
        ExportFormat::Hugo => export_hugo(summary)?,
        ExportFormat::Jekyll => export_jekyll(summary)?,
    };

    fs::write(&output_path, content)
        .with_context(|| format!("Failed to write to {:?}", output_path.as_ref()))?;

    Ok(())
}

/// Export detailed project data to CSV
pub fn export_projects_csv<P: AsRef<Path>>(
    projects: &[ComparisonResult],
    output_path: P,
) -> Result<()> {
    let mut wtr = csv::Writer::from_path(output_path)?;

    // Write header
    wtr.write_record(&[
        "project_name",
        "baseline_usages",
        "current_usages",
        "usages_removed",
        "progress_percentage",
        "migration_status",
        "complexity_change",
    ])?;

    // Write data
    for project in projects {
        wtr.write_record(&[
            &project.project_name,
            &project.baseline_usages.to_string(),
            &project.current_usages.to_string(),
            &project.usages_removed.to_string(),
            &format!("{:.2}", project.progress_percentage),
            &format!("{:?}", project.migration_status),
            &format!("{:.2}", project.complexity_change),
        ])?;
    }

    wtr.flush()?;
    Ok(())
}

/// Export raw usage data to CSV
pub fn export_usages_csv<P: AsRef<Path>>(
    usages: &[EventletUsage],
    output_path: P,
) -> Result<()> {
    let mut wtr = csv::Writer::from_path(output_path)?;

    // Write header
    wtr.write_record(&[
        "project_name",
        "repository",
        "file_path",
        "line_number",
        "usage_type",
        "code_snippet",
        "file_url",
    ])?;

    // Write data
    for usage in usages {
        wtr.write_record(&[
            &usage.project_name,
            &usage.repository,
            &usage.file_path,
            &usage.line_number.to_string(),
            &format!("{:?}", usage.usage_type),
            &usage.code_snippet,
            &usage.file_url,
        ])?;
    }

    wtr.flush()?;
    Ok(())
}

/// Export to JSON format
fn export_json(summary: &AnalysisSummary, pretty: bool) -> Result<String> {
    if pretty {
        serde_json::to_string_pretty(summary)
    } else {
        serde_json::to_string(summary)
    }
    .context("Failed to serialize to JSON")
}

/// Export to YAML format
fn export_yaml(summary: &AnalysisSummary) -> Result<String> {
    serde_yaml::to_string(summary).context("Failed to serialize to YAML")
}

/// Export summary to CSV format
fn export_csv_summary(summary: &AnalysisSummary) -> Result<String> {
    let mut output = String::new();

    // Summary section
    output.push_str("metric,value\n");
    output.push_str(&format!("analysis_date,{}\n", summary.analysis_date.format("%Y-%m-%d %H:%M UTC")));
    output.push_str(&format!("baseline_file,{}\n", summary.baseline_file));
    output.push_str(&format!("current_file,{}\n", summary.current_file));
    output.push_str(&format!("total_projects,{}\n", summary.total_projects));
    output.push_str(&format!("fully_migrated,{}\n", summary.fully_migrated));
    output.push_str(&format!("in_progress,{}\n", summary.in_progress));
    output.push_str(&format!("stalled,{}\n", summary.stalled));
    output.push_str(&format!("regressed,{}\n", summary.regressed));
    output.push_str(&format!("new_projects,{}\n", summary.new_projects));
    output.push_str(&format!("overall_progress,{:.2}\n", summary.overall_progress));
    output.push_str(&format!("total_usages_removed,{}\n", summary.total_usages_removed));

    Ok(output)
}

/// Export for Hugo static site generator
fn export_hugo(summary: &AnalysisSummary) -> Result<String> {
    let mut content = String::new();

    // Hugo front matter
    content.push_str("---\n");
    content.push_str(&format!("title: \"Eventlet Migration Progress - {}\"\n",
                             summary.analysis_date.format("%Y-%m-%d")));
    content.push_str(&format!("date: {}\n", summary.analysis_date.to_rfc3339()));
    content.push_str("layout: analysis\n");
    content.push_str("type: report\n");
    content.push_str(&format!("overall_progress: {:.1}\n", summary.overall_progress));
    content.push_str(&format!("total_projects: {}\n", summary.total_projects));
    content.push_str(&format!("fully_migrated: {}\n", summary.fully_migrated));
    content.push_str(&format!("in_progress: {}\n", summary.in_progress));
    content.push_str("---\n\n");

    // Content
    content.push_str("# OpenStack Eventlet Migration Analysis\n\n");
    content.push_str(&format!("**Analysis Date:** {}\n\n",
                             summary.analysis_date.format("%Y-%m-%d %H:%M UTC")));
    content.push_str(&format!("**Overall Progress:** {:.1}%\n\n", summary.overall_progress));

    // Statistics
    content.push_str("## Summary Statistics\n\n");
    content.push_str(&format!("- **Total Projects:** {}\n", summary.total_projects));
    content.push_str(&format!("- **Fully Migrated:** {} ({:.1}%)\n",
                             summary.fully_migrated,
                             summary.fully_migrated as f64 / summary.total_projects as f64 * 100.0));
    content.push_str(&format!("- **In Progress:** {} ({:.1}%)\n",
                             summary.in_progress,
                             summary.in_progress as f64 / summary.total_projects as f64 * 100.0));
    content.push_str(&format!("- **Stalled:** {} ({:.1}%)\n",
                             summary.stalled,
                             summary.stalled as f64 / summary.total_projects as f64 * 100.0));
    content.push_str(&format!("- **Total Usages Removed:** {}\n\n", summary.total_usages_removed));

    // Data for JavaScript charts
    content.push_str("## Project Data\n\n");
    content.push_str("```json\n");
    content.push_str(&serde_json::to_string_pretty(&summary.projects)?);
    content.push_str("\n```\n");

    Ok(content)
}

/// Export for Jekyll static site generator
fn export_jekyll(summary: &AnalysisSummary) -> Result<String> {
    let mut content = String::new();

    // Jekyll front matter
    content.push_str("---\n");
    content.push_str("layout: analysis\n");
    content.push_str(&format!("title: \"Eventlet Migration Progress - {}\"\n",
                             summary.analysis_date.format("%Y-%m-%d")));
    content.push_str(&format!("date: {}\n", summary.analysis_date.format("%Y-%m-%d %H:%M:%S %z")));
    content.push_str("categories: [analysis, eventlet, migration]\n");
    content.push_str(&format!("overall_progress: {:.1}\n", summary.overall_progress));
    content.push_str(&format!("total_projects: {}\n", summary.total_projects));
    content.push_str("data:\n");
    content.push_str(&format!("  baseline_file: \"{}\"\n", summary.baseline_file));
    content.push_str(&format!("  current_file: \"{}\"\n", summary.current_file));
    content.push_str(&format!("  fully_migrated: {}\n", summary.fully_migrated));
    content.push_str(&format!("  in_progress: {}\n", summary.in_progress));
    content.push_str(&format!("  stalled: {}\n", summary.stalled));
    content.push_str(&format!("  regressed: {}\n", summary.regressed));
    content.push_str(&format!("  new_projects: {}\n", summary.new_projects));
    content.push_str("---\n\n");

    // Content similar to Hugo but Jekyll-formatted
    content.push_str("# OpenStack Eventlet Migration Analysis\n\n");
    content.push_str(&format!("**Analysis Date:** {{ page.date | date: \"%Y-%m-%d %H:%M UTC\" }}\n\n"));

    // Include Jekyll liquid templates for dynamic content
    content.push_str("{% assign progress = page.overall_progress %}\n");
    content.push_str("**Overall Progress:** {{ progress }}%\n\n");

    content.push_str("## Migration Status\n\n");
    content.push_str("{% assign total = page.total_projects %}\n");
    content.push_str("- **Fully Migrated:** {{ page.data.fully_migrated }} ({{ page.data.fully_migrated | times: 100 | divided_by: total | round: 1 }}%)\n");
    content.push_str("- **In Progress:** {{ page.data.in_progress }} ({{ page.data.in_progress | times: 100 | divided_by: total | round: 1 }}%)\n");
    content.push_str("- **Stalled:** {{ page.data.stalled }} ({{ page.data.stalled | times: 100 | divided_by: total | round: 1 }}%)\n\n");

    Ok(content)
}

/// Create a comprehensive export package with multiple formats
pub fn export_package<P: AsRef<Path>>(
    summary: &AnalysisSummary,
    usages_baseline: &[EventletUsage],
    usages_current: &[EventletUsage],
    output_dir: P,
) -> Result<()> {
    let output_dir = output_dir.as_ref();
    fs::create_dir_all(output_dir)?;

    let timestamp = summary.analysis_date.format("%Y%m%d_%H%M%S").to_string();

    // Export summary in multiple formats
    export_summary(summary, &ExportFormat::JsonPretty,
                  output_dir.join(format!("summary_{}.json", timestamp)))?;
    export_summary(summary, &ExportFormat::Yaml,
                  output_dir.join(format!("summary_{}.yaml", timestamp)))?;
    export_summary(summary, &ExportFormat::Hugo,
                  output_dir.join(format!("analysis_{}.md", timestamp)))?;

    // Export detailed data
    export_projects_csv(&summary.projects,
                       output_dir.join(format!("projects_{}.csv", timestamp)))?;

    if !usages_baseline.is_empty() {
        export_usages_csv(usages_baseline,
                         output_dir.join(format!("usages_baseline_{}.csv", timestamp)))?;
    }

    if !usages_current.is_empty() {
        export_usages_csv(usages_current,
                         output_dir.join(format!("usages_current_{}.csv", timestamp)))?;
    }

    // Create index file
    let index_content = create_export_index(summary, &timestamp);
    fs::write(output_dir.join("index.md"), index_content)?;

    Ok(())
}

/// Create an index file for the export package
fn create_export_index(summary: &AnalysisSummary, timestamp: &str) -> String {
    format!(
        r#"# Eventlet Migration Analysis Export

Generated: {}

## Files in this export:

- `summary_{}.json` - Complete analysis summary (JSON)
- `summary_{}.yaml` - Complete analysis summary (YAML)
- `analysis_{}.md` - Hugo/Jekyll compatible markdown
- `projects_{}.csv` - Detailed project comparison data
- `usages_baseline_{}.csv` - Baseline usage data (if available)
- `usages_current_{}.csv` - Current usage data (if available)

## Summary Statistics

- **Total Projects:** {}
- **Overall Progress:** {:.1}%
- **Fully Migrated:** {}
- **In Progress:** {}
- **Stalled:** {}
- **Total Usages Removed:** {}

Generated by eventlet-analyzer v{}
"#,
        summary.analysis_date.format("%Y-%m-%d %H:%M UTC"),
        timestamp, timestamp, timestamp, timestamp, timestamp, timestamp,
        summary.total_projects,
        summary.overall_progress,
        summary.fully_migrated,
        summary.in_progress,
        summary.stalled,
        summary.total_usages_removed,
        env!("CARGO_PKG_VERSION")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_export_format_from_str() {
        assert!(matches!(ExportFormat::from_str("json").unwrap(), ExportFormat::Json));
        assert!(matches!(ExportFormat::from_str("CSV").unwrap(), ExportFormat::Csv));
        assert!(ExportFormat::from_str("invalid").is_err());
    }

    #[test]
    fn test_export_format_extension() {
        assert_eq!(ExportFormat::Json.extension(), "json");
        assert_eq!(ExportFormat::Csv.extension(), "csv");
        assert_eq!(ExportFormat::Hugo.extension(), "md");
    }
}