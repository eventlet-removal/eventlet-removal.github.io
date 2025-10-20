# Eventlet Analyzer

A high-performance Rust toolkit for analyzing eventlet usage patterns in OpenStack projects. This tool processes Beagle search results to track migration progress, identify patterns, and generate comprehensive reports.

## Features

🚀 **High Performance**
- Parallel processing with Rayon for fast analysis
- Memory-mapped file reading for large datasets
- Optimized for processing thousands of usage records

📊 **Rich Visualization**
- Terminal-based charts and progress bars
- Colored output with migration status indicators
- Project comparison tables with detailed metrics

📈 **Comprehensive Analysis**
- Migration progress tracking over time
- Usage pattern categorization (imports, monkey_patch, spawn, etc.)
- Project complexity scoring
- Regression detection
- Smart filtering to exclude documentation and release note false positives

📤 **Multiple Export Formats**
- JSON (compact and pretty-printed)
- CSV for spreadsheet analysis
- YAML for configuration management
- Hugo/Jekyll markdown for static sites

## Installation

### From Source

```bash
git clone <repository-url>
cd eventlet-analyzer
cargo build --release
```

The binary will be available at `target/release/eventlet-analyzer`.

### Prerequisites

- Rust 1.70 or later
- Cargo package manager

## Quick Start

### Compare Two Time Periods

```bash
# Compare baseline vs current state
eventlet-analyzer compare \
  --baseline stats/origin.txt \
  --current stats/october-2025.txt \
  --insights

# Limit output to top 10 projects
eventlet-analyzer compare \
  --baseline stats/origin.txt \
  --current stats/october-2025.txt \
  --limit 10
```

### Analyze Single File

```bash
# Analyze current eventlet usage
eventlet-analyzer analyze \
  --input stats/october-2025.txt \
  --limit 20
```

### Generate Export Package

```bash
# Create comprehensive analysis package
eventlet-analyzer package \
  --baseline stats/origin.txt \
  --current stats/october-2025.txt \
  --output analysis-results/
```

## Usage

### Commands

#### `compare` - Compare Two Analysis Periods

Compare migration progress between two Beagle result files.

```bash
eventlet-analyzer compare [OPTIONS] --baseline <FILE> --current <FILE>
```

**Options:**
- `--baseline <FILE>` - Baseline Beagle results file
- `--current <FILE>` - Current Beagle results file
- `--format <FORMAT>` - Output format: table (default), json, yaml
- `--limit <N>` - Show only top N projects
- `--all` - Show all projects (overrides limit)
- `--insights` - Display detailed insights and recommendations
- `--exclude-tests` - Exclude test files (default: true)
- `--include-docs` - Include documentation and release notes (excluded by default)

**Example:**
```bash
eventlet-analyzer compare \
  --baseline origin.txt \
  --current october-2025.txt \
  --format table \
  --limit 15 \
  --insights
```

#### `analyze` - Single File Analysis

Analyze eventlet usage in a single Beagle results file.

```bash
eventlet-analyzer analyze [OPTIONS] --input <FILE>
```

**Options:**
- `--input <FILE>` - Input Beagle results file
- `--format <FORMAT>` - Output format: table (default), json, yaml
- `--limit <N>` - Show only top N projects
- `--all` - Show all projects
- `--exclude-tests` - Exclude test files (default: true)
- `--include-docs` - Include documentation and release notes (excluded by default)

#### `export` - Export Analysis Results

Export analysis results in various machine-readable formats.

```bash
eventlet-analyzer export [OPTIONS] --output <FILE>
```

**Options:**
- `--input <FILE>` - Single input file (for single analysis)
- `--baseline <FILE>` - Baseline file (for comparison)
- `--current <FILE>` - Current file (for comparison)
- `--format <FORMAT>` - Export format: json, json-pretty, csv, yaml, hugo, jekyll
- `--output <FILE>` - Output file path

**Export Formats:**
- `json` - Compact JSON
- `json-pretty` - Pretty-printed JSON
- `csv` - CSV for spreadsheets
- `yaml` - YAML format
- `hugo` - Hugo static site generator format
- `jekyll` - Jekyll static site generator format

#### `package` - Generate Comprehensive Package

Create a complete analysis package with multiple formats.

```bash
eventlet-analyzer package [OPTIONS] --baseline <FILE> --current <FILE>
```

**Options:**
- `--baseline <FILE>` - Baseline Beagle results file
- `--current <FILE>` - Current Beagle results file
- `--output <DIR>` - Output directory (default: eventlet-analysis)

**Package Contents:**
- Summary in JSON, YAML, and Markdown formats
- Detailed project comparison CSV
- Raw usage data CSV files
- Index file with descriptions

#### `stats` - Usage Statistics

Display statistics about eventlet usage patterns.

```bash
eventlet-analyzer stats [OPTIONS] --input <FILE>...
```

**Options:**
- `--input <FILE>...` - Input files (can specify multiple)
- `--usage-types` - Show usage type breakdown
- `--files` - Show file statistics
- `--min-usages <N>` - Minimum usage count threshold

