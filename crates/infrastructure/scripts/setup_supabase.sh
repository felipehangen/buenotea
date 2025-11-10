#!/bin/bash

# Setup script for Supabase environment variables
# This script helps you configure the necessary environment variables for the regime analysis system

echo "🚀 Setting up Supabase environment variables for regime analysis..."
echo ""

# Check if .env file exists
if [ ! -f .env ]; then
    echo "📝 Creating .env file..."
    touch .env
else
    echo "📝 .env file already exists, updating..."
fi

echo ""
echo "Please provide your Supabase configuration:"
echo ""

# Get Supabase URL
read -p "Enter your Supabase URL (e.g., https://your-project.supabase.co): " SUPABASE_URL

# Get Supabase Anon Key
read -p "Enter your Supabase Anon Key: " SUPABASE_ANON_KEY

# Get OpenAI API Key (optional)
read -p "Enter your OpenAI API Key (optional, for ChatGPT analysis): " OPENAI_API_KEY

echo ""
echo "📝 Writing environment variables to .env file..."

# Write to .env file
cat > .env << EOF
# Supabase Configuration
SUPABASE_URL=$SUPABASE_URL
SUPABASE_ANON_KEY=$SUPABASE_ANON_KEY

# OpenAI Configuration (optional)
OPENAI_API_KEY=$OPENAI_API_KEY

# Financial APIs (optional)
FMP_API_KEY=your_fmp_api_key_here
ALPHA_VANTAGE_API_KEY=your_alpha_vantage_api_key_here
FINNHUB_API_KEY=your_finnhub_api_key_here
EOF

echo "✅ Environment variables written to .env file"
echo ""
echo "🔧 To use these variables, run:"
echo "   source .env"
echo ""
echo "🚀 Then run the regime analysis:"
echo "   cargo run --example regime_to_supabase_real"
echo ""
echo "📊 Or run with mock data (no database):"
echo "   cargo run --example regime_to_supabase"
echo ""
echo "🎯 Make sure your Supabase database has the 'regime' table created from the migration!"

