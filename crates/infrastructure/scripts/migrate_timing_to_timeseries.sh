#!/bin/bash

# Script to apply the timing time-series migration

echo "🔧 Applying timing time-series migration..."
echo ""

# Load environment variables
if [ -f .env ]; then
    source .env
    echo "✅ .env file loaded."
else
    echo "❌ .env file not found. Please create one with SUPABASE_URL and SUPABASE_API_KEY."
    exit 1
fi

# Check for required environment variables
if [ -z "$SUPABASE_URL" ] || [ -z "$SUPABASE_API_KEY" ]; then
    echo "❌ SUPABASE_URL or SUPABASE_API_KEY not set in .env file."
    exit 1
fi

# Extract host and project ID from SUPABASE_URL
SUPABASE_HOST=$(echo "$SUPABASE_URL" | sed -E 's|https://([^.]+)\.supabase\.co|\1.supabase.co|')
SUPABASE_PROJECT_ID=$(echo "$SUPABASE_URL" | sed -E 's|https://([^.]+)\.supabase\.co|\1|')

echo "📡 Connecting to Supabase project: $SUPABASE_PROJECT_ID"
echo ""

# Use psql to apply the migration file
PGPASSWORD="$SUPABASE_API_KEY" psql \
    -h "$SUPABASE_HOST" \
    -p 5432 \
    -U postgres \
    -d postgres \
    -f crates/infrastructure/migrations/convert_timing_to_history.sql

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Timing time-series migration applied successfully!"
    echo ""
    echo "📊 What changed:"
    echo "  - Renamed 'timing' table to 'timing_history'"
    echo "  - Created 'timing' VIEW for latest records"
    echo "  - Added functions for historical queries:"
    echo "    • get_timing_at_date(date) - Get signals at a specific date"
    echo "    • get_timing_history(symbol, days) - Track signal changes"
    echo "    • get_timing_signal_changes(days) - Find signal flips"
    echo "    • get_stocks_by_signal(signal) - Filter by signal type"
    echo ""
    echo "💡 Your timing data now supports time-series tracking!"
else
    echo ""
    echo "❌ Failed to apply timing time-series migration."
    exit 1
fi

