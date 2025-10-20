use crate::{AnalysisSummary, ComparisonResult, MigrationStatus};
use colored::*;
use tabled::{
    builder::Builder,
    settings::{style::Style, Alignment, Color, Modify, object::Rows, object::Columns},
};

/// Display analysis results in a formatted table
pub fn display_summary_table(summary: &AnalysisSummary) {
    println!("\n{}", "📊 OpenStack Eventlet Migration Analysis".bold().blue());
    println!("{}", "━".repeat(60).dimmed());

    // Summary statistics
    println!("\n{}", "📈 Overall Progress".bold());
    println!("Analysis Date: {}", summary.analysis_date.format("%Y-%m-%d %H:%M UTC"));
    println!("Baseline File: {}", summary.baseline_file.green());
    println!("Current File:  {}", summary.current_file.green());
    println!();

    // Progress bar
    let progress_bar = create_progress_bar(summary.overall_progress);
    println!(
        "Overall Progress: {} {:.1}% ({}/{} usages removed)",
        progress_bar,
        summary.overall_progress,
        summary.total_usages_removed,
        summary.total_usages_removed + summary.projects.iter().map(|p| p.current_usages as i32).sum::<i32>()
    );
    println!();

    // Status breakdown
    println!("{}", "📋 Project Status Breakdown".bold());
    let total = summary.total_projects as f64;
    println!("✅ Fully Migrated: {} ({:.1}%)", format!("{:>3}", summary.fully_migrated).green(), (summary.fully_migrated as f64 / total * 100.0));
    println!("🔄 In Progress:    {} ({:.1}%)", format!("{:>3}", summary.in_progress).yellow(), (summary.in_progress as f64 / total * 100.0));
    println!("⏸️  Stalled:        {} ({:.1}%)", format!("{:>3}", summary.stalled).red(), (summary.stalled as f64 / total * 100.0));
    println!("📈 Regressed:      {} ({:.1}%)", format!("{:>3}", summary.regressed).purple(), (summary.regressed as f64 / total * 100.0));
    println!("🆕 New Projects:   {} ({:.1}%)", format!("{:>3}", summary.new_projects).cyan(), (summary.new_projects as f64 / total * 100.0));
}

/// Display detailed project table
pub fn display_projects_table(projects: &[ComparisonResult], limit: Option<usize>) {
    if projects.is_empty() {
        println!("\n{}", "No projects found.".yellow());
        return;
    }

    println!("\n{}", "📋 Project Details".bold());

    let mut builder = Builder::default();
    builder.push_record(["Project", "Baseline", "Current", "Removed", "Progress", "Status"]);

    let display_projects = if let Some(limit) = limit {
        &projects[..projects.len().min(limit)]
    } else {
        projects
    };

    // Find the longest project name for proper column sizing, but limit it for narrow terminals
    let max_project_name_len = display_projects
        .iter()
        .map(|p| p.project_name.len())
        .max()
        .unwrap_or(10)
        .max(7) // Minimum width for "Project" header
        .min(30); // Maximum width to prevent table overflow

    for project in display_projects {
        let status_display = format_migration_status_plain(&project.migration_status);
        let progress_display = if project.progress_percentage >= 0.0 {
            format!("{:>7.1}%", project.progress_percentage)
        } else {
            format!("{:>7.1}%", project.progress_percentage)
        };

        // Truncate project name if it's too long
        let truncated_name = if project.project_name.len() > max_project_name_len {
            format!("{}...", &project.project_name[..max_project_name_len-3])
        } else {
            project.project_name.clone()
        };

        builder.push_record([
            &format!("{:<width$}", truncated_name, width = max_project_name_len),
            &format!("{:>8}", project.baseline_usages),
            &format!("{:>7}", project.current_usages),
            &format!("{:>7}", format_usage_change_plain(project.usages_removed)),
            &progress_display,
            &status_display,
        ]);
    }

    let mut table = builder.build();
    table
        .with(Style::rounded())
        .with(Modify::new(Rows::first()).with(Color::BG_BLUE))
        // Set specific column alignments
        .with(Modify::new(Columns::single(0)).with(Alignment::left()))   // Project name
        .with(Modify::new(Columns::single(1)).with(Alignment::right()))  // Baseline
        .with(Modify::new(Columns::single(2)).with(Alignment::right()))  // Current
        .with(Modify::new(Columns::single(3)).with(Alignment::right()))  // Removed
        .with(Modify::new(Columns::single(4)).with(Alignment::right()))  // Progress
        .with(Modify::new(Columns::single(5)).with(Alignment::center())); // Status

    println!("{}", table);

    if let Some(limit) = limit {
        if projects.len() > limit {
            println!("\n{} projects shown. Use --all to see all {} projects.",
                     limit, projects.len());
        }
    }
}

/// Create a visual progress bar
fn create_progress_bar(percentage: f64) -> String {
    let width = 20;
    let filled = ((percentage / 100.0) * width as f64) as usize;
    let empty = width - filled;

    let bar = "█".repeat(filled) + &"░".repeat(empty);

    if percentage >= 75.0 {
        bar.green().to_string()
    } else if percentage >= 50.0 {
        bar.yellow().to_string()
    } else if percentage >= 25.0 {
        bar.red().to_string()
    } else {
        bar.bright_red().to_string()
    }
}