### Input Format

The tool expects Beagle search results in this format:

```
URL#nLINE : CODE_SNIPPET
```

**Examples:**
```
https://opendev.org/openstack/nova/src/branch/master/nova/service.py#n42 : import eventlet
https://opendev.org/openstack/neutron/src/branch/master/neutron/wsgi.py#n15 : eventlet.monkey_patch()
```

Supports both OpenDev.org and GitHub URL formats.

## Output Examples

### Terminal Output

```
📊 OpenStack Eventlet Migration Analysis
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📈 Overall Progress
Analysis Date: 2025-10-20 09:09 UTC
Baseline File: stats/origin.txt
Current File:  stats/october-2025.txt

Overall Progress: ██░░░░░░░░░░░░░░░░░░ 10.2% (239/2344 usages removed)

📋 Project Status Breakdown
✅ Fully Migrated:  14 (11.5%)
🔄 In Progress:     20 (16.4%)
⏸️  Stalled:         60 (49.2%)
📈 Regressed:       24 (19.7%)
🆕 New Projects:     4 (3.3%)
```

### JSON Export

```json
{
  "analysis_date": "2025-10-20T09:09:14Z",
  "baseline_file": "origin.txt",
  "current_file": "october-2025.txt",
  "summary": {
    "total_projects": 122,
    "fully_migrated": 14,
    "in_progress": 20,
    "overall_progress": 10.2
  },
  "projects": [...]
}
```

## Usage Type Categories

The analyzer recognizes these eventlet usage patterns:

- **Import** - `import eventlet`, `from eventlet`
- **MonkeyPatch** - `eventlet.monkey_patch()`
- **Spawn** - `eventlet.spawn()`
- **Listen** - `eventlet.listen()`
- **WSGI** - `eventlet.wsgi`
- **Executor** - `executor='eventlet'`
- **Sleep** - `eventlet.sleep()`
- **Pool** - `eventlet.GreenPool`, `eventlet.pool`
- **Other** - Other usage patterns

Each pattern has an associated complexity score for migration difficulty assessment.

## Smart Filtering

By default, the analyzer excludes documentation and release note mentions to avoid false positives:

### Automatically Filtered Out:
- **Documentation**: `/doc/`, `/docs/`, `.md`, `.rst`, `README`, etc.
- **Release Notes**: `/releasenotes/`, `/release-notes/`, `/notes/`
- **Translations**: `/locale/`, `/po/`, `.po`, `.pot`
- **Test Files**: `/test/`, `/tests/`, `*_test.py`, `test_*.py`

### Real-World Impact:
```bash
# With docs included
eventlet-analyzer compare --baseline origin.txt --current current.txt --include-docs
# Result: 2,344 → 2,105 usages (10.2% progress)

# With docs excluded (default)
eventlet-analyzer compare --baseline origin.txt --current current.txt
# Result: 1,782 → 1,497 usages (16.0% progress)
```

The filtering removes **~600 false positive matches** from documentation, giving you a much more accurate view of actual code migration progress.

### Override Filtering:
Use `--include-docs` to include all matches when you need the complete picture.

## Migration Status Categories

- **✅ Fully Migrated** - No eventlet usage remaining
- **🔄 In Progress** - Reduced usage count
- **⏸️ Stalled** - No change in usage count
- **📈 Regressed** - Increased usage count
- **🆕 New** - New project with eventlet usage

## Performance

Typical performance on a modern system:
- **2,000+ usage records**: ~0.5 seconds
- **100+ projects**: Parallel analysis
- **Memory usage**: Optimized for large datasets
- **Export**: Multiple formats generated in parallel

## Integration

### Static Site Generation

#### Hugo Integration

Export with Hugo format for static site integration:

```bash
eventlet-analyzer export \
  --baseline origin.txt \
  --current current.txt \
  --format hugo \
  --output content/analysis/migration-progress.md
```

#### Jekyll Integration

```bash
eventlet-analyzer export \
  --baseline origin.txt \
  --current current.txt \
  --format jekyll \
  --output _posts/2025-10-20-migration-progress.md
```

### Continuous Integration

Use in CI/CD pipelines for automated reporting:

```bash
#!/bin/bash
# Generate monthly report
eventlet-analyzer package \
  --baseline stats/baseline.txt \
  --current stats/current.txt \
  --output reports/$(date +%Y-%m)

# Upload to web dashboard
cp reports/$(date +%Y-%m)/*.json web-dashboard/data/
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make changes with tests
4. Submit a pull request

### Development

```bash
# Run tests
cargo test

# Run with debug output
cargo run -- compare --baseline test1.txt --current test2.txt

# Build optimized release
cargo build --release
```

## License

MIT License - see LICENSE file for details.

## Architecture

- **Parser Module** - Parallel Beagle result parsing
- **Analyzer Module** - Migration progress analysis
- **CLI Module** - Terminal visualization
- **Export Module** - Multi-format data export

Built with Rust for performance and reliability.