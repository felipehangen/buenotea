# Sentiment (QSS) Time-Series Setup

This document explains the time-series approach for tracking sentiment (QSS - Quantitative Sentiment Score) analysis over time.

## Overview

The sentiment study has been refactored to use a **time-series** approach that stores historical analysis data in a `sentiment_history` table. This enables tracking sentiment score changes, component scores, and market sentiment over time.

## Database Schema

### Tables

1. **`sentiment_history`** - Stores all sentiment analyses over time
   - Primary key: `id` (BIGSERIAL)
   - Unique constraint: `(symbol, analysis_date)` - prevents duplicate analyses for the same symbol on the same date
   - Tracks: QSS scores, component scores, API data, and analysis metadata

2. **`sentiment`** (VIEW) - Shows the latest sentiment analysis for each symbol
   - This is a view, not a table
   - Automatically shows the most recent analysis per symbol
   - Query this for current sentiment

### Functions

1. **`get_sentiment_at_date(symbol, date)`** - Get sentiment that was valid at a specific date
2. **`get_sentiment_history(symbol, days)`** - Get all analyses for a symbol over N days
3. **`get_sentiment_changes(symbol, days)`** - Find significant score changes (>= 0.10)

## Migration

### Manual Migration (Recommended)

1. Go to your Supabase project dashboard
2. Navigate to SQL Editor
3. Open and run: `crates/infrastructure/migrations/convert_sentiment_to_history.sql`

**⚠️ WARNING**: This drops all existing sentiment data!

## Code Structure

### Infrastructure Crate

**`crates/infrastructure/src/database/sentiment_models.rs`**
- `SentimentRecord` - Full record (read from database)
- `CreateSentimentRecord` - Record creation request
- `SentimentInsert` - Insert payload (without auto-generated fields)
- `ApiUrls` - API tracking data structure

### Sentiment Crate

**`crates/studies/sentiment/src/sentiment_storage.rs`**
- `SentimentStorage::store_sentiment_record()` - Store a single analysis
- `SentimentStorage::store_multiple_records()` - Batch store
- `SentimentStorage::get_latest_sentiment()` - Get latest for a symbol
- `SentimentStorage::get_all_latest_sentiment()` - Get all latest
- `SentimentStorage::get_sentiment_history()` - Get history
- `SentimentStorage::get_sentiment_changes()` - Get score changes

**`crates/studies/sentiment/src/sentiment_models.rs`**
- `create_sentiment_record_with_tracking()` - Helper to convert `QSSResult` to `CreateSentimentRecord`

## Usage Examples

### 1. Run sentiment analysis and store to database

```bash
cargo run --example sentiment_batch_to_supabase --package buenotea-sentiment
```

### 2. Query latest sentiment (SQL)

```sql
-- Get latest sentiment for AAPL
SELECT * FROM sentiment WHERE symbol = 'AAPL';

-- Get all latest sentiment
SELECT * FROM sentiment;
```

### 3. Query historical data (SQL)

```sql
-- Get AAPL sentiment history (90 days)
SELECT * FROM get_sentiment_history('AAPL', 90);

-- Get sentiment 30 days ago
SELECT * FROM get_sentiment_at_date('AAPL', NOW() - INTERVAL '30 days');

-- Find significant score changes
SELECT * FROM get_sentiment_changes('AAPL', 30);
```

### 4. Programmatic usage (Rust)

```rust
use buenotea_sentiment::{QSSCalculator, SentimentStorage, create_sentiment_record_with_tracking};
use buenotea_infrastructure::sentiment_models::ApiUrls;
use buenotea_infrastructure::DatabaseClient;

// Run analysis
let calculator = QSSCalculator::new();
let result = calculator.calculate_qss("AAPL").await?;

// Prepare API tracking
let api_urls = ApiUrls::default();

// Generate explanation
let gpt_explanation = format!(
    "{} shows {:?} sentiment with QSS score of {:.3}",
    result.symbol,
    result.trading_signal,
    result.qss_score
);

// Create database record
let record = create_sentiment_record_with_tracking(
    result,
    api_urls,
    gpt_explanation,
);

// Store to database
let db_client = DatabaseClient::from_env()?;
let storage = SentimentStorage::new(db_client);
let id = storage.store_sentiment_record(&record).await?;

println!("Stored with ID: {}", id);
```

