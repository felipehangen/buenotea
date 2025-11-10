-- Migration: Convert sentiment to time-series history approach
-- Created: 2025-11-10
-- Purpose: Enable tracking sentiment (QSS) analysis over time for each symbol

-- Step 1: Drop existing functions (they might depend on tables/views)
DROP FUNCTION IF EXISTS get_sentiment_at_date(VARCHAR, TIMESTAMPTZ);
DROP FUNCTION IF EXISTS get_sentiment_history(VARCHAR, INTEGER);
DROP FUNCTION IF EXISTS get_sentiment_changes(VARCHAR, INTEGER);

-- Step 2: Drop existing views (they depend on the tables)
DROP VIEW IF EXISTS latest_sentiment CASCADE;
DROP VIEW IF EXISTS sentiment CASCADE; -- This might be a view if already migrated

-- Step 3: Drop and recreate sentiment_history table
DROP TABLE IF EXISTS sentiment_history CASCADE;

-- Step 4: Create sentiment_history table with complete schema
CREATE TABLE sentiment_history (
    id BIGSERIAL PRIMARY KEY,
    symbol VARCHAR(10) NOT NULL,
    analysis_date TIMESTAMPTZ NOT NULL,
    
    -- Core QSS scoring data
    qss_score DECIMAL(5,3) NOT NULL, -- Range: -1.000 to +1.000
    trading_signal VARCHAR(20) NOT NULL, -- StrongBuy, WeakBuy, Hold, WeakSell, StrongSell
    confidence_score DECIMAL(3,2) NOT NULL, -- Range: 0.00 to 1.00
    
    -- Component scores (weighted components of QSS)
    earnings_revisions_score DECIMAL(5,3) NOT NULL,
    relative_strength_score DECIMAL(5,3) NOT NULL,
    short_interest_score DECIMAL(5,3) NOT NULL,
    options_flow_score DECIMAL(5,3) NOT NULL,
    
    -- Component weights (for reference)
    earnings_weight DECIMAL(3,2) NOT NULL DEFAULT 0.40,
    relative_strength_weight DECIMAL(3,2) NOT NULL DEFAULT 0.30,
    short_interest_weight DECIMAL(3,2) NOT NULL DEFAULT 0.20,
    options_flow_weight DECIMAL(3,2) NOT NULL DEFAULT 0.10,
    
    -- API endpoint information
    earnings_api_url TEXT,
    earnings_api_source VARCHAR(50),
    earnings_data_available BOOLEAN DEFAULT FALSE,
    
    price_data_api_url TEXT,
    price_data_api_source VARCHAR(50),
    price_data_available BOOLEAN DEFAULT FALSE,
    
    short_interest_api_url TEXT,
    short_interest_api_source VARCHAR(50),
    short_interest_data_available BOOLEAN DEFAULT FALSE,
    
    options_flow_api_url TEXT,
    options_flow_api_source VARCHAR(50),
    options_flow_data_available BOOLEAN DEFAULT FALSE,
    
    -- Raw API response data (JSON)
    earnings_raw_data JSONB,
    price_data_raw_data JSONB,
    short_interest_raw_data JSONB,
    options_flow_raw_data JSONB,
    
    -- Data quality metrics
    data_coverage_percentage DECIMAL(5,2) NOT NULL,
    computation_time_ms INTEGER NOT NULL,
    data_points_count INTEGER NOT NULL,
    trend_direction DECIMAL(5,3) NOT NULL,
    data_freshness_score DECIMAL(3,2) NOT NULL,
    
    -- Warning flags and context
    warning_flags TEXT[],
    missing_data_components TEXT[],
    
    -- GPT-generated explanation
    gpt_explanation TEXT NOT NULL,
    gpt_explanation_timestamp TIMESTAMPTZ,
    
    -- Technical indicators
    rsi_14 DECIMAL(8,2),
    rsi_source VARCHAR(50),
    
    -- Market context
    market_benchmark_return DECIMAL(8,4),
    sector_benchmark_return DECIMAL(8,4),
    relative_to_market DECIMAL(8,4),
    relative_to_sector DECIMAL(8,4),
    
    -- Earnings data
    current_eps_estimate DECIMAL(10,2),
    previous_eps_estimate DECIMAL(10,2),
    eps_change_percentage DECIMAL(8,4),
    current_revenue_estimate BIGINT,
    previous_revenue_estimate BIGINT,
    revenue_change_percentage DECIMAL(8,4),
    analyst_count INTEGER,
    
    -- Price data
    current_price DECIMAL(10,2),
    price_15d_ago DECIMAL(10,2),
    price_30d_ago DECIMAL(10,2),
    return_15d DECIMAL(8,4),
    return_30d DECIMAL(8,4),
    volume_ratio DECIMAL(8,4),
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Unique constraint to prevent duplicate analyses for same symbol on same date
    CONSTRAINT unique_sentiment_symbol_date UNIQUE (symbol, analysis_date)
);

