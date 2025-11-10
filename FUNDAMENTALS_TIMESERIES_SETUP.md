# Fundamentals Time-Series Setup

This document explains the time-series approach for tracking fundamentals analysis over time.

## Overview

The fundamentals study has been refactored to use a **time-series** approach that stores historical analysis data in a `fundamentals_history` table. This enables tracking fundamentals score changes, component scores, and financial metrics over time.

## Database Schema

### Tables

1. **`fundamentals_history`** - Stores all fundamentals analyses over time
   - Primary key: `id` (BIGSERIAL)
   - Unique constraint: `(symbol, analysis_date)` - prevents duplicate analyses for the same symbol on the same date
   - Tracks: scores, metrics, API data, and analysis metadata

2. **`fundamentals`** (VIEW) - Shows the latest fundamentals analysis for each symbol
   - This is a view, not a table
   - Automatically shows the most recent analysis per symbol
   - Query this for current fundamentals

### Functions

1. **`get_fundamentals_at_date(symbol, date)`** - Get fundamentals that was valid at a specific date
2. **`get_fundamentals_history(symbol, days)`** - Get all analyses for a symbol over N days
3. **`get_fundamentals_changes(symbol, days)`** - Find significant score changes (>= 5 points)

## Migration

### Automated Migration (Recommended)

Run the migration script:

```bash
./crates/infrastructure/scripts/migrate_fundamentals_to_timeseries.sh
```

This will:
- Drop existing functions and views
- Drop and recreate `fundamentals_history` table
- Create `fundamentals` view
- Create helper functions

**⚠️ WARNING**: This drops all existing fundamentals data!

### Manual Migration (via Supabase Dashboard)

If the script doesn't work:

1. Go to your Supabase project dashboard
2. Navigate to SQL Editor
3. Open and run: `crates/infrastructure/migrations/convert_fundamentals_to_history.sql`

## Code Structure

### Infrastructure Crate

**`crates/infrastructure/src/database/fundamentals_models.rs`**
- `FundamentalsRecord` - Full record (read from database)
- `CreateFundamentalsRecord` - Record creation request
- `FundamentalsInsert` - Insert payload (without auto-generated fields)

### Fundamentals Crate

**`crates/studies/fundamentals/src/fundamentals_storage.rs`**
- `FundamentalsStorage::store_fundamentals_record()` - Store a single analysis
- `FundamentalsStorage::store_multiple_records()` - Batch store
- `FundamentalsStorage::get_latest_fundamentals()` - Get latest for a symbol
- `FundamentalsStorage::get_all_latest_fundamentals()` - Get all latest
- `FundamentalsStorage::get_fundamentals_history()` - Get history
- `FundamentalsStorage::get_fundamentals_changes()` - Get score changes

**`crates/studies/fundamentals/src/fundamentals_models.rs`**
- `create_fundamentals_record_with_tracking()` - Helper to convert `FundamentalsResult` to `CreateFundamentalsRecord`

## Usage Examples

### 1. Run fundamentals analysis and store to database

```bash
cargo run --example fundamentals_batch_to_supabase --package buenotea-fundamentals
```

### 2. Query latest fundamentals (SQL)

```sql
-- Get latest fundamentals for AAPL
SELECT * FROM fundamentals WHERE symbol = 'AAPL';

-- Get all latest fundamentals
SELECT * FROM fundamentals;
```

### 3. Query historical data (SQL)

```sql
-- Get AAPL fundamentals history (365 days)
SELECT * FROM get_fundamentals_history('AAPL', 365);

-- Get fundamentals 30 days ago
SELECT * FROM get_fundamentals_at_date('AAPL', NOW() - INTERVAL '30 days');

-- Find significant score changes
SELECT * FROM get_fundamentals_changes('AAPL', 90);
```

### 4. Programmatic usage (Rust)