/// Format migration status with colors and icons
fn format_migration_status(status: &MigrationStatus) -> String {
    match status {
        MigrationStatus::FullyMigrated => "✅ Migrated".green().to_string(),
        MigrationStatus::InProgress => "🔄 Progress".yellow().to_string(),
        MigrationStatus::Stalled => "⏸️ Stalled".red().to_string(),
        MigrationStatus::Regressed => "📈 Regressed".purple().to_string(),
        MigrationStatus::New => "🆕 New".cyan().to_string(),
    }
}

/// Format migration status without colors for table alignment
fn format_migration_status_plain(status: &MigrationStatus) -> String {
    // Use fixed-width text representations for better table alignment
    match status {
        MigrationStatus::FullyMigrated => format!("{:<12}", "✅ Migrated"),
        MigrationStatus::InProgress => format!("{:<12}", "🔄 Progress"),
        MigrationStatus::Stalled => format!("{:<12}", "⏸️ Stalled"),
        MigrationStatus::Regressed => format!("{:<12}", "📈 Regressed"),
        MigrationStatus::New => format!("{:<12}", "🆕 New"),
    }
}

/// Format usage change with appropriate colors
fn format_usage_change(change: i32) -> String {
    if change > 0 {
        format!("+{}", change).green().to_string()
    } else if change < 0 {
        change.to_string().red().to_string()
    } else {
        "0".dimmed().to_string()
    }
}

/// Format usage change without colors for table alignment
fn format_usage_change_plain(change: i32) -> String {
    if change > 0 {
        format!("+{}", change)
    } else {
        change.to_string()
    }
}

/// Display top contributors
pub fn display_top_contributors(contributors: &[&ComparisonResult], title: &str) {
    if contributors.is_empty() {
        return;
    }

    println!("\n{}", title.bold());

    for (i, project) in contributors.iter().enumerate() {
        let rank = match i {
            0 => "🥇".to_string(),
            1 => "🥈".to_string(),
            2 => "🥉".to_string(),
            _ => format!("{}.", i + 1),
        };

        println!(
            "{} {} - {} usages removed ({:.1}% progress)",
            rank,
            project.project_name.bold(),
            project.usages_removed.to_string().green(),
            project.progress_percentage
        );
    }
}

/// Display projects needing attention
pub fn display_attention_needed(projects: &[&ComparisonResult]) {
    if projects.is_empty() {
        println!("\n{}", "🎉 No projects need immediate attention!".green().bold());
        return;
    }

    println!("\n{}", "⚠️  Projects Needing Attention".red().bold());

    for project in projects {
        let status_icon = match project.migration_status {
            MigrationStatus::Stalled => "⏸️",
            MigrationStatus::Regressed => "📈",
            _ => "⚠️",
        };

        println!(
            "{} {} - {} usages, {} status",
            status_icon,
            project.project_name.bold(),
            project.current_usages.to_string().red(),
            format_migration_status(&project.migration_status)
        );
    }
}

/// Display usage type breakdown
pub fn display_usage_breakdown(projects: &[ComparisonResult]) {
    use std::collections::HashMap;

    let mut type_counts = HashMap::new();
    let mut type_projects = HashMap::new();

    for project in projects {
        if project.current_usages > 0 {
            // This is a simplified version - in a real implementation,
            // you'd need to track usage types through the analysis
            type_counts.entry("General Usage".to_string()).or_insert(0usize);
            *type_counts.get_mut("General Usage").unwrap() += project.current_usages;

            type_projects.entry("General Usage".to_string()).or_insert(0usize);
            *type_projects.get_mut("General Usage").unwrap() += 1;
        }
    }

    if type_counts.is_empty() {
        return;
    }

    println!("\n{}", "🔍 Usage Type Breakdown".bold());

    let mut sorted_types: Vec<_> = type_counts.iter().collect();
    sorted_types.sort_by(|a, b| b.1.cmp(a.1));

    for (usage_type, count) in sorted_types {
        let project_count = type_projects.get(usage_type).unwrap_or(&0);
        println!(
            "• {}: {} usages across {} projects",
            usage_type.cyan(),
            count.to_string().yellow(),
            project_count.to_string().blue()
        );
    }
}

/// Display analysis insights and recommendations
pub fn display_insights(summary: &AnalysisSummary) {
    println!("\n{}", "💡 Insights & Recommendations".bold().blue());

    // Migration velocity
    let active_projects = summary.in_progress + summary.fully_migrated;
    if active_projects > 0 {
        println!(
            "• {}/{} projects ({:.1}%) have made progress",
            active_projects,
            summary.total_projects,
            (active_projects as f64 / summary.total_projects as f64) * 100.0
        );
    }

    // Identify patterns
    if summary.stalled > summary.total_projects / 4 {
        println!(
            "• {} Warning: High number of stalled projects - consider reviewing migration strategies",
            "⚠️".yellow()
        );
    }

    if summary.regressed > 0 {
        println!(
            "• {} Alert: {} projects have regressed - investigate recent changes",
            "🚨".red(),
            summary.regressed
        );
    }

    if summary.fully_migrated > summary.total_projects / 2 {
        println!(
            "• {} Great progress! Over half of projects are fully migrated",
            "🎉".green()
        );
    }
}

/// Format file size for display
pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1} {}", size, UNITS[unit_index])
}