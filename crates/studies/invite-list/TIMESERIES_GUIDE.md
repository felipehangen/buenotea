# Invite List Time-Series Database Guide

## Overview

The invite list uses a **time-series approach** to track safety analysis changes over time. Each time you run an analysis, a new record is inserted into the `invite_list_history` table, allowing you to see how stock safety evolves.

## Database Structure

### Main Table: `invite_list_history`

Stores all historical safety analyses with a composite unique key on `(symbol, analysis_date)`.

```sql
-- Key fields:
- symbol: Stock ticker (e.g., "AAPL")
- analysis_date: When the analysis was performed
- is_safe_to_trade: Boolean safety flag
- safety_score: Score from 0.00 to 1.00
- risk_level: Low, Medium, High, VeryHigh
```

### Views for Easy Querying

1. **`invite_list`** - Shows the most recent analysis for each stock
2. **`safe_stocks`** - Shows only currently safe stocks
3. **`invite_list_trends`** - Shows safety trends over 30 days
4. **`sector_safety_analysis`** - Current safety by sector
5. **`risk_distribution`** - Distribution by risk level

## Useful Queries

### Get Current Safe Stocks

```sql
-- Using the view (easiest)
SELECT * FROM safe_stocks
ORDER BY safety_score DESC
LIMIT 20;

-- Or directly from history
SELECT DISTINCT ON (symbol)
    symbol, company_name, safety_score, risk_level
FROM invite_list_history
WHERE is_safe_to_trade = TRUE
ORDER BY symbol, analysis_date DESC;
```

### Get Safety History for a Specific Stock

```sql
-- Using the helper function
SELECT * FROM get_stock_safety_history('AAPL', 90);

-- Or manually
SELECT 
    analysis_date,
    is_safe_to_trade,
    safety_score,
    risk_level,
    safety_reasoning
FROM invite_list_history
WHERE symbol = 'AAPL'
ORDER BY analysis_date DESC
LIMIT 10;
```

### Get Historical Analysis at a Specific Date

```sql
-- What was safe to trade 30 days ago?
SELECT * FROM get_invite_list_at_date(NOW() - INTERVAL '30 days')
WHERE is_safe_to_trade = TRUE;
```

### Find Stocks That Changed Safety Status

```sql
-- Stocks that changed in the last 7 days
SELECT * FROM get_safety_changes(7)
ORDER BY ABS(score_change) DESC;

-- Manual query for custom logic
WITH current AS (
    SELECT DISTINCT ON (symbol)
        symbol, is_safe_to_trade as current_safe, safety_score as current_score
    FROM invite_list_history
    ORDER BY symbol, analysis_date DESC
),
week_ago AS (
    SELECT DISTINCT ON (symbol)
        symbol, is_safe_to_trade as old_safe, safety_score as old_score
    FROM invite_list_history
    WHERE analysis_date <= NOW() - INTERVAL '7 days'
    ORDER BY symbol, analysis_date DESC
)
SELECT 
    c.symbol,
    c.current_safe,
    w.old_safe,
    c.current_score,
    w.old_score,
    c.current_score - w.old_score as score_change
FROM current c
JOIN week_ago w ON c.symbol = w.symbol
WHERE c.current_safe != w.old_safe
   OR ABS(c.current_score - w.old_score) > 0.1;
```

### View Safety Trends

```sql
-- Stocks with most consistent safety over 30 days
SELECT * FROM invite_list_trends
WHERE safety_consistency_pct > 80
ORDER BY avg_safety_score DESC;

-- Custom trend analysis
SELECT 
    symbol,
    DATE_TRUNC('day', analysis_date) as date,
    AVG(safety_score) as avg_score,
    COUNT(*) as analysis_count
FROM invite_list_history
WHERE analysis_date >= NOW() - INTERVAL '30 days'
GROUP BY symbol, DATE_TRUNC('day', analysis_date)
ORDER BY symbol, date DESC;
```

### Compare Safety Across Time Periods

```sql
SELECT 
    symbol,
    company_name,
    MAX(CASE WHEN analysis_date >= NOW() - INTERVAL '1 day' 
        THEN safety_score END) as today_score,
    MAX(CASE WHEN analysis_date >= NOW() - INTERVAL '7 days' 
        AND analysis_date < NOW() - INTERVAL '1 day'
        THEN safety_score END) as week_ago_score,
    MAX(CASE WHEN analysis_date >= NOW() - INTERVAL '30 days' 
        AND analysis_date < NOW() - INTERVAL '7 days'
        THEN safety_score END) as month_ago_score
FROM invite_list_history
WHERE symbol IN (SELECT symbol FROM safe_stocks)
GROUP BY symbol, company_name
HAVING COUNT(DISTINCT DATE_TRUNC('day', analysis_date)) >= 3;
```

