# 🚀 Quick Start Guide

## 1. Environment Setup

Create a `.env` file in the project root:

```bash
# Required
SUPABASE_URL=your_supabase_url
SUPABASE_API_KEY=your_supabase_api_key

# Optional (for better data coverage)
FMP_API_KEY=your_fmp_api_key
ALPHA_VANTAGE_API_KEY=your_alpha_vantage_api_key
FINNHUB_API_KEY=your_finnhub_api_key
OPENAI_API_KEY=your_openai_api_key
```

## 2. Database Setup

```bash
./scripts/setup_supabase.sh
```

## 3. Run Analysis

### Sentiment Analysis
```bash
./scripts/run_sentiment_analysis.sh
```

### Fundamentals Analysis
```bash
./scripts/run_fundamentals_batch_analysis.sh
```

### Market Regime Analysis
```bash
./scripts/run_regime_analysis.sh
```

### Timing Analysis
```bash
./scripts/run_timing_batch_analysis.sh
```

## 4. Check Status

```bash
./scripts/run_fundamentals_status_check.sh
```

## 5. Examples

### Run Individual Examples
```bash
# Sentiment analysis
cargo run --example sentiment/sentiment_to_supabase

# Fundamentals analysis
cargo run --example fundamentals/fundamentals_batch_analysis

# Regime analysis
cargo run --example regime/regime_analysis_to_supabase -- AAPL

# Timing analysis
cargo run --example timing/timing_analysis_batch
```

## 6. Project Structure

```
OKTStocks2/
├── README.md                 # Main documentation
├── docs/                     # Documentation
├── scripts/                  # Shell scripts
├── examples/                 # Code examples
│   ├── sentiment/          # Sentiment examples
│   ├── fundamentals/        # Fundamentals examples
│   ├── regime/              # Regime examples
│   ├── timing/              # Timing examples
│   └── invite_list/         # Invite list examples
├── src/                      # Source code
│   ├── sentiment/           # Sentiment analysis
│   ├── fundamentals/        # Fundamentals analysis
│   ├── regime/              # Market regime detection
│   ├── timing/              # Technical timing
│   ├── database/            # Database integration
│   ├── ai/                  # AI integration
│   └── RULES/               # Project rules
└── database_migrations/     # SQL migrations
```

## 7. Available Scripts

### Analysis Scripts
- `scripts/run_sentiment_analysis.sh` - Complete sentiment analysis
- `scripts/run_fundamentals_batch_analysis.sh` - Batch fundamentals analysis
- `scripts/run_regime_analysis.sh` - Market regime analysis
- `scripts/run_timing_batch_analysis.sh` - Technical timing analysis

### Utility Scripts
- `scripts/setup_supabase.sh` - Database setup
- `scripts/run_fundamentals_status_check.sh` - Check analysis status

## 8. Troubleshooting

### Common Issues
1. **Missing environment variables**: Check your `.env` file
2. **Database connection errors**: Verify Supabase credentials
3. **API rate limits**: Check API quotas and keys
4. **Compilation errors**: Run `cargo clean && cargo build`

### Debug Mode
```bash
RUST_LOG=debug cargo run --example <example_name>
```

## 9. Performance Tips

1. **API Keys**: Configure all API keys for better data coverage
2. **Batch Size**: Adjust batch sizes in examples if needed
3. **Rate Limiting**: Respect API rate limits
4. **Database**: Ensure Supabase is properly configured

## 10. Next Steps

After running the analysis:
1. Check the database for results
2. Set up a dashboard to visualize data
3. Configure cron jobs for automated analysis
4. Monitor data quality and coverage
