# Timing Study Analysis & Fixes

## Issues Found

### 1. **CRITICAL: Missing `analysis_date` field**
- **Database schema** (`timing_history` table) requires: `analysis_date TIMESTAMPTZ NOT NULL`
- **Rust struct** (`TimingInsert`) does NOT have this field
- **Impact**: Cannot insert records into database

### 2. Schema Alignment Issues
- `TimingRecord` (for reading) has `created_at` and `updated_at` 
- `CreateTimingRecord` (for creating) does NOT have `analysis_date`
- `TimingInsert` (for database insert) does NOT have `analysis_date`

## Required Fixes

### Fix 1: Add `analysis_date` to all timing models

**File: `crates/infrastructure/src/database/timing_models.rs`**

1. Add `analysis_date` field to `CreateTimingRecord`:
   ```rust
   pub struct CreateTimingRecord {
       pub symbol: String,
       pub analysis_date: DateTime<Utc>,  // ADD THIS
       pub tts_score: f64,
       // ... rest of fields
   }
   ```

2. Add `analysis_date` field to `TimingInsert`:
   ```rust
   pub struct TimingInsert {
       pub symbol: String,
       pub analysis_date: DateTime<Utc>,  // ADD THIS
       pub tts_score: f64,
       // ... rest of fields
   }
   ```

3. Update `From<CreateTimingRecord> for TimingInsert` impl:
   ```rust
   impl From<CreateTimingRecord> for TimingInsert {
       fn from(record: CreateTimingRecord) -> Self {
           Self {
               symbol: record.symbol,
               analysis_date: record.analysis_date,  // ADD THIS
               tts_score: record.tts_score,
               // ... rest of fields
           }
       }
   }
   ```

**File: `crates/studies/timing/src/timing_models.rs`**

4. Update `create_timing_record_with_tracking` helper to set `analysis_date`:
   ```rust
   pub fn create_timing_record_with_tracking(
       tts_result: TTSResult,
       api_tracking: TTSApiTracking,
   ) -> CreateTimingRecord {
       CreateTimingRecord {
           symbol: tts_result.symbol,
           analysis_date: tts_result.timestamp,  // ADD THIS - use timestamp from result
           tts_score: tts_result.tts_score,
           // ... rest of fields
       }
   }
   ```

## API Call Verification

### FMP API (Financial Modeling Prep)
- **Endpoint**: `https://financialmodelingprep.com/api/v3/historical-price-full/{symbol}?apikey={key}`
- **Returns**: Historical price data with OHLCV
- **Used for**: All technical indicators calculation
- **Logical**: ✅ YES - provides necessary price data

### Alpha Vantage API (Fallback)
- **Endpoint**: `https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol={symbol}&apikey={key}`
- **Returns**: Daily time series data
- **Used for**: Fallback when FMP fails
- **Logical**: ✅ YES - good redundancy

## Logic Review

### Calculator Logic
1. **Price Data Collection**: ✅ Good - tries FMP first, falls back to Alpha Vantage
2. **Indicator Calculation**: ✅ Good - uses all major technical indicators (RSI, MACD, Bollinger, MA, Stochastic, Williams %R, ATR, Volume)
3. **Scoring System**: ✅ Good - uses -1.0 to +1.0 scale for consistency
4. **Trend Analysis**: ✅ Good - checks short/medium/long term trends
5. **Support/Resistance**: ✅ Good - identifies key levels
6. **Risk Assessment**: ✅ Good - calculates volatility, stop loss, risk-reward

### Storage Logic
1. **Time-Series Approach**: ✅ CORRECT - stores each analysis as new record
2. **Target Table**: ✅ CORRECT - uses `timing_history` table
3. **View Usage**: ✅ CORRECT - queries `timing` view for latest data
4. **Missing Field**: ❌ BUG - `analysis_date` not being set

## Refactoring Recommendations

### 1. Simplify Model Conversion
Current flow is complex:
```
TTSResult + TTSApiTracking 
  → CreateTimingRecord (in timing crate)
  → TimingInsert (in infrastructure crate)
  → Database
```

This is fine, but ensure all fields are properly mapped.

### 2. Add Validation
Add validation to ensure critical fields are present:
- `analysis_date` should default to `Utc::now()` if not provided
- `symbol` should be uppercase
- Scores should be within expected ranges

### 3. Consistent Timestamp Handling
- Use `TTSResult.timestamp` for `analysis_date` 
- Let database auto-generate `created_at` and `updated_at`

## Database Schema is CORRECT

The `timing_history` table in `recreate_all_tables.sql` is properly structured:
- Has all necessary fields
- Uses `analysis_date` for tracking analysis time
- Uses `created_at`/`updated_at` for record audit
- Has proper indexes
- Has unique constraint on `(symbol, analysis_date)`

## Summary

**Primary Issue**: Missing `analysis_date` field in Rust structs prevents data insertion.

**Fix Priority**: HIGH - Must fix before running timing analysis.

**Code Quality**: Overall good - logic is sound, just missing one critical field mapping.

