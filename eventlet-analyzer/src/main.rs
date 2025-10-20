use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use colored::*;
use eventlet_analyzer::{
    analyzer::{analyze_usages, compare_snapshots, generate_analysis_summary, find_top_contributors, find_attention_needed, find_low_hanging_fruits, find_low_hanging_fruits_from_snapshots},
    cli::{display_summary_table, display_projects_table, display_top_contributors, display_attention_needed, display_insights},
    export::{export_summary, export_usages_csv, export_package, ExportFormat},
    parser::{parse_beagle_file, parse_multiple_files, filter_usages},
    AnalysisConfig,
};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(name = "eventlet-analyzer")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze a single Beagle results file
    Analyze(AnalyzeArgs),
    /// Compare two Beagle results files to show migration progress
    Compare(CompareArgs),
    /// Export analysis results to various formats
    Export(ExportArgs),
    /// Generate a comprehensive analysis package
    Package(PackageArgs),
    /// Display statistics about eventlet usage patterns
    Stats(StatsArgs),
}

#[derive(Args)]
struct AnalyzeArgs {
    /// Input Beagle results file
    #[arg(short, long)]
    input: PathBuf,

    /// Display format (table, json, yaml)
    #[arg(short, long, default_value = "table")]
    format: String,

    /// Exclude test files from analysis
    #[arg(long, default_value = "true")]
    exclude_tests: bool,

    /// Include documentation and release notes (excluded by default)
    #[arg(long)]
    include_docs: bool,

    /// Show only top N projects
    #[arg(short, long)]
    limit: Option<usize>,

    /// Show all projects (overrides limit)
    #[arg(long)]
    all: bool,

    /// Show only low hanging fruits (projects with ≤N usages and low complexity)
    #[arg(long)]
    low_hanging_fruits: bool,

    /// Maximum usages for low hanging fruits filter
    #[arg(long, default_value = "10")]
    max_usages: usize,

    /// Maximum complexity score for low hanging fruits filter
    #[arg(long, default_value = "20.0")]
    max_complexity: f64,
}

#[derive(Args)]
struct CompareArgs {
    /// Baseline Beagle results file
    #[arg(short, long)]
    baseline: PathBuf,

    /// Current Beagle results file
    #[arg(short, long)]
    current: PathBuf,

    /// Display format (table, json, yaml)
    #[arg(short, long, default_value = "table")]
    format: String,

    /// Exclude test files from analysis
    #[arg(long, default_value = "true")]
    exclude_tests: bool,

    /// Include documentation and release notes (excluded by default)
    #[arg(long)]
    include_docs: bool,

    /// Show only top N projects
    #[arg(short, long)]
    limit: Option<usize>,

    /// Show all projects (overrides limit)
    #[arg(long)]
    all: bool,

    /// Show detailed insights and recommendations
    #[arg(long)]
    insights: bool,

    /// Show only low hanging fruits (projects with ≤N usages and low complexity)
    #[arg(long)]
    low_hanging_fruits: bool,

    /// Maximum usages for low hanging fruits filter
    #[arg(long, default_value = "10")]
    max_usages: usize,

    /// Maximum complexity score for low hanging fruits filter
    #[arg(long, default_value = "20.0")]
    max_complexity: f64,
}

#[derive(Args)]
struct ExportArgs {
    /// Input Beagle results file (for single file analysis)
    #[arg(short, long)]
    input: Option<PathBuf>,

    /// Baseline file (for comparison)
    #[arg(short, long)]
    baseline: Option<PathBuf>,

    /// Current file (for comparison)
    #[arg(short, long)]
    current: Option<PathBuf>,

    /// Export format (json, json-pretty, csv, yaml, hugo, jekyll)
    #[arg(short, long, default_value = "json")]
    format: String,

    /// Output file path
    #[arg(short, long)]
    output: PathBuf,

    /// Exclude test files from analysis
    #[arg(long, default_value = "true")]
    exclude_tests: bool,

    /// Include documentation and release notes (excluded by default)
    #[arg(long)]
    include_docs: bool,
}

#[derive(Args)]
struct PackageArgs {
    /// Baseline Beagle results file
    #[arg(short, long)]
    baseline: PathBuf,

    /// Current Beagle results file
    #[arg(short, long)]
    current: PathBuf,

    /// Output directory for the package
    #[arg(short, long, default_value = "eventlet-analysis")]
    output: PathBuf,

