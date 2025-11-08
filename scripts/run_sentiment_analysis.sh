#!/bin/bash

# Sentiment Analysis Script
# This script can run sentiment analysis in different modes:
# - test: Test with a small subset of stocks
# - batch: Run batch analysis on all stocks
# - parallel: Run parallel batch analysis
# - robust: Run robust batch analysis with retry logic

MODE=${1:-"test"}

echo "=== Sentiment Analysis Script ==="
echo "Mode: $MODE"
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

# Build the appropriate example
case $MODE in
    "test")
        echo "🧪 Building test sentiment analysis..."
        cargo build --example sentiment_test_subset --release
        EXAMPLE="sentiment_test_subset"
        DESCRIPTION="Test sentiment analysis on 3 stocks (AAPL, MSFT, GOOGL)"
        ;;
    "batch")
        echo "🔨 Building batch sentiment analysis..."
        cargo build --example sentiment_batch_analysis --release
        EXAMPLE="sentiment_batch_analysis"
        DESCRIPTION="Sequential batch sentiment analysis on all stocks"
        ;;
    "parallel")
        echo "🔨 Building parallel batch sentiment analysis..."
        cargo build --example sentiment_batch_parallel --release
        EXAMPLE="sentiment_batch_parallel"
        DESCRIPTION="Parallel batch sentiment analysis on all stocks"
        ;;
    "robust")
        echo "🔨 Building robust batch sentiment analysis..."
        cargo build --example sentiment_batch_robust --release
        EXAMPLE="sentiment_batch_robust"
        DESCRIPTION="Robust parallel batch sentiment analysis with retry logic"
        ;;
    *)
        echo "❌ Error: Invalid mode '$MODE'"
        echo "Available modes:"
        echo "  - test: Test with a small subset of stocks"
        echo "  - batch: Run batch analysis on all stocks"
        echo "  - parallel: Run parallel batch analysis"
        echo "  - robust: Run robust batch analysis with retry logic"
        exit 1
        ;;
esac

if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build successful"
echo "📊 $DESCRIPTION"
echo ""

# Run the sentiment analysis
echo "🚀 Starting sentiment analysis..."
echo "This may take several minutes depending on the mode and number of stocks."
echo ""

cargo run --example $EXAMPLE --release

if [ $? -eq 0 ]; then
    echo ""
    echo "🎉 Sentiment analysis completed successfully!"
    echo "📊 Check your Supabase database for the results."
    
    if [ "$MODE" = "test" ]; then
        echo ""
        echo "💡 If the test was successful, you can now run the full batch analysis:"
        echo "   ./run_sentiment_analysis.sh robust"
    fi
else
    echo ""
    echo "❌ Sentiment analysis failed!"
    echo "Check the error messages above for details."
    exit 1
fi


