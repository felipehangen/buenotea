#!/bin/bash

# Script to run batch fundamentals analysis for all stocks in invite_list table
# This will analyze all stocks and save the results to the fundamentals table

echo "🚀 Starting Batch Fundamentals Analysis"
echo "========================================"

# Check if .env file exists
if [ ! -f ".env" ]; then
    echo "❌ Error: .env file not found!"
    echo "Please create a .env file with your Supabase credentials:"
    echo "SUPABASE_URL=your_supabase_url"
    echo "SUPABASE_API_KEY=your_supabase_api_key"
    echo "FMP_API_KEY=your_fmp_api_key (optional)"
    echo "ALPHA_VANTAGE_API_KEY=your_alpha_vantage_api_key (optional)"
    echo "FINNHUB_API_KEY=your_finnhub_api_key (optional)"
    exit 1
fi

# Load environment variables
source .env

# Check required environment variables
if [ -z "$SUPABASE_URL" ] || [ -z "$SUPABASE_API_KEY" ]; then
    echo "❌ Error: Missing required environment variables!"
    echo "Please ensure SUPABASE_URL and SUPABASE_API_KEY are set in your .env file"
    exit 1
fi

echo "✅ Environment variables loaded"
echo "📊 Supabase URL: $SUPABASE_URL"
echo "🔑 API Key: ${SUPABASE_API_KEY:0:8}..."

# Check if invite_list has data
echo ""
echo "🔍 Checking invite_list table for stocks to analyze..."

# Run the batch analysis
echo ""
echo "🚀 Running batch fundamentals analysis..."
echo "This may take several minutes depending on the number of stocks."
echo ""

# Run the Rust example
cargo run --example fundamentals_batch_analysis -p buenotea-fundamentals_batch_analysis

# Check exit status
if [ $? -eq 0 ]; then
    echo ""
    echo "🎉 Batch fundamentals analysis completed successfully!"
    echo "📊 Check the fundamentals table in your Supabase dashboard to see the results."
    echo ""
    echo "💡 Next steps:"
    echo "  - View results in Supabase fundamentals table"
    echo "  - Run timing analysis if needed"
    echo "  - Set up automated cron job for regular updates"
else
    echo ""
    echo "❌ Batch fundamentals analysis failed!"
    echo "Check the error messages above for details."
    exit 1
fi


