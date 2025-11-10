#!/bin/bash

echo "🎯 S&P 500 Safety Analysis System"
echo "================================="
echo ""

# Load environment variables from .env file
if [[ -f ".env" ]]; then
    echo "📁 Loading environment variables from .env file..."
    source .env
else
    echo "❌ .env file not found!"
    echo ""
    echo "Please ensure the .env file exists with the required API keys:"
    echo "  FMP_API_KEY=your_fmp_api_key"
    echo "  SUPABASE_URL=your_supabase_url"
    echo "  SUPABASE_API_KEY=your_supabase_api_key"
    echo ""
    exit 1
fi

echo "✅ Environment variables configured!"
echo ""

echo "🚀 Starting S&P 500 analysis and saving to Supabase..."
echo "⏳ This will analyze all 500+ stocks and may take 10-15 minutes..."
echo ""

# Run the analysis
cargo run --example invite_list_to_supabase

echo ""
echo "🎉 Analysis complete!"
echo "💡 Check your Supabase 'invite_list' table to see all results"

