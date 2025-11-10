-- Migration: Convert invite_list to time-series table for historical tracking
-- Created: 2025-01-08
-- Purpose: Enable tracking of safety analysis changes over time

-- Step 1: Rename current table to history table
ALTER TABLE IF EXISTS invite_list 
    RENAME TO invite_list_history;

-- Step 2: Drop UNIQUE constraint on symbol (allow multiple records per symbol)
ALTER TABLE invite_list_history 
    DROP CONSTRAINT IF EXISTS invite_list_symbol_key;

-- Step 3: Add composite unique constraint (symbol + analysis_date)
-- This prevents duplicate entries for the same stock on the same date
ALTER TABLE invite_list_history 
    ADD CONSTRAINT invite_list_history_symbol_date_key 
    UNIQUE(symbol, analysis_date);

-- Step 4: Update indexes for time-series queries
DROP INDEX IF EXISTS idx_invite_list_symbol;
CREATE INDEX IF NOT EXISTS idx_invite_history_symbol_date 
    ON invite_list_history(symbol, analysis_date DESC);

CREATE INDEX IF NOT EXISTS idx_invite_history_analysis_date 
    ON invite_list_history(analysis_date DESC);

-- Keep other indexes with updated names
DROP INDEX IF EXISTS idx_invite_list_safe_to_trade;
CREATE INDEX IF NOT EXISTS idx_invite_history_safe_to_trade 
    ON invite_list_history(is_safe_to_trade);

DROP INDEX IF EXISTS idx_invite_list_safety_score;
CREATE INDEX IF NOT EXISTS idx_invite_history_safety_score 
    ON invite_list_history(safety_score);

DROP INDEX IF EXISTS idx_invite_list_sector;
CREATE INDEX IF NOT EXISTS idx_invite_history_sector 
    ON invite_list_history(sector);

DROP INDEX IF EXISTS idx_invite_list_risk_level;
CREATE INDEX IF NOT EXISTS idx_invite_history_risk_level 
    ON invite_list_history(risk_level);

-- Step 5: Create view for latest data per stock (backwards compatibility)
CREATE OR REPLACE VIEW invite_list AS
SELECT DISTINCT ON (symbol) 
    id,
    symbol,
    company_name,
    sector,
    industry,
    market_cap,
    current_price,
    is_safe_to_trade,
    safety_score,
    safety_reasoning,
    has_recent_earnings,
    has_positive_revenue,
    has_stable_price,
    has_sufficient_volume,
    has_analyst_coverage,
    risk_level,
    volatility_rating,
    liquidity_rating,
    data_source,
    last_updated,
    data_freshness_score,
    analysis_date,
    analysis_duration_ms,
    warning_flags,
    missing_data_components,
    raw_company_data,
    raw_financial_data,
    raw_price_data,
    created_at,
    updated_at
FROM invite_list_history
ORDER BY symbol, analysis_date DESC;

COMMENT ON VIEW invite_list IS 'View showing the most recent analysis for each stock';

-- Step 6: Update safe_stocks view to use history table
DROP VIEW IF EXISTS safe_stocks;
CREATE OR REPLACE VIEW safe_stocks AS
SELECT 
    symbol,
    company_name,
    sector,
    industry,
    current_price,
    safety_score,
    safety_reasoning,
    risk_level,
    volatility_rating,
    liquidity_rating,
    last_updated,
    analysis_date
FROM invite_list
WHERE is_safe_to_trade = TRUE
ORDER BY safety_score DESC, symbol;

COMMENT ON VIEW safe_stocks IS 'View of stocks that are currently safe to trade, ordered by safety score';

-- Step 7: Update sector_safety_analysis view
DROP VIEW IF EXISTS sector_safety_analysis;
CREATE OR REPLACE VIEW sector_safety_analysis AS
SELECT 
    sector,
    COUNT(*) as total_stocks,
    COUNT(CASE WHEN is_safe_to_trade = TRUE THEN 1 END) as safe_stocks,
    ROUND(
        COUNT(CASE WHEN is_safe_to_trade = TRUE THEN 1 END)::DECIMAL / COUNT(*) * 100, 
        2
    ) as safety_percentage,
    AVG(safety_score) as avg_safety_score,
    COUNT(CASE WHEN risk_level = 'Low' THEN 1 END) as low_risk_count,
    COUNT(CASE WHEN risk_level = 'Medium' THEN 1 END) as medium_risk_count,
    COUNT(CASE WHEN risk_level = 'High' THEN 1 END) as high_risk_count
FROM invite_list
WHERE sector IS NOT NULL
GROUP BY sector
ORDER BY safety_percentage DESC, avg_safety_score DESC;

COMMENT ON VIEW sector_safety_analysis IS 'Current safety analysis summary by sector';

-- Step 8: Update risk_distribution view
DROP VIEW IF EXISTS risk_distribution;
CREATE OR REPLACE VIEW risk_distribution AS
SELECT 
    risk_level,
    COUNT(*) as stock_count,
    COUNT(CASE WHEN is_safe_to_trade = TRUE THEN 1 END) as safe_count,
    ROUND(
        COUNT(CASE WHEN is_safe_to_trade = TRUE THEN 1 END)::DECIMAL / COUNT(*) * 100, 
        2
    ) as safety_percentage,
    AVG(safety_score) as avg_safety_score
FROM invite_list
GROUP BY risk_level
ORDER BY 
    CASE risk_level 
        WHEN 'Low' THEN 1 
        WHEN 'Medium' THEN 2 
        WHEN 'High' THEN 3 
        WHEN 'VeryHigh' THEN 4 
    END;

COMMENT ON VIEW risk_distribution IS 'Current distribution of stocks by risk level and safety status';