-- Step 5: Create indexes for efficient querying
CREATE INDEX IF NOT EXISTS idx_sentiment_history_symbol ON sentiment_history(symbol);
CREATE INDEX IF NOT EXISTS idx_sentiment_history_date ON sentiment_history(analysis_date DESC);
CREATE INDEX IF NOT EXISTS idx_sentiment_history_symbol_date ON sentiment_history(symbol, analysis_date DESC);
CREATE INDEX IF NOT EXISTS idx_sentiment_history_score ON sentiment_history(qss_score);
CREATE INDEX IF NOT EXISTS idx_sentiment_history_signal ON sentiment_history(trading_signal);
CREATE INDEX IF NOT EXISTS idx_sentiment_history_created ON sentiment_history(created_at DESC);

-- Step 6: Create view for latest sentiment per symbol
CREATE OR REPLACE VIEW sentiment AS
SELECT DISTINCT ON (symbol)
    id,
    symbol,
    analysis_date,
    qss_score,
    trading_signal,
    confidence_score,
    earnings_revisions_score,
    relative_strength_score,
    short_interest_score,
    options_flow_score,
    earnings_weight,
    relative_strength_weight,
    short_interest_weight,
    options_flow_weight,
    earnings_api_url,
    earnings_api_source,
    earnings_data_available,
    price_data_api_url,
    price_data_api_source,
    price_data_available,
    short_interest_api_url,
    short_interest_api_source,
    short_interest_data_available,
    options_flow_api_url,
    options_flow_api_source,
    options_flow_data_available,
    earnings_raw_data,
    price_data_raw_data,
    short_interest_raw_data,
    options_flow_raw_data,
    data_coverage_percentage,
    computation_time_ms,
    data_points_count,
    trend_direction,
    data_freshness_score,
    warning_flags,
    missing_data_components,
    gpt_explanation,
    gpt_explanation_timestamp,
    rsi_14,
    rsi_source,
    market_benchmark_return,
    sector_benchmark_return,
    relative_to_market,
    relative_to_sector,
    current_eps_estimate,
    previous_eps_estimate,
    eps_change_percentage,
    current_revenue_estimate,
    previous_revenue_estimate,
    revenue_change_percentage,
    analyst_count,
    current_price,
    price_15d_ago,
    price_30d_ago,
    return_15d,
    return_30d,
    volume_ratio,
    created_at,
    updated_at
FROM sentiment_history
ORDER BY symbol, analysis_date DESC, created_at DESC;