```rust
use buenotea_fundamentals::{FundamentalsCalculator, FundamentalsStorage, FundamentalsApiUrls, create_fundamentals_record_with_tracking};
use buenotea_infrastructure::DatabaseClient;

// Run analysis
let mut calculator = FundamentalsCalculator::new();
let result = calculator.calculate_fundamentals("AAPL").await?;

// Prepare API tracking
let api_urls = FundamentalsApiUrls::default();

// Create database record
let record = create_fundamentals_record_with_tracking(
    result,
    api_urls,
    None, // gpt_explanation
    None, // gpt_trading_suggestion
);

// Store to database
let db_client = DatabaseClient::from_env()?;
let storage = FundamentalsStorage::new(db_client);
let id = storage.store_fundamentals_record(&record).await?;

println!("Stored with ID: {}", id);
```

## Benefits of Time-Series Approach

### 1. Historical Tracking
- Track how fundamentals scores change over time
- Identify trends in financial metrics
- Compare analyses from different dates

### 2. Change Detection
- Detect significant score changes (>= 5 points)
- Monitor improvement or deterioration
- Alert on major shifts

### 3. Backtesting
- Test trading strategies using historical fundamentals
- Validate scoring methodology
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

### Core Scoring
- `fundamentals_score` - Overall score (0-100)
- `trading_signal` - Buy/Sell recommendation
- `confidence_score` - Confidence in analysis (0-1)

### Component Scores
- `profitability_score` - Profitability analysis
- `growth_score` - Growth analysis
- `valuation_score` - Valuation analysis
- `financial_strength_score` - Balance sheet strength
- `efficiency_score` - Operational efficiency

### Metrics (50+ fields)
- **Profitability**: ROE, ROA, ROIC, margins
- **Growth**: Revenue growth, EPS growth
- **Valuation**: P/E, PEG, P/B, EV/EBITDA
- **Strength**: Debt ratios, current ratio, Altman Z-score
- **Efficiency**: Turnover ratios, DSO, DIO, DPO

### Company Metadata
- Sector, industry, market cap
- Beta, dividend yield, shares outstanding

### API Tracking
- URLs, sources, raw data for each metric category
- Data availability flags
- Computation time and data freshness

## Monitoring

### Check recent analyses

```sql
SELECT symbol, analysis_date, fundamentals_score, trading_signal, confidence_score
FROM fundamentals_history
ORDER BY created_at DESC
LIMIT 20;
```

### Count analyses per symbol

```sql
SELECT symbol, COUNT(*) as analysis_count
FROM fundamentals_history
GROUP BY symbol
ORDER BY analysis_count DESC;
```

### Check for duplicates

```sql
SELECT symbol, analysis_date, COUNT(*) as count
FROM fundamentals_history
GROUP BY symbol, analysis_date
HAVING COUNT(*) > 1;
```

## Troubleshooting

### Issue: Duplicate analysis error

**Error**: `duplicate key value violates unique constraint "unique_fundamentals_symbol_date"`

**Cause**: Trying to insert multiple analyses for the same symbol on the same date

**Solution**: Each symbol can only have one analysis per date. If you need to re-run, either:
1. Use a different `analysis_date`
2. Delete the existing record first
3. Update the existing record instead of inserting

### Issue: Migration fails

**Error**: `relation "fundamentals_history" already exists`

**Solution**: The migration script drops existing tables. If it fails, manually run:

```sql
DROP TABLE IF EXISTS fundamentals_history CASCADE;
DROP VIEW IF EXISTS fundamentals CASCADE;
```

Then re-run the migration.

### Issue: Function not found

**Error**: `function get_fundamentals_history does not exist`

**Solution**: Run the migration again. The functions are created as part of the migration.

## Next Steps

1. ✅ Run migration to set up database schema
2. ✅ Test with example: `cargo run --example fundamentals_batch_to_supabase`
3. 📈 Integrate with CLI for batch processing
4. 🌐 Add to website for visualization
5. 📊 Create dashboards for fundamentals tracking

## Related Files

- **Migration**: `crates/infrastructure/migrations/convert_fundamentals_to_history.sql`
- **Script**: `crates/infrastructure/scripts/migrate_fundamentals_to_timeseries.sh`
- **Models**: `crates/infrastructure/src/database/fundamentals_models.rs`
- **Storage**: `crates/studies/fundamentals/src/fundamentals_storage.rs`
- **Helper**: `crates/studies/fundamentals/src/fundamentals_models.rs`
- **Example**: `crates/studies/fundamentals/examples/fundamentals_batch_to_supabase.rs`

