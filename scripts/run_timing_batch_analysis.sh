#!/bin/bash

# Batch Timing Analysis Script
# Runs timing analysis on all stocks in the invite_list table and saves results to timing table

echo "Starting batch timing analysis for all stocks in invite_list table..."
echo "This may take several minutes depending on the number of stocks."
echo ""

# Check if .env file exists
if [ -f .env ]; then
    echo "✅ Found .env file - environment variables will be loaded automatically"
else
    echo "⚠️  Warning: .env file not found."
    echo "   Make sure your environment variables are set:"
    echo "   - SUPABASE_URL"
    echo "   - SUPABASE_API_KEY (or SUPABASE_ANON_KEY)"
    echo "   - FMP_API_KEY"
    echo "   - ALPHA_VANTAGE_API_KEY"
fi

echo ""

# Run the batch timing analysis
echo "Running batch timing analysis..."
echo "This will analyze all stocks in the invite_list table and save timing data to the timing table."
echo ""

cargo run --example timing_analysis_batch --release

echo ""
echo "Batch timing analysis completed!"
echo "Check the logs above for detailed results and any errors."