## Benefits of Time-Series Approach

### 1. Historical Tracking
- Track how sentiment scores change over time
- Identify trends in market sentiment
- Compare analyses from different dates

### 2. Change Detection
- Detect significant score changes (>= 0.10)
- Monitor sentiment shifts
- Alert on major changes

### 3. Backtesting
- Test trading strategies using historical sentiment
- Validate QSS methodology
- Improve signal generation

### 4. Audit Trail
- Complete record of all analyses
- API data preserved
- Metadata for reproducibility

## Key Differences from Old Approach

### Before (UPSERT)
- Only one record per symbol (latest)
- Historical data lost on updates
- No way to track changes

### After (Time-Series INSERT)
- All analyses preserved
- Each analysis is a new record
- Full historical tracking
- Query data at any point in time

## Database Fields

### Core QSS Scoring
- `qss_score` - Overall sentiment score (-1 to +1)
- `trading_signal` - Buy/Sell recommendation
- `confidence_score` - Confidence in analysis (0-1)

### Component Scores
- `earnings_revisions_score` - Earnings sentiment (40% weight)
- `relative_strength_score` - Price momentum (30% weight)
- `short_interest_score` - Short interest sentiment (20% weight)
- `options_flow_score` - Options activity (10% weight)

### API Tracking
- URLs, sources, raw data for each data source
- Data availability flags
- Computation time and data freshness

### Market Context
- RSI, price data, earnings estimates
- Market and sector benchmarks
- Volume ratios and trend direction

## Monitoring

### Check recent analyses

```sql
SELECT symbol, analysis_date, qss_score, trading_signal, confidence_score
FROM sentiment_history
ORDER BY created_at DESC
LIMIT 20;
```

### Count analyses per symbol

```sql
SELECT symbol, COUNT(*) as analysis_count
FROM sentiment_history
GROUP BY symbol
ORDER BY analysis_count DESC;
```

### Check for duplicates

```sql
SELECT symbol, analysis_date, COUNT(*) as count
FROM sentiment_history
GROUP BY symbol, analysis_date
HAVING COUNT(*) > 1;
```

## Troubleshooting

### Issue: Duplicate analysis error

**Error**: `duplicate key value violates unique constraint "unique_sentiment_symbol_date"`

**Cause**: Trying to insert multiple analyses for the same symbol on the same date

**Solution**: Each symbol can only have one analysis per date. If you need to re-run, either:
1. Use a different `analysis_date`
2. Delete the existing record first
3. Update the existing record instead of inserting

### Issue: Migration fails

**Error**: `relation "sentiment_history" already exists`

**Solution**: The migration script drops existing tables. If it fails, manually run:

```sql
DROP TABLE IF EXISTS sentiment_history CASCADE;
DROP VIEW IF EXISTS sentiment CASCADE;
```

Then re-run the migration.

### Issue: Function not found

**Error**: `function get_sentiment_history does not exist`

**Solution**: Run the migration again. The functions are created as part of the migration.

## Next Steps

1. ✅ Run migration to set up database schema
2. ✅ Test with example: `cargo run --example sentiment_batch_to_supabase`
3. 📈 Integrate with CLI for batch processing
4. 🌐 Add to website for visualization
5. 📊 Create dashboards for sentiment tracking

## Related Files

- **Migration**: `crates/infrastructure/migrations/convert_sentiment_to_history.sql`
- **Script**: `crates/infrastructure/scripts/migrate_sentiment_to_timeseries.sh`
- **Models**: `crates/infrastructure/src/database/sentiment_models.rs`
- **Storage**: `crates/studies/sentiment/src/sentiment_storage.rs`
- **Helper**: `crates/studies/sentiment/src/sentiment_models.rs`
- **Example**: `crates/studies/sentiment/examples/sentiment_batch_to_supabase.rs`

## Summary

All 5 studies now support historical time-series tracking:

1. ✅ **Timing** - `timing_history` table
2. ✅ **Invite List** - `invite_list_history` table
3. ✅ **Regime** - `market_regime_history` table
4. ✅ **Fundamentals** - `fundamentals_history` table
5. ✅ **Sentiment** - `sentiment_history` table

Each study follows the same pattern:
- Time-series history table with unique constraint (symbol, analysis_date)
- View for latest analysis per symbol
- Helper functions for querying history and changes
- Complete audit trail with API tracking