-- Step 7: Create function to get sentiment at a specific date
CREATE OR REPLACE FUNCTION get_sentiment_at_date(target_symbol VARCHAR, target_date TIMESTAMPTZ)
RETURNS TABLE (
    symbol VARCHAR,
    analysis_date TIMESTAMPTZ,
    qss_score DECIMAL,
    trading_signal VARCHAR,
    confidence_score DECIMAL,
    created_at TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        h.symbol,
        h.analysis_date,
        h.qss_score,
        h.trading_signal,
        h.confidence_score,
        h.created_at
    FROM sentiment_history h
    WHERE h.symbol = target_symbol
      AND h.analysis_date <= target_date
    ORDER BY h.analysis_date DESC, h.created_at DESC
    LIMIT 1;
END;
$$ LANGUAGE plpgsql;

-- Step 8: Create function to get sentiment history for a symbol
CREATE OR REPLACE FUNCTION get_sentiment_history(target_symbol VARCHAR, days_back INTEGER DEFAULT 90)
RETURNS TABLE (
    id BIGINT,
    symbol VARCHAR,
    analysis_date TIMESTAMPTZ,
    qss_score DECIMAL,
    trading_signal VARCHAR,
    confidence_score DECIMAL,
    earnings_revisions_score DECIMAL,
    relative_strength_score DECIMAL,
    short_interest_score DECIMAL,
    options_flow_score DECIMAL,
    data_coverage_percentage DECIMAL,
    gpt_explanation TEXT,
    created_at TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        h.id,
        h.symbol,
        h.analysis_date,
        h.qss_score,
        h.trading_signal,
        h.confidence_score,
        h.earnings_revisions_score,
        h.relative_strength_score,
        h.short_interest_score,
        h.options_flow_score,
        h.data_coverage_percentage,
        h.gpt_explanation,
        h.created_at
    FROM sentiment_history h
    WHERE h.symbol = target_symbol
      AND h.analysis_date >= NOW() - (days_back || ' days')::INTERVAL
    ORDER BY h.analysis_date DESC, h.created_at DESC;
END;
$$ LANGUAGE plpgsql;

-- Step 9: Create function to detect significant sentiment score changes
CREATE OR REPLACE FUNCTION get_sentiment_changes(target_symbol VARCHAR, days_back INTEGER DEFAULT 30)
RETURNS TABLE (
    symbol VARCHAR,
    old_score DECIMAL,
    new_score DECIMAL,
    score_change DECIMAL,
    old_signal VARCHAR,
    new_signal VARCHAR,
    change_date TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    WITH ranked_sentiment AS (
        SELECT
            h.symbol,
            h.qss_score,
            h.trading_signal,
            h.analysis_date,
            LAG(h.qss_score) OVER (ORDER BY h.analysis_date, h.created_at) as prev_score,
            LAG(h.trading_signal) OVER (ORDER BY h.analysis_date, h.created_at) as prev_signal
        FROM sentiment_history h
        WHERE h.symbol = target_symbol
          AND h.analysis_date >= NOW() - (days_back || ' days')::INTERVAL
    )
    SELECT
        rs.symbol,
        rs.prev_score as old_score,
        rs.qss_score as new_score,
        (rs.qss_score - rs.prev_score) as score_change,
        rs.prev_signal as old_signal,
        rs.trading_signal as new_signal,
        rs.analysis_date as change_date
    FROM ranked_sentiment rs
    WHERE rs.prev_score IS NOT NULL
      AND ABS(rs.qss_score - rs.prev_score) >= 0.10 -- Only show changes >= 0.10
    ORDER BY rs.analysis_date DESC;
END;
$$ LANGUAGE plpgsql;

-- Step 10: Add helpful comments
COMMENT ON TABLE sentiment_history IS 'Time-series storage of sentiment (QSS) analysis for each symbol. Enables tracking changes over time.';
COMMENT ON VIEW sentiment IS 'View showing the latest sentiment analysis for each symbol. Query this view for current analysis.';
COMMENT ON FUNCTION get_sentiment_at_date IS 'Returns the sentiment analysis that was valid for a specific symbol at a specific date in the past';
COMMENT ON FUNCTION get_sentiment_history IS 'Returns all sentiment analyses for a symbol over the past N days';
COMMENT ON FUNCTION get_sentiment_changes IS 'Detects significant sentiment score changes (>= 0.10) for a symbol';

-- Migration complete!
-- Usage examples:
-- 1. Get latest sentiment for AAPL: SELECT * FROM sentiment WHERE symbol = 'AAPL';
-- 2. Get sentiment 30 days ago: SELECT * FROM get_sentiment_at_date('AAPL', NOW() - INTERVAL '30 days');
-- 3. Get sentiment history: SELECT * FROM get_sentiment_history('AAPL', 90);
-- 4. Find score changes: SELECT * FROM get_sentiment_changes('AAPL', 30);