    /// Exclude test files from analysis
    #[arg(long, default_value = "true")]
    exclude_tests: bool,

    /// Include documentation and release notes (excluded by default)
    #[arg(long)]
    include_docs: bool,
}

#[derive(Args)]
struct StatsArgs {
    /// Input files (can be multiple)
    #[arg(short, long)]
    input: Vec<PathBuf>,

    /// Show usage type breakdown
    #[arg(long)]
    usage_types: bool,

    /// Show file statistics
    #[arg(long)]
    files: bool,

    /// Minimum usage count threshold
    #[arg(long, default_value = "1")]
    min_usages: usize,

    /// Exclude test files from analysis
    #[arg(long, default_value = "true")]
    exclude_tests: bool,

    /// Include documentation and release notes (excluded by default)
    #[arg(long)]
    include_docs: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze(args) => analyze_command(args),
        Commands::Compare(args) => compare_command(args),
        Commands::Export(args) => export_command(args),
        Commands::Package(args) => package_command(args),
        Commands::Stats(args) => stats_command(args),
    }
}

fn analyze_command(args: AnalyzeArgs) -> Result<()> {
    let start_time = Instant::now();

    println!("{}", "🔍 Analyzing eventlet usage...".bold().blue());

    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} {msg}")
        .unwrap());
    pb.set_message("Parsing Beagle results...");

    let usages = parse_beagle_file(&args.input)
        .with_context(|| format!("Failed to parse input file: {:?}", args.input))?;

    pb.set_message("Filtering usages...");
    let filtered_usages = filter_usages(usages, args.exclude_tests, !args.include_docs);

    pb.set_message("Analyzing projects...");
    let config = AnalysisConfig::default();
    let snapshots = analyze_usages(&filtered_usages, &config);

    pb.finish_with_message("✅ Analysis complete!");

    let duration = start_time.elapsed();
    let filter_info = if args.include_docs {
        " (including docs/release notes)".dimmed()
    } else {
        " (excluding docs/release notes)".dimmed()
    };
    println!("Processed {} usages across {} projects in {:.2}s{}",
             filtered_usages.len().to_string().green(),
             snapshots.len().to_string().blue(),
             duration.as_secs_f64(),
             filter_info);

    match args.format.as_str() {
        "table" => {
            let projects: Vec<_> = if args.low_hanging_fruits {
                // Filter for low hanging fruits
                let fruits = find_low_hanging_fruits_from_snapshots(&snapshots, args.max_usages, args.max_complexity);
                let mut sorted: Vec<_> = fruits.into_iter().cloned().collect();
                sorted.sort_by(|a, b| a.total_usages.cmp(&b.total_usages)); // Sort by ascending usage count
                println!("\n{}", format!("🍇 Low Hanging Fruits (≤{} usages, ≤{:.1} complexity)", args.max_usages, args.max_complexity).bold().green());
                sorted
            } else {
                let mut projects: Vec<_> = snapshots.into_values().collect();
                projects.sort_by(|a, b| b.total_usages.cmp(&a.total_usages));
                println!("\n{}", "📊 Project Usage Summary".bold());
                projects
            };

            let limit = if args.all { None } else { args.limit.or(Some(20)) };

            if !args.low_hanging_fruits && limit.is_some() {
                println!("Showing top {} projects by usage count", limit.unwrap());
            } else if args.low_hanging_fruits {
                println!("Found {} low hanging fruits projects", projects.len());
            }

            // Convert snapshots to comparison results for display
            let comparisons: Vec<_> = projects.into_iter().take(limit.unwrap_or(usize::MAX))
                .map(|p| eventlet_analyzer::ComparisonResult {
                    project_name: p.project_name,
                    baseline_usages: p.total_usages,
                    current_usages: p.total_usages,
                    usages_removed: 0,
                    progress_percentage: 0.0,
                    migration_status: eventlet_analyzer::MigrationStatus::Stalled,
                    complexity_change: 0.0,
                })
                .collect();

            display_projects_table(&comparisons, limit);
        }
        "json" => {
            println!("{}", serde_json::to_string_pretty(&snapshots)?);
        }
        "yaml" => {
            println!("{}", serde_yaml::to_string(&snapshots)?);
        }
        _ => {
            eprintln!("{}", format!("Unsupported format: {}", args.format).red());
            std::process::exit(1);
        }
    }

    Ok(())
}

