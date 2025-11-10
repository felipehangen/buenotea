-- Recreate market_regime_history table from scratch
-- This drops everything and recreates with the correct structure

-- Step 1: Drop existing functions first (they might depend on tables/views)
DROP FUNCTION IF EXISTS get_market_regime_at_date(TIMESTAMPTZ);
DROP FUNCTION IF EXISTS get_market_regime_history(INTEGER);
DROP FUNCTION IF EXISTS get_market_regime_changes(INTEGER);

-- Step 2: Drop existing views (they depend on the tables)
DROP VIEW IF EXISTS latest_market_regime CASCADE;
DROP VIEW IF EXISTS market_regime_history_view CASCADE;

-- Step 3: Drop existing tables (check both possibilities)
-- market_regime might be either a table or a view, so try both
DO $$
BEGIN
    -- Try to drop as view first
    BEGIN
        EXECUTE 'DROP VIEW IF EXISTS market_regime CASCADE';
    EXCEPTION WHEN OTHERS THEN
        NULL; -- Ignore error if it's not a view
    END;
    
    -- Then try to drop as table
    BEGIN
        EXECUTE 'DROP TABLE IF EXISTS market_regime CASCADE';
    EXCEPTION WHEN OTHERS THEN
        NULL; -- Ignore error if it's not a table
    END;
END $$;

DROP TABLE IF EXISTS market_regime_history CASCADE;

