use crate::{
    AnalysisConfig, AnalysisSummary, ComparisonResult, EventletUsage, MigrationStatus,
    ProjectSnapshot, UsageType,
};
use chrono::Utc;
use rayon::prelude::*;
use std::collections::HashMap;

/// Analyze eventlet usages and generate project snapshots
pub fn analyze_usages(
    usages: &[EventletUsage],
    config: &AnalysisConfig,
) -> HashMap<String, ProjectSnapshot> {
    usages
        .par_iter()
        .fold(
            HashMap::new,
            |mut acc: HashMap<String, Vec<&EventletUsage>>, usage| {
                acc.entry(usage.project_name.clone())
                    .or_default()
                    .push(usage);
                acc
            },
        )
        .reduce(
            HashMap::new,
            |mut acc, map| {
                for (project, usages) in map {
                    acc.entry(project)
                        .or_default()
                        .extend(usages);
                }
                acc
            },
        )
        .into_par_iter()
        .map(|(project_name, project_usages)| {
            let snapshot = create_project_snapshot(&project_name, &project_usages, config);
            (project_name, snapshot)
        })
        .collect()
}

/// Create a snapshot of a project's eventlet usage
fn create_project_snapshot(
    project_name: &str,
    usages: &[&EventletUsage],
    config: &AnalysisConfig,
) -> ProjectSnapshot {
    let total_usages = usages.len();

    // Count usage types
    let mut usage_breakdown = HashMap::new();
    for usage in usages {
        *usage_breakdown.entry(usage.usage_type.clone()).or_insert(0) += 1;
    }

    // Calculate complexity score
    let complexity_score = calculate_complexity_score(&usage_breakdown, config);

    // Count unique files affected
    let files_affected = usages
        .iter()
        .map(|u| &u.file_path)
        .collect::<std::collections::HashSet<_>>()
        .len();

    let repository = usages
        .first()
        .map(|u| u.repository.clone())
        .unwrap_or_default();

    ProjectSnapshot {
        project_name: project_name.to_string(),
        repository,
        total_usages,
        usage_breakdown,
        complexity_score,
        files_affected,
    }
}

/// Calculate complexity score based on usage types and weights
fn calculate_complexity_score(
    usage_breakdown: &HashMap<UsageType, usize>,
    config: &AnalysisConfig,
) -> f64 {
    usage_breakdown
        .iter()
        .map(|(usage_type, count)| {
            let weight = config
                .complexity_weights
                .get(usage_type)
                .copied()
                .unwrap_or(usage_type.migration_complexity() as f64);
            *count as f64 * weight
        })
        .sum()
}

/// Compare two sets of project snapshots to generate migration analysis
pub fn compare_snapshots(
    baseline: &HashMap<String, ProjectSnapshot>,
    current: &HashMap<String, ProjectSnapshot>,
) -> Vec<ComparisonResult> {
    // Get all project names from both datasets
    let all_projects: std::collections::HashSet<String> = baseline
        .keys()
        .chain(current.keys())
        .cloned()
        .collect();

    all_projects
        .into_par_iter()
        .map(|project_name| {
            let baseline_snapshot = baseline.get(&project_name);
            let current_snapshot = current.get(&project_name);

            match (baseline_snapshot, current_snapshot) {
                (Some(baseline), Some(current)) => {
                    // Project exists in both datasets
                    let baseline_usages = baseline.total_usages as i32;
                    let current_usages = current.total_usages as i32;
                    let usages_removed = baseline_usages - current_usages;

                    let progress_percentage = if baseline_usages > 0 {
                        (usages_removed as f64 / baseline_usages as f64) * 100.0
                    } else {
                        0.0
                    };

                    let migration_status = determine_migration_status(
                        baseline_usages,
                        current_usages,
                        usages_removed,
                        &baseline.usage_breakdown,
                        &current.usage_breakdown,
                    );

                    let complexity_change = current.complexity_score - baseline.complexity_score;

                    ComparisonResult {
                        project_name,
                        baseline_usages: baseline_usages as usize,
                        current_usages: current_usages as usize,
                        usages_removed,
                        progress_percentage,
                        migration_status,
                        complexity_change,
                    }
                }
                (Some(baseline), None) => {
                    // Project was fully migrated
                    ComparisonResult {
                        project_name,
                        baseline_usages: baseline.total_usages,
                        current_usages: 0,
                        usages_removed: baseline.total_usages as i32,
                        progress_percentage: 100.0,
                        migration_status: MigrationStatus::FullyMigrated,
                        complexity_change: -baseline.complexity_score,
                    }
                }
                (None, Some(current)) => {
                    // New project with eventlet usage
                    ComparisonResult {
                        project_name,
                        baseline_usages: 0,
                        current_usages: current.total_usages,
                        usages_removed: -(current.total_usages as i32),
                        progress_percentage: -100.0,
                        migration_status: MigrationStatus::New,
                        complexity_change: current.complexity_score,
                    }
                }
                (None, None) => unreachable!(),
            }
        })
        .collect()
}

