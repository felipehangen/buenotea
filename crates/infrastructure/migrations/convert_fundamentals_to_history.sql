-- Migration: Convert fundamentals to time-series history approach
-- Created: 2025-11-10
-- Purpose: Enable tracking fundamentals analysis over time for each symbol

-- Step 1: Drop existing functions (they might depend on tables/views)
DROP FUNCTION IF EXISTS get_fundamentals_at_date(VARCHAR, TIMESTAMPTZ);
DROP FUNCTION IF EXISTS get_fundamentals_history(VARCHAR, INTEGER);
DROP FUNCTION IF EXISTS get_fundamentals_changes(VARCHAR, INTEGER);

-- Step 2: Drop existing views (they depend on the tables)
DROP VIEW IF EXISTS latest_fundamentals CASCADE;
DROP VIEW IF EXISTS fundamentals CASCADE; -- This might be a view if already migrated

-- Step 3: Drop and recreate fundamentals_history table
-- We drop it to ensure clean state (like we did with regime)
DROP TABLE IF EXISTS fundamentals_history CASCADE;

-- Step 4: Create fundamentals_history table with complete schema
CREATE TABLE fundamentals_history (
    id BIGSERIAL PRIMARY KEY,
    symbol VARCHAR(10) NOT NULL,
    analysis_date TIMESTAMPTZ NOT NULL,
    
    -- Core scoring data
    fundamentals_score DECIMAL(5,2) NOT NULL,
    trading_signal VARCHAR(20) NOT NULL,
    confidence_score DECIMAL(3,2) NOT NULL,
    
    -- Component scores (0-100)
    profitability_score DECIMAL(5,2) NOT NULL,
    growth_score DECIMAL(5,2) NOT NULL,
    valuation_score DECIMAL(5,2) NOT NULL,
    financial_strength_score DECIMAL(5,2) NOT NULL,
    efficiency_score DECIMAL(5,2) NOT NULL,
    
    -- Profitability metrics
    roe DECIMAL(8,4),
    roa DECIMAL(8,4),
    roic DECIMAL(8,4),
    net_profit_margin DECIMAL(8,4),
    gross_profit_margin DECIMAL(8,4),
    operating_profit_margin DECIMAL(8,4),
    ebitda_margin DECIMAL(8,4),
    
    -- Growth metrics
    revenue_growth_yoy DECIMAL(8,4),
    revenue_growth_qoq DECIMAL(8,4),
    eps_growth_yoy DECIMAL(8,4),
    eps_growth_qoq DECIMAL(8,4),
    net_income_growth_yoy DECIMAL(8,4),
    book_value_growth_yoy DECIMAL(8,4),
    operating_cash_flow_growth_yoy DECIMAL(8,4),
    
    -- Valuation metrics
    pe_ratio DECIMAL(8,2),
    peg_ratio DECIMAL(8,2),
    ps_ratio DECIMAL(8,2),
    pb_ratio DECIMAL(8,2),
    pcf_ratio DECIMAL(8,2),
    ev_ebitda DECIMAL(8,2),
    ev_sales DECIMAL(8,2),
    pfcf_ratio DECIMAL(8,2),
    
    -- Financial strength metrics
    debt_to_equity DECIMAL(8,4),
    debt_to_assets DECIMAL(8,4),
    current_ratio DECIMAL(8,2),
    quick_ratio DECIMAL(8,2),
    interest_coverage DECIMAL(8,2),
    cash_to_debt DECIMAL(8,2),
    equity_multiplier DECIMAL(8,2),
    altman_z_score DECIMAL(8,2),
    
    -- Efficiency metrics
    asset_turnover DECIMAL(8,2),
    inventory_turnover DECIMAL(8,2),
    receivables_turnover DECIMAL(8,2),
    payables_turnover DECIMAL(8,2),
    working_capital_turnover DECIMAL(8,2),
    days_sales_outstanding DECIMAL(8,1),
    days_inventory_outstanding DECIMAL(8,1),
    days_payables_outstanding DECIMAL(8,1),
    
    -- Company metadata
    sector VARCHAR(100),
    industry VARCHAR(100),
    market_cap_category VARCHAR(50),
    beta DECIMAL(6,3),
    dividend_yield DECIMAL(6,4),
    payout_ratio DECIMAL(6,4),
    shares_outstanding BIGINT,
    market_cap BIGINT,
    enterprise_value BIGINT,
    
    -- Analysis metadata
    computation_time_ms INTEGER,
    data_points_count INTEGER,
    data_freshness DECIMAL(3,2),
    flags TEXT[],
    
    -- API tracking
    profitability_api_url TEXT,
    profitability_api_source VARCHAR(50),
    profitability_data_available BOOLEAN DEFAULT FALSE,
    profitability_raw_data JSONB,
    
    growth_api_url TEXT,
    growth_api_source VARCHAR(50),
    growth_data_available BOOLEAN DEFAULT FALSE,
    growth_raw_data JSONB,
    
    valuation_api_url TEXT,
    valuation_api_source VARCHAR(50),
    valuation_data_available BOOLEAN DEFAULT FALSE,
    valuation_raw_data JSONB,
    
    financial_strength_api_url TEXT,
    financial_strength_api_source VARCHAR(50),
    financial_strength_data_available BOOLEAN DEFAULT FALSE,
    financial_strength_raw_data JSONB,
    
    efficiency_api_url TEXT,
    efficiency_api_source VARCHAR(50),
    efficiency_data_available BOOLEAN DEFAULT FALSE,
    efficiency_raw_data JSONB,
    
    -- AI Analysis
    gpt_explanation TEXT,
    gpt_trading_suggestion TEXT,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Unique constraint to prevent duplicate analyses for same symbol on same date
    CONSTRAINT unique_fundamentals_symbol_date UNIQUE (symbol, analysis_date)
);

