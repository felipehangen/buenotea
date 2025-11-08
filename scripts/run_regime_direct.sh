#!/bin/bash

# Direct regime analysis runner - no .env file modification
# Provide your Supabase credentials directly when running

echo "🎯 Regime Analysis System - Direct Database Save"
echo "================================================"
echo ""

# Get Supabase credentials from user
echo "Please provide your Supabase credentials:"
echo ""
read -p "Enter your Supabase URL (e.g., https://your-project.supabase.co): " SUPABASE_URL
read -p "Enter your Supabase Anon Key: " SUPABASE_ANON_KEY

echo ""
echo "✅ Credentials received!"
echo "   URL: $SUPABASE_URL"
echo "   Key: ${SUPABASE_ANON_KEY:0:20}..."
echo ""

echo "🚀 Running regime analysis and saving to database..."
echo ""

# Export environment variables for this session only
export SUPABASE_URL
export SUPABASE_ANON_KEY

# Run the regime analysis
cargo run --example regime_save_to_db

echo ""
echo "🎉 Regime analysis complete!"
echo ""
echo "📊 Check your Supabase database 'regime' table to see the saved data"
echo "📁 JSON files are also saved locally for reference"

