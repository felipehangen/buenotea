#!/bin/bash
# Migration script to convert sentiment to time-series history

set -e

echo "🔄 Starting Sentiment Time-Series Migration..."

# Check if .env file exists
if [ ! -f .env ]; then
    echo "❌ Error: .env file not found"
    exit 1
fi

# Load environment variables
source .env

# Check if required variables are set
if [ -z "$SUPABASE_URL" ] || [ -z "$SUPABASE_SERVICE_KEY" ]; then
    echo "❌ Error: SUPABASE_URL and SUPABASE_SERVICE_KEY must be set in .env"
    exit 1
fi

echo "📦 Supabase URL: $SUPABASE_URL"
echo "🔑 Using service key for migration"

# Get the migration file path
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MIGRATION_FILE="$SCRIPT_DIR/../migrations/convert_sentiment_to_history.sql"

if [ ! -f "$MIGRATION_FILE" ]; then
    echo "❌ Error: Migration file not found at $MIGRATION_FILE"
    exit 1
fi

echo "📄 Migration file: $MIGRATION_FILE"
echo ""
echo "This migration will:"
echo "  1. Drop existing functions (get_sentiment_at_date, get_sentiment_history, get_sentiment_changes)"
echo "  2. Drop existing views (latest_sentiment, sentiment)"
echo "  3. Drop and recreate sentiment_history table"
echo "  4. Create sentiment view (shows latest analysis per symbol)"
echo "  5. Create helper functions for querying history"
echo ""
echo "⚠️  WARNING: This will DROP the existing sentiment_history table if it exists!"
echo "⚠️  All existing sentiment data will be LOST!"
echo ""
read -p "Do you want to continue? (yes/no) " -n 3 -r
echo
if [[ ! $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
    echo "❌ Migration cancelled"
    exit 1
fi

echo ""
echo "🚀 Running migration..."
echo "⚠️  Note: This script assumes you'll run the migration manually via Supabase SQL Editor"
echo ""
echo "Please follow these steps:"
echo "1. Go to your Supabase project dashboard"
echo "2. Navigate to SQL Editor"
echo "3. Open and run: $MIGRATION_FILE"
echo ""
echo "After running the migration, you can test it with:"
echo "  cargo run --example sentiment_batch_to_supabase --package buenotea-sentiment"
echo ""