-- Step 5: Create indexes for efficient querying
CREATE INDEX IF NOT EXISTS idx_fundamentals_history_symbol ON fundamentals_history(symbol);
CREATE INDEX IF NOT EXISTS idx_fundamentals_history_date ON fundamentals_history(analysis_date DESC);
CREATE INDEX IF NOT EXISTS idx_fundamentals_history_symbol_date ON fundamentals_history(symbol, analysis_date DESC);
CREATE INDEX IF NOT EXISTS idx_fundamentals_history_score ON fundamentals_history(fundamentals_score);
CREATE INDEX IF NOT EXISTS idx_fundamentals_history_signal ON fundamentals_history(trading_signal);
CREATE INDEX IF NOT EXISTS idx_fundamentals_history_sector ON fundamentals_history(sector);
CREATE INDEX IF NOT EXISTS idx_fundamentals_history_created ON fundamentals_history(created_at DESC);

-- Step 6: Create view for latest fundamentals per symbol
CREATE OR REPLACE VIEW fundamentals AS
SELECT DISTINCT ON (symbol)
    id,
    symbol,
    analysis_date,
    fundamentals_score,
    trading_signal,
    confidence_score,
    profitability_score,
    growth_score,
    valuation_score,
    financial_strength_score,
    efficiency_score,
    roe,
    roa,
    roic,
    net_profit_margin,
    gross_profit_margin,
    operating_profit_margin,
    ebitda_margin,
    revenue_growth_yoy,
    revenue_growth_qoq,
    eps_growth_yoy,
    eps_growth_qoq,
    net_income_growth_yoy,
    book_value_growth_yoy,
    operating_cash_flow_growth_yoy,
    pe_ratio,
    peg_ratio,
    ps_ratio,
    pb_ratio,
    pcf_ratio,
    ev_ebitda,
    ev_sales,
    pfcf_ratio,
    debt_to_equity,
    debt_to_assets,
    current_ratio,
    quick_ratio,
    interest_coverage,
    cash_to_debt,
    equity_multiplier,
    altman_z_score,
    asset_turnover,
    inventory_turnover,
    receivables_turnover,
    payables_turnover,
    working_capital_turnover,
    days_sales_outstanding,
    days_inventory_outstanding,
    days_payables_outstanding,
    sector,
    industry,
    market_cap_category,
    beta,
    dividend_yield,
    payout_ratio,
    shares_outstanding,
    market_cap,
    enterprise_value,
    computation_time_ms,
    data_points_count,
    data_freshness,
    flags,
    profitability_api_url,
    profitability_api_source,
    profitability_data_available,
    profitability_raw_data,
    growth_api_url,
    growth_api_source,
    growth_data_available,
    growth_raw_data,
    valuation_api_url,
    valuation_api_source,
    valuation_data_available,
    valuation_raw_data,
    financial_strength_api_url,
    financial_strength_api_source,
    financial_strength_data_available,
    financial_strength_raw_data,
    efficiency_api_url,
    efficiency_api_source,
    efficiency_data_available,
    efficiency_raw_data,
    gpt_explanation,
    gpt_trading_suggestion,
    created_at,
    updated_at
FROM fundamentals_history
ORDER BY symbol, analysis_date DESC, created_at DESC;

