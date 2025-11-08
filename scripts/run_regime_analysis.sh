#!/bin/bash

# Regime Analysis Runner Script
# This script runs regime analysis with TTS scoring and saves results to Supabase

set -e  # Exit on any error

echo "🚀 Starting Regime Analysis Pipeline..."
echo "========================================"

# Check if environment variables are set
if [ -z "$SUPABASE_URL" ]; then
    echo "❌ Error: SUPABASE_URL environment variable not set"
    echo "   Please set your Supabase URL:"
    echo "   export SUPABASE_URL='https://your-project.supabase.co'"
    exit 1
fi

if [ -z "$SUPABASE_API_KEY" ]; then
    echo "❌ Error: SUPABASE_API_KEY environment variable not set"
    echo "   Please set your Supabase API key:"
    echo "   export SUPABASE_API_KEY='your-api-key'"
    exit 1
fi

echo "✅ Environment variables configured"
echo "   Supabase URL: $SUPABASE_URL"

# Get symbol from command line or use default
SYMBOL=${1:-AAPL}
echo "📊 Analyzing symbol: $SYMBOL"

# Check if ChatGPT API key is available (optional)
if [ -n "$OPENAI_API_KEY" ]; then
    echo "✅ ChatGPT API key found - AI analysis will be included"
else
    echo "⚠️  ChatGPT API key not found - analysis will run without AI insights"
    echo "   To enable AI analysis, set: export OPENAI_API_KEY='your-key'"
fi

echo ""
echo "🔧 Building and running regime analysis..."

# Build and run the regime analysis
cargo run --example regime_analysis_to_supabase -- $SYMBOL

echo ""
echo "🎉 Regime analysis completed!"
echo "📁 Check the generated JSON files for detailed results"
echo "💾 Data has been saved to your Supabase regime table"