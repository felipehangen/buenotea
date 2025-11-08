#!/bin/bash

# Batch Sentiment Analysis Script
# This script runs sentiment analysis on all stocks in the invite_list table

echo "=== Batch Sentiment Analysis for All Invite List Stocks ==="
echo "This will analyze sentiment for all stocks in the invite_list table"
echo ""

# Check if .env file exists
if [ ! -f .env ]; then
    echo "❌ Error: .env file not found!"
    echo "Please create a .env file with your API keys and database credentials."
    echo "Required variables:"
    echo "  - SUPABASE_URL"
    echo "  - SUPABASE_API_KEY"
    echo "  - FMP_API_KEY"
    echo "  - ALPHA_VANTAGE_API_KEY (optional)"
    echo "  - FINNHUB_API_KEY (optional)"
    exit 1
fi

# Load environment variables
source .env

# Check required environment variables
if [ -z "$SUPABASE_URL" ] || [ -z "$SUPABASE_API_KEY" ] || [ -z "$FMP_API_KEY" ]; then
    echo "❌ Error: Missing required environment variables!"
    echo "Please ensure SUPABASE_URL, SUPABASE_API_KEY, and FMP_API_KEY are set in your .env file."
    exit 1
fi

echo "✅ Environment variables loaded"
echo "🔗 Supabase URL: $SUPABASE_URL"
echo "🔑 FMP API Key: ${FMP_API_KEY:0:8}..."
if [ ! -z "$ALPHA_VANTAGE_API_KEY" ]; then
    echo "🔑 Alpha Vantage API Key: ${ALPHA_VANTAGE_API_KEY:0:8}..."
fi
if [ ! -z "$FINNHUB_API_KEY" ]; then
    echo "🔑 Finnhub API Key: ${FINNHUB_API_KEY:0:8}..."
fi
echo ""

# Build and run the sentiment analysis
echo "🔨 Building sentiment batch analysis..."
cargo build --example sentiment_batch_analysis --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build successful"
echo ""

# Run the sentiment analysis
echo "🚀 Starting batch sentiment analysis..."
echo "This may take several minutes depending on the number of stocks."
echo ""

cargo run --example sentiment_batch_analysis --release

if [ $? -eq 0 ]; then
    echo ""
    echo "🎉 Batch sentiment analysis completed successfully!"
    echo "📊 Check your Supabase database for the results."
else
    echo ""
    echo "❌ Batch sentiment analysis failed!"
    echo "Check the error messages above for details."
    exit 1
fi