-- Step 9: Create function to get historical data at specific date
CREATE OR REPLACE FUNCTION get_invite_list_at_date(target_date TIMESTAMPTZ)
RETURNS TABLE (
    symbol VARCHAR,
    company_name VARCHAR,
    sector VARCHAR,
    is_safe_to_trade BOOLEAN,
    safety_score DECIMAL,
    risk_level VARCHAR,
    analysis_date TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    SELECT DISTINCT ON (h.symbol)
        h.symbol,
        h.company_name,
        h.sector,
        h.is_safe_to_trade,
        h.safety_score,
        h.risk_level,
        h.analysis_date
    FROM invite_list_history h
    WHERE h.analysis_date <= target_date
    ORDER BY h.symbol, h.analysis_date DESC;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION get_invite_list_at_date IS 'Get the most recent analysis for each stock as of a specific date';

-- Step 10: Create function to get safety history for a specific stock
CREATE OR REPLACE FUNCTION get_stock_safety_history(
    stock_symbol VARCHAR,
    days_back INTEGER DEFAULT 90
)
RETURNS TABLE (
    analysis_date TIMESTAMPTZ,
    is_safe_to_trade BOOLEAN,
    safety_score DECIMAL,
    risk_level VARCHAR,
    safety_reasoning TEXT
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        h.analysis_date,
        h.is_safe_to_trade,
        h.safety_score,
        h.risk_level,
        h.safety_reasoning
    FROM invite_list_history h
    WHERE h.symbol = stock_symbol
      AND h.analysis_date >= NOW() - (days_back || ' days')::INTERVAL
    ORDER BY h.analysis_date DESC;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION get_stock_safety_history IS 'Get safety analysis history for a specific stock over a time period';

-- Step 11: Create function to compare current vs historical safety
CREATE OR REPLACE FUNCTION get_safety_changes(days_back INTEGER DEFAULT 7)
RETURNS TABLE (
    symbol VARCHAR,
    company_name VARCHAR,
    current_safe BOOLEAN,
    previous_safe BOOLEAN,
    current_score DECIMAL,
    previous_score DECIMAL,
    score_change DECIMAL,
    status_changed BOOLEAN
) AS $$
BEGIN
    RETURN QUERY
    WITH current_data AS (
        SELECT DISTINCT ON (symbol)
            symbol,
            company_name,
            is_safe_to_trade as current_safe,
            safety_score as current_score,
            analysis_date
        FROM invite_list_history
        ORDER BY symbol, analysis_date DESC
    ),
    previous_data AS (
        SELECT DISTINCT ON (symbol)
            symbol,
            is_safe_to_trade as previous_safe,
            safety_score as previous_score
        FROM invite_list_history
        WHERE analysis_date <= NOW() - (days_back || ' days')::INTERVAL
        ORDER BY symbol, analysis_date DESC
    )
    SELECT 
        c.symbol,
        c.company_name,
        c.current_safe,
        COALESCE(p.previous_safe, c.current_safe) as previous_safe,
        c.current_score,
        COALESCE(p.previous_score, c.current_score) as previous_score,
        c.current_score - COALESCE(p.previous_score, c.current_score) as score_change,
        (c.current_safe != COALESCE(p.previous_safe, c.current_safe)) as status_changed
    FROM current_data c
    LEFT JOIN previous_data p ON c.symbol = p.symbol
    WHERE c.current_safe != COALESCE(p.previous_safe, c.current_safe)
       OR ABS(c.current_score - COALESCE(p.previous_score, c.current_score)) > 0.1
    ORDER BY ABS(c.current_score - COALESCE(p.previous_score, c.current_score)) DESC;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION get_safety_changes IS 'Get stocks whose safety status or score has changed significantly';

-- Step 12: Create view for safety trends (last 30 days)
CREATE OR REPLACE VIEW invite_list_trends AS
SELECT 
    symbol,
    company_name,
    COUNT(*) as analysis_count,
    AVG(safety_score) as avg_safety_score,
    MIN(safety_score) as min_safety_score,
    MAX(safety_score) as max_safety_score,
    STDDEV(safety_score) as score_volatility,
    SUM(CASE WHEN is_safe_to_trade THEN 1 ELSE 0 END) as safe_count,
    ROUND(
        SUM(CASE WHEN is_safe_to_trade THEN 1 ELSE 0 END)::DECIMAL / COUNT(*) * 100,
        2
    ) as safety_consistency_pct
FROM invite_list_history
WHERE analysis_date >= NOW() - INTERVAL '30 days'
GROUP BY symbol, company_name
HAVING COUNT(*) >= 2  -- Only show stocks with multiple analyses
ORDER BY safety_consistency_pct DESC, avg_safety_score DESC;

COMMENT ON VIEW invite_list_trends IS 'Safety trends over the last 30 days for stocks with multiple analyses';

-- Step 13: Update comments
COMMENT ON TABLE invite_list_history IS 'Historical time-series data of S&P 500 stock safety analyses';
COMMENT ON CONSTRAINT invite_list_history_symbol_date_key ON invite_list_history 
    IS 'Ensures one analysis per stock per date';

-- Step 14: Create maintenance function to archive old data
CREATE OR REPLACE FUNCTION archive_old_invite_list_data(months_to_keep INTEGER DEFAULT 12)
RETURNS INTEGER AS $$
DECLARE
    deleted_count INTEGER;
BEGIN
    DELETE FROM invite_list_history
    WHERE analysis_date < NOW() - (months_to_keep || ' months')::INTERVAL;
    
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    
    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION archive_old_invite_list_data IS 'Archive/delete old analysis data older than specified months';

-- Migration complete!