-- Step 4: Create market_regime_history table with ALL required columns
CREATE TABLE market_regime_history (
    id SERIAL PRIMARY KEY,
    analysis_date TIMESTAMP WITH TIME ZONE NOT NULL,
    
    -- Market Regime Classification
    market_regime VARCHAR(20) NOT NULL,
    regime_confidence DECIMAL(5,4) NOT NULL,
    
    -- Market Context Data
    spy_price DECIMAL(10,2),
    spy_20d_change DECIMAL(8,6),
    spy_50d_change DECIMAL(8,6),
    vix DECIMAL(6,2),
    market_breadth DECIMAL(5,4),
    sector_relative_performance DECIMAL(8,6),
    
    -- Market Volatility Analysis
    market_volatility DECIMAL(8,6),
    volatility_percentile DECIMAL(5,2),
    
    -- Market Trend Analysis
    short_term_trend VARCHAR(20),
    medium_term_trend VARCHAR(20),
    long_term_trend VARCHAR(20),
    trend_strength DECIMAL(5,2),
    trend_consistency DECIMAL(5,2),
    
    -- Market Breadth Analysis
    advancing_stocks INTEGER,
    declining_stocks INTEGER,
    unchanged_stocks INTEGER,
    new_highs INTEGER,
    new_lows INTEGER,
    
    -- Sector Analysis
    technology_performance DECIMAL(8,6),
    healthcare_performance DECIMAL(8,6),
    financial_performance DECIMAL(8,6),
    energy_performance DECIMAL(8,6),
    consumer_performance DECIMAL(8,6),
    
    -- Market Sentiment Indicators
    fear_greed_index INTEGER,
    put_call_ratio DECIMAL(8,4),
    margin_debt_trend VARCHAR(20),
    
    -- Risk Assessment
    market_risk_level VARCHAR(20),
    market_risk_score DECIMAL(5,2),
    max_drawdown_risk DECIMAL(5,2),
    
    -- AI Analysis (Optional)
    chatgpt_regime_analysis TEXT,
    chatgpt_market_outlook TEXT,
    chatgpt_risk_assessment TEXT,
    chatgpt_model_used VARCHAR(50),
    chatgpt_analysis_timestamp TIMESTAMP WITH TIME ZONE,
    
    -- Analysis Metadata
    data_sources_used TEXT[],
    analysis_period_days INTEGER DEFAULT 250,
    computation_time_ms INTEGER,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Step 5: Add unique constraint for time-series (prevent duplicate timestamps)
ALTER TABLE market_regime_history 
    ADD CONSTRAINT market_regime_history_date_created_key 
    UNIQUE(analysis_date, created_at);

-- Step 6: Create indexes for performance
CREATE INDEX idx_market_regime_history_date_created ON market_regime_history(analysis_date DESC, created_at DESC);
CREATE INDEX idx_market_regime_history_regime_date ON market_regime_history(market_regime, analysis_date DESC);

-- Step 7: Create view for latest market regime (backwards compatibility)
CREATE OR REPLACE VIEW market_regime AS
SELECT *
FROM market_regime_history
ORDER BY analysis_date DESC, created_at DESC
LIMIT 1;

-- Step 8: Create view for latest_market_regime (backwards compatibility)
CREATE OR REPLACE VIEW latest_market_regime AS
SELECT *
FROM market_regime_history
ORDER BY analysis_date DESC, created_at DESC
LIMIT 1;

-- Step 9: Create view for market_regime_history_view (with percentages)
CREATE OR REPLACE VIEW market_regime_history_view AS
SELECT 
    analysis_date,
    market_regime,
    regime_confidence,
    spy_price,
    COALESCE(spy_20d_change * 100, 0) as spy_20d_change_pct,
    COALESCE(spy_50d_change * 100, 0) as spy_50d_change_pct,
    vix,
    COALESCE(market_breadth * 100, 0) as market_breadth_pct,
    COALESCE(market_volatility * 100, 0) as market_volatility_pct,
    trend_strength,
    trend_consistency,
    market_risk_level,
    market_risk_score,
    created_at
FROM market_regime_history 
ORDER BY analysis_date DESC;

-- Step 10: Create helper function to get market regime at a specific date
CREATE OR REPLACE FUNCTION get_market_regime_at_date(target_date TIMESTAMPTZ)
RETURNS TABLE (
    market_regime VARCHAR,
    regime_confidence DECIMAL,
    market_risk_level VARCHAR,
    analysis_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        h.market_regime,
        h.regime_confidence,
        h.market_risk_level,
        h.analysis_date,
        h.created_at
    FROM market_regime_history h
    WHERE h.analysis_date <= target_date
    ORDER BY h.analysis_date DESC, h.created_at DESC
    LIMIT 1;
END;
$$ LANGUAGE plpgsql;

-- Step 11: Create helper function to get market regime history
CREATE OR REPLACE FUNCTION get_market_regime_history(days_back INTEGER DEFAULT 90)
RETURNS TABLE (
    market_regime VARCHAR,
    regime_confidence DECIMAL,
    spy_price DECIMAL,
    spy_20d_change DECIMAL,
    spy_50d_change DECIMAL,
    vix DECIMAL,
    market_volatility DECIMAL,
    short_term_trend VARCHAR,
    medium_term_trend VARCHAR,
    long_term_trend VARCHAR,
    trend_strength DECIMAL,
    market_risk_level VARCHAR,
    market_risk_score DECIMAL,
    analysis_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        h.market_regime,
        h.regime_confidence,
        h.spy_price,
        h.spy_20d_change,
        h.spy_50d_change,
        h.vix,
        h.market_volatility,
        h.short_term_trend,
        h.medium_term_trend,
        h.long_term_trend,
        h.trend_strength,
        h.market_risk_level,
        h.market_risk_score,
        h.analysis_date,
        h.created_at
    FROM market_regime_history h
    WHERE h.analysis_date >= NOW() - (days_back || ' days')::INTERVAL
    ORDER BY h.analysis_date DESC, h.created_at DESC;
END;
$$ LANGUAGE plpgsql;

-- Step 12: Create helper function to detect regime changes
CREATE OR REPLACE FUNCTION get_market_regime_changes(days_back INTEGER DEFAULT 30)
RETURNS TABLE (
    old_regime VARCHAR,
    new_regime VARCHAR,
    old_confidence DECIMAL,
    new_confidence DECIMAL,
    change_date TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    WITH ranked_regimes AS (
        SELECT 
            h.market_regime,
            h.regime_confidence,
            h.analysis_date,
            LAG(h.market_regime) OVER (ORDER BY h.analysis_date, h.created_at) as prev_regime,
            LAG(h.regime_confidence) OVER (ORDER BY h.analysis_date, h.created_at) as prev_confidence
        FROM market_regime_history h
        WHERE h.analysis_date >= NOW() - (days_back || ' days')::INTERVAL
    )
    SELECT 
        rr.prev_regime as old_regime,
        rr.market_regime as new_regime,
        rr.prev_confidence as old_confidence,
        rr.regime_confidence as new_confidence,
        rr.analysis_date as change_date
    FROM ranked_regimes rr
    WHERE rr.prev_regime IS NOT NULL 
      AND rr.prev_regime != rr.market_regime
    ORDER BY rr.analysis_date DESC;
END;
$$ LANGUAGE plpgsql;

-- Step 13: Add helpful comments
COMMENT ON TABLE market_regime_history IS 'Time-series history of market regime analysis - tracks regime changes over time';
COMMENT ON VIEW market_regime IS 'View showing the latest market regime analysis';
COMMENT ON VIEW latest_market_regime IS 'View showing the latest market regime analysis';
COMMENT ON FUNCTION get_market_regime_at_date IS 'Returns the market regime that was valid at a specific date';
COMMENT ON FUNCTION get_market_regime_history IS 'Returns all market regime analyses over the past N days';
COMMENT ON FUNCTION get_market_regime_changes IS 'Detects when the market regime changed (e.g., Bull → Bear)';

-- Complete!
SELECT '✅ market_regime_history table created successfully!' as status;
SELECT 'Ready to store market regime data' as next_step;

-- Usage examples:
-- 1. Insert a record: INSERT INTO market_regime_history (analysis_date, market_regime, regime_confidence, ...) VALUES (...);
-- 2. Get latest regime: SELECT * FROM market_regime;
-- 3. Get history: SELECT * FROM get_market_regime_history(90);
-- 4. Get regime changes: SELECT * FROM get_market_regime_changes(30);

