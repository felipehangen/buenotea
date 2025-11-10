#!/bin/bash
# Migration script to convert fundamentals to time-series history

set -e

echo "🔄 Starting Fundamentals Time-Series Migration..."

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
MIGRATION_FILE="$SCRIPT_DIR/../migrations/convert_fundamentals_to_history.sql"

if [ ! -f "$MIGRATION_FILE" ]; then
    echo "❌ Error: Migration file not found at $MIGRATION_FILE"
    exit 1
fi

echo "📄 Migration file: $MIGRATION_FILE"
echo ""
echo "This migration will:"
echo "  1. Drop existing functions (get_fundamentals_at_date, get_fundamentals_history, get_fundamentals_changes)"
echo "  2. Drop existing views (latest_fundamentals, fundamentals)"
echo "  3. Drop and recreate fundamentals_history table"
echo "  4. Create fundamentals view (shows latest analysis per symbol)"
echo "  5. Create helper functions for querying history"
echo ""
echo "⚠️  WARNING: This will DROP the existing fundamentals_history table if it exists!"
echo "⚠️  All existing fundamentals data will be LOST!"
echo ""
read -p "Do you want to continue? (yes/no) " -n 3 -r
echo
if [[ ! $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
    echo "❌ Migration cancelled"
    exit 1
fi

echo ""
echo "🚀 Running migration..."

# Execute migration using Supabase SQL API
SQL_CONTENT=$(cat "$MIGRATION_FILE")

RESPONSE=$(curl -s -w "\n%{http_code}" -X POST \
    "${SUPABASE_URL}/rest/v1/rpc/exec_sql" \
    -H "apikey: ${SUPABASE_SERVICE_KEY}" \
    -H "Authorization: Bearer ${SUPABASE_SERVICE_KEY}" \
    -H "Content-Type: application/json" \
    -d "{\"query\": $(jq -Rs . <<< "$SQL_CONTENT")}")

HTTP_CODE=$(echo "$RESPONSE" | tail -n 1)
BODY=$(echo "$RESPONSE" | sed '$d')

if [ "$HTTP_CODE" -eq 200 ] || [ "$HTTP_CODE" -eq 201 ]; then
    echo "✅ Migration completed successfully!"
    echo ""
    echo "📊 Summary:"
    echo "   - Created fundamentals_history table (time-series storage)"
    echo "   - Created fundamentals view (latest analysis per symbol)"
    echo "   - Created get_fundamentals_at_date() function"
    echo "   - Created get_fundamentals_history() function"
    echo "   - Created get_fundamentals_changes() function"
    echo ""
    echo "🎉 Fundamentals time-series migration complete!"
    echo ""
    echo "Usage examples:"
    echo "  1. Get latest fundamentals: SELECT * FROM fundamentals WHERE symbol = 'AAPL';"
    echo "  2. Get history: SELECT * FROM get_fundamentals_history('AAPL', 365);"
    echo "  3. Get changes: SELECT * FROM get_fundamentals_changes('AAPL', 90);"
else
    echo "❌ Migration failed with HTTP code: $HTTP_CODE"
    echo "Response: $BODY"
    exit 1
fi

