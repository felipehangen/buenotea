#!/bin/bash

# Script to check the status of fundamentals analysis
# This shows which stocks have been analyzed and their results

echo "📊 Fundamentals Analysis Status Check"
echo "====================================="

# Check if .env file exists
if [ ! -f ".env" ]; then
    echo "❌ Error: .env file not found!"
    echo "Please create a .env file with your Supabase credentials:"
    echo "SUPABASE_URL=your_supabase_url"
    echo "SUPABASE_API_KEY=your_supabase_api_key"
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

echo ""
echo "🔍 Checking fundamentals analysis status..."

# Run the status check
cargo run --example fundamentals_status_check

# Check exit status
if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Status check completed successfully!"
else
    echo ""
    echo "❌ Status check failed!"
    echo "Check the error messages above for details."
    exit 1
fi