### Sector Analysis Over Time

```sql
SELECT 
    sector,
    DATE_TRUNC('week', analysis_date) as week,
    AVG(safety_score) as avg_score,
    COUNT(CASE WHEN is_safe_to_trade THEN 1 END) as safe_count,
    COUNT(*) as total_count
FROM invite_list_history
WHERE sector IS NOT NULL
  AND analysis_date >= NOW() - INTERVAL '90 days'
GROUP BY sector, DATE_TRUNC('week', analysis_date)
ORDER BY sector, week DESC;
```

## Code Usage

### Storing Analysis Results

```rust
use buenotea_invite_list::{InviteListCalculator, InviteListStorage};
use buenotea_infrastructure::DatabaseClient;

#[tokio::main]
async fn main() -> Result<()> {
    // Perform analysis
    let calculator = InviteListCalculator::new();
    let results = calculator.analyze_list(&stocks).await?;
    
    // Store in database (creates new historical record)
    let db = DatabaseClient::from_env()?;
    let storage = InviteListStorage::new(db);
    
    for result in results {
        storage.store_invite_list_record(&result).await?;
    }
    
    Ok(())
}
```

### Querying Historical Data

```rust
// Get current safe stocks
let safe_stocks = storage.get_safe_stocks().await?;

// Get specific stock history
let history = storage.get_stock_history("AAPL", 30).await?;

// Get all stocks (most recent analysis)
let all_stocks = storage.get_all_stocks().await?;
```

## Data Maintenance

### Archive Old Data (Keep Last 12 Months)

```sql
-- Manual deletion
DELETE FROM invite_list_history
WHERE analysis_date < NOW() - INTERVAL '12 months';

-- Or use the helper function
SELECT archive_old_invite_list_data(12);  -- keeps 12 months
```

### Create Summary Tables for Long-Term Analysis

```sql
-- Monthly aggregations
CREATE TABLE invite_list_monthly_summary AS
SELECT 
    symbol,
    company_name,
    sector,
    DATE_TRUNC('month', analysis_date) as month,
    AVG(safety_score) as avg_safety_score,
    STDDEV(safety_score) as score_volatility,
    COUNT(*) as analysis_count,
    SUM(CASE WHEN is_safe_to_trade THEN 1 END) as safe_count
FROM invite_list_history
GROUP BY symbol, company_name, sector, DATE_TRUNC('month', analysis_date);

-- Then archive old daily data
DELETE FROM invite_list_history
WHERE analysis_date < NOW() - INTERVAL '3 months';
```

## Migration

To migrate from the old single-record structure to time-series:

```bash
# Run the migration SQL file
psql $DATABASE_URL -f crates/infrastructure/migrations/convert_invite_list_to_history.sql
```

Or through Supabase dashboard:
1. Go to SQL Editor
2. Paste contents of `convert_invite_list_to_history.sql`
3. Run the migration

## Best Practices

1. **Run analyses regularly** (e.g., daily) to build meaningful trends
2. **Use the views** for most queries - they're optimized
3. **Archive old data** periodically to keep table size manageable
4. **Monitor score changes** to catch sudden risk increases
5. **Use date ranges** in queries to improve performance

## Performance Tips

- The table is indexed on `(symbol, analysis_date DESC)`
- Always include a date range in queries when possible
- Use `DISTINCT ON (symbol)` for "latest per stock" queries
- Consider partitioning by month for very large datasets

## Example Dashboard Queries

### Today's Safety Summary

```sql
SELECT 
    COUNT(*) as total_stocks,
    COUNT(CASE WHEN is_safe_to_trade THEN 1 END) as safe_stocks,
    ROUND(AVG(safety_score), 2) as avg_safety_score,
    COUNT(CASE WHEN risk_level = 'Low' THEN 1 END) as low_risk,
    COUNT(CASE WHEN risk_level = 'High' THEN 1 END) as high_risk
FROM invite_list;
```

### Top Safe Stocks

```sql
SELECT 
    symbol,
    company_name,
    sector,
    safety_score,
    risk_level
FROM safe_stocks
LIMIT 10;
```

### Recent Safety Changes (Last 24 Hours)

```sql
SELECT * FROM get_safety_changes(1)
WHERE status_changed = TRUE;
```

