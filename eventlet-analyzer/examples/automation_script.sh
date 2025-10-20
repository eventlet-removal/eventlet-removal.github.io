#!/bin/bash
# Automation script for regular eventlet migration monitoring
# This script demonstrates how to integrate eventlet-analyzer into
# a continuous monitoring workflow

set -e

ANALYZER="./target/release/eventlet-analyzer"
DATE=$(date +%Y%m%d)
REPORT_DIR="reports/$DATE"
WEB_DIR="web-dashboard/data"

# Configuration
BASELINE_FILE="../stats/origin.txt"
CURRENT_FILE="../stats/october-2025.txt"  # This would typically be generated dynamically

echo "🤖 Automated Eventlet Migration Analysis - $DATE"
echo "=================================================="

# Create directories
mkdir -p "$REPORT_DIR"
mkdir -p "$WEB_DIR"

# 1. Generate comprehensive analysis package
echo "📦 Generating comprehensive analysis package..."
$ANALYZER package \
  --baseline "$BASELINE_FILE" \
  --current "$CURRENT_FILE" \
  --output "$REPORT_DIR"

# 2. Generate web-friendly exports
echo "🌐 Creating web dashboard data..."

# JSON for web APIs
$ANALYZER export \
  --baseline "$BASELINE_FILE" \
  --current "$CURRENT_FILE" \
  --format json \
  --output "$WEB_DIR/latest-analysis.json"

# Pretty JSON for human reading
$ANALYZER export \
  --baseline "$BASELINE_FILE" \
  --current "$CURRENT_FILE" \
  --format json-pretty \
  --output "$REPORT_DIR/analysis-readable.json"

# CSV for data analysis
$ANALYZER export \
  --baseline "$BASELINE_FILE" \
  --current "$CURRENT_FILE" \
  --format csv \
  --output "$WEB_DIR/project-data.csv"

# 3. Generate static site content
echo "📄 Creating static site content..."

# Hugo format
$ANALYZER export \
  --baseline "$BASELINE_FILE" \
  --current "$CURRENT_FILE" \
  --format hugo \
  --output "$REPORT_DIR/migration-progress-hugo.md"

# Jekyll format
$ANALYZER export \
  --baseline "$BASELINE_FILE" \
  --current "$CURRENT_FILE" \
  --format jekyll \
  --output "$REPORT_DIR/migration-progress-jekyll.md"

# 4. Generate summary report for CLI
echo "📊 Creating terminal summary..."
$ANALYZER compare \
  --baseline "$BASELINE_FILE" \
  --current "$CURRENT_FILE" \
  --insights \
  --format table > "$REPORT_DIR/terminal-summary.txt"

# 5. Extract key metrics for alerting
echo "🚨 Extracting key metrics..."

# Parse JSON to extract key metrics (requires jq)
if command -v jq &> /dev/null; then
    OVERALL_PROGRESS=$(jq -r '.overall_progress' "$WEB_DIR/latest-analysis.json")
    STALLED_COUNT=$(jq -r '.stalled' "$WEB_DIR/latest-analysis.json")
    REGRESSED_COUNT=$(jq -r '.regressed' "$WEB_DIR/latest-analysis.json")

    echo "Overall Progress: $OVERALL_PROGRESS%"
    echo "Stalled Projects: $STALLED_COUNT"
    echo "Regressed Projects: $REGRESSED_COUNT"

    # Create metrics file for monitoring
    cat > "$REPORT_DIR/metrics.txt" << EOF
date=$DATE
overall_progress=$OVERALL_PROGRESS
stalled_projects=$STALLED_COUNT
regressed_projects=$REGRESSED_COUNT
EOF

    # Alert conditions
    if (( $(echo "$OVERALL_PROGRESS < 15" | bc -l) )); then
        echo "⚠️  WARNING: Overall progress is below 15%"
    fi

    if (( REGRESSED_COUNT > 20 )); then
        echo "🚨 ALERT: More than 20 projects have regressed"
    fi
else
    echo "📝 jq not found - skipping metric extraction"
fi

# 6. Create index file
echo "📋 Creating report index..."
cat > "$REPORT_DIR/README.md" << EOF
# Eventlet Migration Report - $DATE

## Quick Stats
- Generated: $(date)
- Baseline: $BASELINE_FILE
- Current: $CURRENT_FILE

## Files in this report:

### Analysis Data
- \`summary_*.json\` - Complete analysis summary
- \`projects_*.csv\` - Project-by-project data
- \`usages_*.csv\` - Raw usage data

### Web Dashboard
- \`../web-dashboard/latest-analysis.json\` - API data
- \`../web-dashboard/project-data.csv\` - Spreadsheet data

### Static Sites
- \`migration-progress-hugo.md\` - Hugo format
- \`migration-progress-jekyll.md\` - Jekyll format

### Reports
- \`terminal-summary.txt\` - CLI output
- \`metrics.txt\` - Key metrics (if jq available)

## Usage

View the terminal summary:
\`\`\`bash
cat terminal-summary.txt
\`\`\`

Open JSON data:
\`\`\`bash
jq '.' analysis-readable.json
\`\`\`

Import CSV data into spreadsheet applications.
EOF

echo "✅ Analysis complete!"
echo "📁 Reports saved to: $REPORT_DIR"
echo "🌐 Web data saved to: $WEB_DIR"

# List generated files
echo -e "\n📄 Generated files:"
find "$REPORT_DIR" -type f -exec basename {} \; | sort
echo -e "\n💾 Total report size: $(du -sh "$REPORT_DIR" | cut -f1)"

# Optional: Commit to git or upload to cloud storage
# git add "$REPORT_DIR" && git commit -m "Automated eventlet analysis $DATE"
# aws s3 sync "$REPORT_DIR" s3://your-bucket/reports/$DATE/