-- Step 7: Create function to get fundamentals at a specific date
CREATE OR REPLACE FUNCTION get_fundamentals_at_date(target_symbol VARCHAR, target_date TIMESTAMPTZ)
RETURNS TABLE (
    symbol VARCHAR,
    analysis_date TIMESTAMPTZ,
    fundamentals_score DECIMAL,
    trading_signal VARCHAR,
    confidence_score DECIMAL,
    created_at TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        h.symbol,
        h.analysis_date,
        h.fundamentals_score,
        h.trading_signal,
        h.confidence_score,
        h.created_at
    FROM fundamentals_history h
    WHERE h.symbol = target_symbol
      AND h.analysis_date <= target_date
    ORDER BY h.analysis_date DESC, h.created_at DESC
    LIMIT 1;
END;
$$ LANGUAGE plpgsql;

-- Step 8: Create function to get fundamentals history for a symbol
CREATE OR REPLACE FUNCTION get_fundamentals_history(target_symbol VARCHAR, days_back INTEGER DEFAULT 365)
RETURNS TABLE (
    id BIGINT,
    symbol VARCHAR,
    analysis_date TIMESTAMPTZ,
    fundamentals_score DECIMAL,
    trading_signal VARCHAR,
    confidence_score DECIMAL,
    profitability_score DECIMAL,
    growth_score DECIMAL,
    valuation_score DECIMAL,
    financial_strength_score DECIMAL,
    efficiency_score DECIMAL,
    sector VARCHAR,
    industry VARCHAR,
    market_cap BIGINT,
    beta DECIMAL,
    dividend_yield DECIMAL,
    created_at TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        h.id,
        h.symbol,
        h.analysis_date,
        h.fundamentals_score,
        h.trading_signal,
        h.confidence_score,
        h.profitability_score,
        h.growth_score,
        h.valuation_score,
        h.financial_strength_score,
        h.efficiency_score,
        h.sector,
        h.industry,
        h.market_cap,
        h.beta,
        h.dividend_yield,
        h.created_at
    FROM fundamentals_history h
    WHERE h.symbol = target_symbol
      AND h.analysis_date >= NOW() - (days_back || ' days')::INTERVAL
    ORDER BY h.analysis_date DESC, h.created_at DESC;
END;
$$ LANGUAGE plpgsql;

-- Step 9: Create function to detect significant fundamentals score changes
CREATE OR REPLACE FUNCTION get_fundamentals_changes(target_symbol VARCHAR, days_back INTEGER DEFAULT 90)
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
    WITH ranked_fundamentals AS (
        SELECT
            h.symbol,
            h.fundamentals_score,
            h.trading_signal,
            h.analysis_date,
            LAG(h.fundamentals_score) OVER (ORDER BY h.analysis_date, h.created_at) as prev_score,
            LAG(h.trading_signal) OVER (ORDER BY h.analysis_date, h.created_at) as prev_signal
        FROM fundamentals_history h
        WHERE h.symbol = target_symbol
          AND h.analysis_date >= NOW() - (days_back || ' days')::INTERVAL
    )
    SELECT
        rf.symbol,
        rf.prev_score as old_score,
        rf.fundamentals_score as new_score,
        (rf.fundamentals_score - rf.prev_score) as score_change,
        rf.prev_signal as old_signal,
        rf.trading_signal as new_signal,
        rf.analysis_date as change_date
    FROM ranked_fundamentals rf
    WHERE rf.prev_score IS NOT NULL
      AND ABS(rf.fundamentals_score - rf.prev_score) >= 5.0 -- Only show changes >= 5 points
    ORDER BY rf.analysis_date DESC;
END;
$$ LANGUAGE plpgsql;

-- Step 10: Add helpful comments
COMMENT ON TABLE fundamentals_history IS 'Time-series storage of fundamentals analysis for each symbol. Enables tracking changes over time.';
COMMENT ON VIEW fundamentals IS 'View showing the latest fundamentals analysis for each symbol. Query this view for current analysis.';
COMMENT ON FUNCTION get_fundamentals_at_date IS 'Returns the fundamentals analysis that was valid for a specific symbol at a specific date in the past';
COMMENT ON FUNCTION get_fundamentals_history IS 'Returns all fundamentals analyses for a symbol over the past N days';
COMMENT ON FUNCTION get_fundamentals_changes IS 'Detects significant fundamentals score changes (>= 5 points) for a symbol';

-- Migration complete!
-- Usage examples:
-- 1. Get latest fundamentals for AAPL: SELECT * FROM fundamentals WHERE symbol = 'AAPL';
-- 2. Get fundamentals 30 days ago: SELECT * FROM get_fundamentals_at_date('AAPL', NOW() - INTERVAL '30 days');
-- 3. Get fundamentals history: SELECT * FROM get_fundamentals_history('AAPL', 365);
-- 4. Find score changes: SELECT * FROM get_fundamentals_changes('AAPL', 90);

