#!/bin/bash

# Script to run invite list analysis with S&P 500 stocks
# This script analyzes S&P 500 stocks for trading safety and saves to database

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
echo "📋 Current Configuration:"
echo "   FMP_API_KEY: ${FMP_API_KEY:0:10}..."
echo "   SUPABASE_URL: $SUPABASE_URL"
echo "   SUPABASE_API_KEY: ${SUPABASE_API_KEY:0:20}..."
echo ""

echo "🚀 Starting S&P 500 safety analysis..."
echo ""

# Run the simple analysis first (without database)
echo "🔍 Running simple analysis (test mode)..."
cargo run --example invite_list_simple

echo ""
echo "💾 Running full analysis with database storage..."
cargo run --example invite_list_to_supabase

echo ""
echo "🎉 S&P 500 safety analysis complete!"
echo ""
echo "📊 Check your Supabase database 'invite_list' table to see the results"
echo "📈 Use the 'safe_stocks' view to see only stocks that are safe to trade"
echo "📊 Use the 'sector_safety_analysis' view for sector breakdown"
echo "⚠️  Use the 'risk_distribution' view for risk level analysis"
