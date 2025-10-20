#!/bin/bash
# Basic usage examples for eventlet-analyzer

# Build the project first
echo "Building eventlet-analyzer..."
cargo build --release

ANALYZER="./target/release/eventlet-analyzer"

echo "=== Basic Usage Examples ==="

# 1. Simple comparison
echo "1. Comparing baseline vs current state:"
$ANALYZER compare \
  --baseline ../stats/origin.txt \
  --current ../stats/october-2025.txt \
  --limit 10

echo -e "\n" && read -p "Press Enter to continue..."

# 2. Full comparison with insights
echo "2. Full comparison with insights:"
$ANALYZER compare \
  --baseline ../stats/origin.txt \
  --current ../stats/october-2025.txt \
  --insights \
  --all

echo -e "\n" && read -p "Press Enter to continue..."

# 3. Single file analysis
echo "3. Analyzing current state:"
$ANALYZER analyze \
  --input ../stats/october-2025.txt \
  --limit 15

echo -e "\n" && read -p "Press Enter to continue..."

# 4. Generate comprehensive package
echo "4. Generating analysis package:"
$ANALYZER package \
  --baseline ../stats/origin.txt \
  --current ../stats/october-2025.txt \
  --output example-analysis

echo "Package created in example-analysis/"
ls -la example-analysis/

echo -e "\n" && read -p "Press Enter to continue..."

# 5. Export to different formats
echo "5. Exporting to different formats:"

# JSON export
$ANALYZER export \
  --baseline ../stats/origin.txt \
  --current ../stats/october-2025.txt \
  --format json-pretty \
  --output migration-summary.json

# CSV export for spreadsheets
$ANALYZER export \
  --input ../stats/october-2025.txt \
  --format csv \
  --output current-usage.csv

# Hugo format for static sites
$ANALYZER export \
  --baseline ../stats/origin.txt \
  --current ../stats/october-2025.txt \
  --format hugo \
  --output migration-report.md

echo "Exported files:"
ls -la *.json *.csv *.md

echo -e "\n=== Examples completed! ==="