-- Migration: Create invite_list table for storing S&P 500 stocks safe to trade
-- Created: 2025-01-05
-- Purpose: Store S&P 500 stock list with safety analysis and trading eligibility

-- Create invite_list table for storing S&P 500 stocks and their safety status
CREATE TABLE IF NOT EXISTS invite_list (
    id BIGSERIAL PRIMARY KEY,
    symbol VARCHAR(10) NOT NULL UNIQUE,
    company_name VARCHAR(255) NOT NULL,
    sector VARCHAR(100),
    industry VARCHAR(100),
    market_cap BIGINT, -- Market capitalization in USD
    current_price DECIMAL(10,2),
    
    -- Safety analysis results
    is_safe_to_trade BOOLEAN NOT NULL DEFAULT FALSE,
    safety_score DECIMAL(3,2), -- Range: 0.00 to 1.00
    safety_reasoning TEXT,
    
    -- Basic financial health checks
    has_recent_earnings BOOLEAN DEFAULT TRUE,
    has_positive_revenue BOOLEAN DEFAULT TRUE,
    has_stable_price BOOLEAN DEFAULT TRUE,
    has_sufficient_volume BOOLEAN DEFAULT TRUE,
    has_analyst_coverage BOOLEAN DEFAULT TRUE,
    
    -- Risk assessment
    risk_level VARCHAR(10) NOT NULL DEFAULT 'Medium', -- Low, Medium, High, VeryHigh
    volatility_rating VARCHAR(10), -- Low, Medium, High
    liquidity_rating VARCHAR(10), -- Low, Medium, High
    
    -- Data quality and source tracking
    data_source VARCHAR(50) NOT NULL, -- FMP, AlphaVantage, etc.
    last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    data_freshness_score DECIMAL(3,2), -- Range: 0.00 to 1.00
    
    -- Analysis metadata
    analysis_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    analysis_duration_ms INTEGER,
    warning_flags JSONB, -- Array of warning flags
    missing_data_components JSONB, -- Array of missing data components
    
    -- Raw API response data (JSON)
    raw_company_data JSONB,
    raw_financial_data JSONB,
    raw_price_data JSONB,
    
    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_invite_list_symbol ON invite_list(symbol);
CREATE INDEX IF NOT EXISTS idx_invite_list_safe_to_trade ON invite_list(is_safe_to_trade);
CREATE INDEX IF NOT EXISTS idx_invite_list_safety_score ON invite_list(safety_score);
CREATE INDEX IF NOT EXISTS idx_invite_list_sector ON invite_list(sector);
CREATE INDEX IF NOT EXISTS idx_invite_list_risk_level ON invite_list(risk_level);
CREATE INDEX IF NOT EXISTS idx_invite_list_analysis_date ON invite_list(analysis_date);
CREATE INDEX IF NOT EXISTS idx_invite_list_last_updated ON invite_list(last_updated);

-- Create composite indexes for common queries
CREATE INDEX IF NOT EXISTS idx_invite_list_safe_sector ON invite_list(is_safe_to_trade, sector);
CREATE INDEX IF NOT EXISTS idx_invite_list_safe_risk ON invite_list(is_safe_to_trade, risk_level);

-- Add comments for documentation
COMMENT ON TABLE invite_list IS 'Stores S&P 500 stocks with safety analysis for trading eligibility';
COMMENT ON COLUMN invite_list.symbol IS 'Stock symbol (e.g., AAPL, MSFT)';
COMMENT ON COLUMN invite_list.company_name IS 'Full company name';
COMMENT ON COLUMN invite_list.is_safe_to_trade IS 'Whether the stock is considered safe for trading';
COMMENT ON COLUMN invite_list.safety_score IS 'Overall safety score from 0.00 to 1.00';
COMMENT ON COLUMN invite_list.safety_reasoning IS 'Human-readable explanation of safety assessment';
COMMENT ON COLUMN invite_list.risk_level IS 'Risk assessment: Low, Medium, High, VeryHigh';
COMMENT ON COLUMN invite_list.volatility_rating IS 'Price volatility assessment';
COMMENT ON COLUMN invite_list.liquidity_rating IS 'Trading liquidity assessment';
COMMENT ON COLUMN invite_list.data_source IS 'Primary data source used for analysis';
COMMENT ON COLUMN invite_list.warning_flags IS 'JSON array of warning flags and data quality issues';
COMMENT ON COLUMN invite_list.missing_data_components IS 'JSON array of missing data components';

-- Create trigger to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_invite_list_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_invite_list_updated_at
    BEFORE UPDATE ON invite_list
    FOR EACH ROW
    EXECUTE FUNCTION update_invite_list_updated_at();

-- Create view for safe stocks only
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

COMMENT ON VIEW safe_stocks IS 'View of stocks that are safe to trade, ordered by safety score';

-- Create view for sector analysis
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

COMMENT ON VIEW sector_safety_analysis IS 'Safety analysis summary by sector';

-- Create view for risk level distribution
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

COMMENT ON VIEW risk_distribution IS 'Distribution of stocks by risk level and safety status';

