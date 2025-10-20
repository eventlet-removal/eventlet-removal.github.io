pub mod parser;
pub mod analyzer;
pub mod cli;
pub mod export;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents different types of eventlet usage patterns
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UsageType {
    Import,           // import eventlet
    MonkeyPatch,      // eventlet.monkey_patch()
    Spawn,            // eventlet.spawn()
    Listen,           // eventlet.listen()
    Wsgi,             // eventlet.wsgi
    Executor,         // executor='eventlet'
    Sleep,            // eventlet.sleep()
    Pool,             // eventlet.GreenPool, eventlet.pool
    Other(String),    // Other usage patterns
}

impl UsageType {
    /// Categorize usage by complexity/migration difficulty
    pub fn migration_complexity(&self) -> u8 {
        match self {
            UsageType::Import => 1,
            UsageType::Executor => 2,
            UsageType::Sleep => 3,
            UsageType::Spawn => 4,
            UsageType::Pool => 5,
            UsageType::Listen => 6,
            UsageType::Wsgi => 7,
            UsageType::MonkeyPatch => 8,
            UsageType::Other(_) => 5, // Default medium complexity
        }
    }
}

/// Represents a single eventlet usage occurrence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventletUsage {
    pub file_url: String,
    pub line_number: u32,
    pub project_name: String,
    pub repository: String,
    pub usage_type: UsageType,
    pub code_snippet: String,
    pub file_path: String,
}

/// Represents the state of a project at a specific point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSnapshot {
    pub project_name: String,
    pub repository: String,
    pub total_usages: usize,
    pub usage_breakdown: HashMap<UsageType, usize>,
    pub complexity_score: f64,
    pub files_affected: usize,
}

/// Represents analysis results comparing two time periods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonResult {
    pub project_name: String,
    pub baseline_usages: usize,
    pub current_usages: usize,
    pub usages_removed: i32,
    pub progress_percentage: f64,
    pub migration_status: MigrationStatus,
    pub complexity_change: f64,
}

/// Migration status categories
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MigrationStatus {
    FullyMigrated,    // 0 usages remaining
    InProgress,       // Some reduction in usages
    Stalled,          // No change in usages
    Regressed,        // Increased usages
    New,              // New project with eventlet
}

/// Overall analysis summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisSummary {
    pub analysis_date: DateTime<Utc>,
    pub baseline_file: String,
    pub current_file: String,
    pub total_projects: usize,
    pub fully_migrated: usize,
    pub in_progress: usize,
    pub stalled: usize,
    pub regressed: usize,
    pub new_projects: usize,
    pub overall_progress: f64,
    pub total_usages_removed: i32,
    pub projects: Vec<ComparisonResult>,
}

/// Configuration for analysis parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    pub ignore_test_files: bool,
    pub min_usage_threshold: usize,
    pub complexity_weights: HashMap<UsageType, f64>,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        let mut complexity_weights = HashMap::new();
        complexity_weights.insert(UsageType::Import, 1.0);
        complexity_weights.insert(UsageType::Executor, 2.0);
        complexity_weights.insert(UsageType::Sleep, 3.0);
        complexity_weights.insert(UsageType::Spawn, 4.0);
        complexity_weights.insert(UsageType::Pool, 5.0);
        complexity_weights.insert(UsageType::Listen, 6.0);
        complexity_weights.insert(UsageType::Wsgi, 7.0);
        complexity_weights.insert(UsageType::MonkeyPatch, 8.0);

        Self {
            ignore_test_files: true,
            min_usage_threshold: 1,
            complexity_weights,
        }
    }
}