/// Determine migration status based on usage changes and usage patterns
fn determine_migration_status(
    _baseline_usages: i32,
    current_usages: i32,
    usages_removed: i32,
    baseline_breakdown: &HashMap<UsageType, usize>,
    current_breakdown: &HashMap<UsageType, usize>,
) -> MigrationStatus {
    if current_usages == 0 {
        MigrationStatus::FullyMigrated
    } else if usages_removed > 0 {
        MigrationStatus::InProgress
    } else if usages_removed == 0 {
        MigrationStatus::Stalled
    } else {
        // usages_removed < 0 (increased usages)
        // Check if the increase is primarily due to deprecation warnings
        let baseline_deprecations = baseline_breakdown.get(&UsageType::Deprecation).unwrap_or(&0);
        let current_deprecations = current_breakdown.get(&UsageType::Deprecation).unwrap_or(&0);

        if current_deprecations > baseline_deprecations {
            let deprecation_increase = current_deprecations - baseline_deprecations;
            let total_increase = (-usages_removed) as usize; // Convert negative to positive

            // If most of the increase (>= 60%) is due to deprecations, classify as Deprecating
            if deprecation_increase as f64 / total_increase as f64 >= 0.6 {
                return MigrationStatus::Deprecating;
            }
        }

        MigrationStatus::Regressed
    }
}

/// Generate comprehensive analysis summary
pub fn generate_analysis_summary(
    baseline_file: String,
    current_file: String,
    comparisons: Vec<ComparisonResult>,
) -> AnalysisSummary {
    let total_projects = comparisons.len();

    let fully_migrated = comparisons
        .iter()
        .filter(|c| c.migration_status == MigrationStatus::FullyMigrated)
        .count();

    let in_progress = comparisons
        .iter()
        .filter(|c| c.migration_status == MigrationStatus::InProgress)
        .count();

    let stalled = comparisons
        .iter()
        .filter(|c| c.migration_status == MigrationStatus::Stalled)
        .count();

    let deprecating = comparisons
        .iter()
        .filter(|c| c.migration_status == MigrationStatus::Deprecating)
        .count();

    let regressed = comparisons
        .iter()
        .filter(|c| c.migration_status == MigrationStatus::Regressed)
        .count();

    let new_projects = comparisons
        .iter()
        .filter(|c| c.migration_status == MigrationStatus::New)
        .count();

    let total_usages_removed: i32 = comparisons
        .iter()
        .map(|c| c.usages_removed)
        .sum();

    let total_baseline_usages: usize = comparisons
        .iter()
        .map(|c| c.baseline_usages)
        .sum();

    let overall_progress = if total_baseline_usages > 0 {
        (total_usages_removed as f64 / total_baseline_usages as f64) * 100.0
    } else {
        0.0
    }.max(0.0); // Ensure non-negative

    AnalysisSummary {
        analysis_date: Utc::now(),
        baseline_file,
        current_file,
        total_projects,
        fully_migrated,
        in_progress,
        deprecating,
        stalled,
        regressed,
        new_projects,
        overall_progress,
        total_usages_removed,
        projects: comparisons,
    }
}

/// Find top contributors (projects with most usages removed)
pub fn find_top_contributors(
    comparisons: &[ComparisonResult],
    limit: usize,
) -> Vec<&ComparisonResult> {
    let mut sorted = comparisons.iter().collect::<Vec<_>>();
    sorted.sort_by(|a, b| b.usages_removed.cmp(&a.usages_removed));
    sorted.into_iter().take(limit).collect()
}

/// Find projects that need attention (high usage, no progress)
pub fn find_attention_needed(
    comparisons: &[ComparisonResult],
    min_usages: usize,
) -> Vec<&ComparisonResult> {
    comparisons
        .iter()
        .filter(|c| {
            c.current_usages >= min_usages &&
            (c.migration_status == MigrationStatus::Stalled ||
             c.migration_status == MigrationStatus::Regressed)
        })
        .collect()
}

/// Find low hanging fruits - projects that are easy to migrate
pub fn find_low_hanging_fruits(
    comparisons: &[ComparisonResult],
    max_usages: usize,
    max_complexity: f64,
) -> Vec<&ComparisonResult> {
    comparisons
        .iter()
        .filter(|c| {
            // Only include projects that still have eventlet usage
            c.current_usages > 0 &&
            c.current_usages <= max_usages &&
            c.complexity_change.abs() <= max_complexity &&
            // Exclude already migrated projects
            c.migration_status != MigrationStatus::FullyMigrated
        })
        .collect()
}

/// Find low hanging fruits from project snapshots (for single file analysis)
pub fn find_low_hanging_fruits_from_snapshots(
    snapshots: &std::collections::HashMap<String, crate::ProjectSnapshot>,
    max_usages: usize,
    max_complexity: f64,
) -> Vec<&crate::ProjectSnapshot> {
    snapshots
        .values()
        .filter(|snapshot| {
            snapshot.total_usages > 0 &&
            snapshot.total_usages <= max_usages &&
            snapshot.complexity_score <= max_complexity
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_migration_status() {
        let empty_breakdown = HashMap::new();
        let mut baseline_breakdown = HashMap::new();
        let mut current_breakdown = HashMap::new();

        // Test basic cases with empty breakdowns
        assert_eq!(
            determine_migration_status(10, 0, 10, &empty_breakdown, &empty_breakdown),
            MigrationStatus::FullyMigrated
        );
        assert_eq!(
            determine_migration_status(10, 5, 5, &empty_breakdown, &empty_breakdown),
            MigrationStatus::InProgress
        );
        assert_eq!(
            determine_migration_status(10, 10, 0, &empty_breakdown, &empty_breakdown),
            MigrationStatus::Stalled
        );
        assert_eq!(
            determine_migration_status(10, 15, -5, &empty_breakdown, &empty_breakdown),
            MigrationStatus::Regressed
        );

        // Test deprecating case
        baseline_breakdown.insert(UsageType::Deprecation, 0);
        current_breakdown.insert(UsageType::Deprecation, 4);
        current_breakdown.insert(UsageType::Import, 1);

        assert_eq!(
            determine_migration_status(10, 15, -5, &baseline_breakdown, &current_breakdown),
            MigrationStatus::Deprecating
        );
    }
}