fn compare_command(args: CompareArgs) -> Result<()> {
    let start_time = Instant::now();

    println!("{}", "🔄 Comparing eventlet usage between files...".bold().blue());

    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} {msg}")
        .unwrap());

    // Parse baseline file
    pb.set_message("Parsing baseline file...");
    let baseline_usages = parse_beagle_file(&args.baseline)
        .with_context(|| format!("Failed to parse baseline file: {:?}", args.baseline))?;
    let baseline_filtered = filter_usages(baseline_usages, args.exclude_tests, !args.include_docs);

    // Parse current file
    pb.set_message("Parsing current file...");
    let current_usages = parse_beagle_file(&args.current)
        .with_context(|| format!("Failed to parse current file: {:?}", args.current))?;
    let current_filtered = filter_usages(current_usages, args.exclude_tests, !args.include_docs);

    // Analyze both datasets
    pb.set_message("Analyzing baseline...");
    let config = AnalysisConfig::default();
    let baseline_snapshots = analyze_usages(&baseline_filtered, &config);

    pb.set_message("Analyzing current state...");
    let current_snapshots = analyze_usages(&current_filtered, &config);

    // Compare snapshots
    pb.set_message("Generating comparison...");
    let comparisons = compare_snapshots(&baseline_snapshots, &current_snapshots);

    // Generate summary
    let summary = generate_analysis_summary(
        args.baseline.to_string_lossy().to_string(),
        args.current.to_string_lossy().to_string(),
        comparisons,
    );

    pb.finish_with_message("✅ Comparison complete!");

    let duration = start_time.elapsed();
    let filter_info = if args.include_docs {
        " (including docs/release notes)".dimmed()
    } else {
        " (excluding docs/release notes)".dimmed()
    };
    println!("Analyzed {} → {} usages across {} projects in {:.2}s{}",
             baseline_filtered.len().to_string().yellow(),
             current_filtered.len().to_string().green(),
             summary.total_projects.to_string().blue(),
             duration.as_secs_f64(),
             filter_info);

    match args.format.as_str() {
        "table" => {
            let sorted_projects = if args.low_hanging_fruits {
                // Filter for low hanging fruits
                let fruits = find_low_hanging_fruits(&summary.projects, args.max_usages, args.max_complexity);
                let mut sorted: Vec<_> = fruits.into_iter().cloned().collect();
                sorted.sort_by(|a, b| a.current_usages.cmp(&b.current_usages)); // Sort by ascending current usage count

                println!("\n{}", format!("🍇 Low Hanging Fruits (≤{} usages, ≤{:.1} complexity change)", args.max_usages, args.max_complexity).bold().green());
                println!("Found {} low hanging fruits projects\n", sorted.len());
                sorted
            } else {
                display_summary_table(&summary);
                let mut sorted_projects = summary.projects.clone();
                sorted_projects.sort_by(|a, b| b.usages_removed.cmp(&a.usages_removed));
                sorted_projects
            };

            let limit = if args.all { None } else { args.limit.or(Some(20)) };
            display_projects_table(&sorted_projects, limit);

            if !args.low_hanging_fruits {
                // Show top contributors
                let top_contributors = find_top_contributors(&summary.projects, 5);
                display_top_contributors(&top_contributors, "🏆 Top Contributors (Most Usages Removed)");

                // Show projects needing attention
                let attention_needed = find_attention_needed(&summary.projects, args.limit.unwrap_or(10));
                display_attention_needed(&attention_needed);

                if args.insights {
                    display_insights(&summary);
                }
            }
        }
        "json" => {
            println!("{}", serde_json::to_string_pretty(&summary)?);
        }
        "yaml" => {
            println!("{}", serde_yaml::to_string(&summary)?);
        }
        _ => {
            eprintln!("{}", format!("Unsupported format: {}", args.format).red());
            std::process::exit(1);
        }
    }

    Ok(())
}

