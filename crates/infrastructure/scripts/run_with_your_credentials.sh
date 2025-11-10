#!/bin/bash

# Script to run regime analysis with your actual Supabase credentials
# Replace the placeholder values with your actual Supabase URL and Anon Key

echo "🎯 Regime Analysis System - Ready to Save to Database"
echo "====================================================="
echo ""

# Replace these with your actual Supabase credentials
SUPABASE_URL="https://your-project.supabase.co"
SUPABASE_ANON_KEY="your_anon_key_here"

echo "📋 Current Configuration:"
echo "   SUPABASE_URL: $SUPABASE_URL"
echo "   SUPABASE_ANON_KEY: ${SUPABASE_ANON_KEY:0:20}..."
echo ""

echo "⚠️  IMPORTANT: Please edit this script and replace the placeholder values with your actual Supabase credentials!"
echo ""

# Check if still using placeholder values
if [[ "$SUPABASE_URL" == "https://your-project.supabase.co" ]] || [[ "$SUPABASE_ANON_KEY" == "your_anon_key_here" ]]; then
    echo "❌ Please update the credentials in this script first!"
    echo ""
    echo "Edit the file: run_with_your_credentials.sh"
    echo "Replace:"
    echo "  SUPABASE_URL=\"https://your-project.supabase.co\""
    echo "  SUPABASE_ANON_KEY=\"your_anon_key_here\""
    echo ""
    echo "With your actual Supabase credentials, then run this script again."
    exit 1
fi

echo "✅ Credentials configured!"
echo ""

echo "🚀 Running regime analysis and saving to database..."
echo ""

# Export environment variables and run the analysis
export SUPABASE_URL
export SUPABASE_ANON_KEY

cargo run --example regime_save_to_db

echo ""
echo "🎉 Regime analysis complete!"
echo ""
echo "📊 Check your Supabase database 'regime' table to see the saved data"
echo "📁 JSON files are also saved locally for reference"

