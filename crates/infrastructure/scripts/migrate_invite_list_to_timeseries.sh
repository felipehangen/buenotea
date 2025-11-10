#!/bin/bash

# Script to migrate invite_list to time-series structure
# This converts the table to track historical safety changes

set -e

echo "🔄 Migrating invite_list to time-series structure..."
echo ""

# Check if .env file exists
if [ ! -f .env ]; then
    echo "❌ Error: .env file not found"
    echo "Please create a .env file with SUPABASE_URL and SUPABASE_API_KEY"
    exit 1
fi

# Load environment variables
source .env

# Check if required variables are set
if [ -z "$SUPABASE_URL" ] || [ -z "$SUPABASE_API_KEY" ]; then
    echo "❌ Error: SUPABASE_URL or SUPABASE_API_KEY not set in .env"
    exit 1
fi

echo "✅ Environment variables loaded"
echo "📍 Supabase URL: $SUPABASE_URL"
echo ""

# Get the migration file path
MIGRATION_FILE="crates/infrastructure/migrations/convert_invite_list_to_history.sql"

if [ ! -f "$MIGRATION_FILE" ]; then
    echo "❌ Error: Migration file not found at $MIGRATION_FILE"
    exit 1
fi

echo "📄 Migration file found: $MIGRATION_FILE"
echo ""

# Extract project ID from Supabase URL
PROJECT_ID=$(echo $SUPABASE_URL | sed -E 's|https://([^.]+)\.supabase\.co|\1|')

echo "🔍 Detected Supabase Project ID: $PROJECT_ID"
echo ""

echo "⚠️  WARNING: This migration will:"
echo "   1. Rename 'invite_list' table to 'invite_list_history'"
echo "   2. Remove UNIQUE constraint on symbol (allow multiple records per stock)"
echo "   3. Add composite UNIQUE(symbol, analysis_date)"
echo "   4. Create views and helper functions"
echo ""
echo "   Your existing data will be preserved!"
echo ""

read -p "Do you want to proceed? (yes/no): " -r
echo ""

if [[ ! $REPLY =~ ^[Yy]es$ ]]; then
    echo "❌ Migration cancelled"
    exit 0
fi

echo "🚀 Running migration..."
echo ""

# Option 1: If you have psql installed and database connection string
if command -v psql &> /dev/null && [ ! -z "$DATABASE_URL" ]; then
    echo "📊 Using psql to run migration..."
    psql $DATABASE_URL -f $MIGRATION_FILE
    
    if [ $? -eq 0 ]; then
        echo ""
        echo "✅ Migration completed successfully!"
    else
        echo ""
        echo "❌ Migration failed!"
        exit 1
    fi
else
    # Option 2: Instructions for manual migration via Supabase dashboard
    echo "📋 Manual Migration Instructions:"
    echo ""
    echo "1. Go to your Supabase dashboard: https://app.supabase.com/project/$PROJECT_ID/sql"
    echo ""
    echo "2. Copy the contents of: $MIGRATION_FILE"
    echo ""
    echo "3. Paste into the SQL Editor and click 'Run'"
    echo ""
    echo "4. Verify the migration by running:"
    echo "   SELECT COUNT(*) FROM invite_list_history;"
    echo "   SELECT * FROM invite_list LIMIT 5;"
    echo ""
    
    # Optionally show the SQL
    read -p "Would you like to see the SQL now? (yes/no): " -r
    if [[ $REPLY =~ ^[Yy]es$ ]]; then
        echo ""
        echo "========== MIGRATION SQL =========="
        cat $MIGRATION_FILE
        echo ""
        echo "========== END OF SQL =========="
    fi
fi

echo ""
echo "📚 Documentation: See crates/studies/invite-list/TIMESERIES_GUIDE.md for usage examples"
echo ""
echo "🎯 Next steps:"
echo "   1. Run invite_list analysis: cargo run --example invite_list_to_supabase -p buenotea-invite-list"
echo "   2. Check historical data: SELECT * FROM invite_list_history ORDER BY analysis_date DESC;"
echo "   3. View current safe stocks: SELECT * FROM safe_stocks;"
echo ""