fn export_command(args: ExportArgs) -> Result<()> {
    let format = ExportFormat::from_str(&args.format)
        .with_context(|| format!("Invalid export format: {}", args.format))?;

    let config = AnalysisConfig::default();

    match (args.input, args.baseline, args.current) {
        (Some(input), None, None) => {
            // Single file analysis
            println!("{}", "📤 Exporting single file analysis...".bold().blue());

            let usages = parse_beagle_file(&input)?;
            let filtered_usages = filter_usages(usages, args.exclude_tests, !args.include_docs);
            let _snapshots = analyze_usages(&filtered_usages, &config);

            export_usages_csv(&filtered_usages, &args.output)?;
            println!("✅ Exported to: {}", args.output.display().to_string().green());
        }
        (None, Some(baseline), Some(current)) => {
            // Comparison analysis
            println!("{}", "📤 Exporting comparison analysis...".bold().blue());

            let baseline_usages = parse_beagle_file(&baseline)?;
            let current_usages = parse_beagle_file(&current)?;
            let baseline_filtered = filter_usages(baseline_usages, args.exclude_tests, !args.include_docs);
            let current_filtered = filter_usages(current_usages, args.exclude_tests, !args.include_docs);

            let baseline_snapshots = analyze_usages(&baseline_filtered, &config);
            let current_snapshots = analyze_usages(&current_filtered, &config);
            let comparisons = compare_snapshots(&baseline_snapshots, &current_snapshots);

            let summary = generate_analysis_summary(
                baseline.to_string_lossy().to_string(),
                current.to_string_lossy().to_string(),
                comparisons,
            );

            export_summary(&summary, &format, &args.output)?;
            println!("✅ Exported to: {}", args.output.display().to_string().green());
        }
        _ => {
            eprintln!("{}", "Error: Provide either --input for single file analysis or both --baseline and --current for comparison".red());
            std::process::exit(1);
        }
    }

    Ok(())
}

fn package_command(args: PackageArgs) -> Result<()> {
    println!("{}", "📦 Generating comprehensive analysis package...".bold().blue());

    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} {msg}")
        .unwrap());

    let config = AnalysisConfig::default();

    // Parse files
    pb.set_message("Parsing files...");
    let baseline_usages = parse_beagle_file(&args.baseline)?;
    let current_usages = parse_beagle_file(&args.current)?;
    let baseline_filtered = filter_usages(baseline_usages, args.exclude_tests, !args.include_docs);
    let current_filtered = filter_usages(current_usages, args.exclude_tests, !args.include_docs);

    // Analyze and compare
    pb.set_message("Analyzing data...");
    let baseline_snapshots = analyze_usages(&baseline_filtered, &config);
    let current_snapshots = analyze_usages(&current_filtered, &config);
    let comparisons = compare_snapshots(&baseline_snapshots, &current_snapshots);

    let summary = generate_analysis_summary(
        args.baseline.to_string_lossy().to_string(),
        args.current.to_string_lossy().to_string(),
        comparisons,
    );

    // Export package
    pb.set_message("Creating export package...");
    export_package(&summary, &baseline_filtered, &current_filtered, &args.output)?;

    pb.finish_with_message("✅ Package created!");

    println!("📦 Analysis package created in: {}", args.output.display().to_string().green());
    println!("📁 Contains: JSON, YAML, CSV, and Markdown files");
    println!("📄 See index.md for file descriptions");

    Ok(())
}

fn stats_command(args: StatsArgs) -> Result<()> {
    println!("{}", "📈 Generating usage statistics...".bold().blue());

    if args.input.is_empty() {
        eprintln!("{}", "Error: At least one input file is required".red());
        std::process::exit(1);
    }

    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} {msg}")
        .unwrap());

    pb.set_message("Parsing input files...");
    let all_usages = parse_multiple_files(&args.input)?;

    for (file_path, usages) in &all_usages {
        println!("\n{} {}", "📄".blue(), file_path.bold());

        // Apply filtering
        let filtered_usages = filter_usages(usages.clone(), args.exclude_tests, !args.include_docs);
        println!("  Total usages: {} (filtered: {})",
                 usages.len().to_string().yellow(),
                 filtered_usages.len().to_string().green());

        let config = AnalysisConfig::default();
        let snapshots = analyze_usages(&filtered_usages, &config);
        println!("  Projects: {}", snapshots.len().to_string().blue());

        let high_usage_projects = snapshots.values()
            .filter(|p| p.total_usages >= args.min_usages)
            .count();
        println!("  Projects with ≥{} usages: {}", args.min_usages, high_usage_projects.to_string().yellow());

        if args.files {
            let total_files: usize = snapshots.values().map(|p| p.files_affected).sum();
            println!("  Files affected: {}", total_files.to_string().cyan());
        }
    }

    pb.finish_with_message("✅ Statistics complete!");

    Ok(())
}
