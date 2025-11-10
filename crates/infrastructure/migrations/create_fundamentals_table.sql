-- Migration: Create fundamentals table for storing stock fundamentals analysis
-- Created: 2025-10-05
-- Purpose: Store comprehensive fundamentals analysis data including scoring, metrics, and AI analysis

-- Create fundamentals table for storing stock fundamentals analysis
CREATE TABLE IF NOT EXISTS fundamentals (
    id BIGSERIAL PRIMARY KEY,
    symbol VARCHAR(10) NOT NULL,
    analysis_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Core scoring data
    fundamentals_score DECIMAL(5,2) NOT NULL,
    rating VARCHAR(5) NOT NULL,
    recommendation VARCHAR(20) NOT NULL,
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
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_fundamentals_symbol ON fundamentals(symbol);
CREATE INDEX IF NOT EXISTS idx_fundamentals_analysis_date ON fundamentals(analysis_date);
CREATE INDEX IF NOT EXISTS idx_fundamentals_symbol_date ON fundamentals(symbol, analysis_date);
CREATE INDEX IF NOT EXISTS idx_fundamentals_fundamentals_score ON fundamentals(fundamentals_score);
CREATE INDEX IF NOT EXISTS idx_fundamentals_recommendation ON fundamentals(recommendation);
CREATE INDEX IF NOT EXISTS idx_fundamentals_sector ON fundamentals(sector);
CREATE INDEX IF NOT EXISTS idx_fundamentals_rating ON fundamentals(rating);

-- Create a function to automatically update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_fundamentals_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to automatically update updated_at
CREATE TRIGGER trigger_update_fundamentals_updated_at
    BEFORE UPDATE ON fundamentals
    FOR EACH ROW
    EXECUTE FUNCTION update_fundamentals_updated_at();

-- Create a view for the latest fundamentals analysis per symbol
CREATE OR REPLACE VIEW latest_fundamentals AS
SELECT DISTINCT ON (symbol)
    id,
    symbol,
    analysis_date,
    fundamentals_score,
    rating,
    recommendation,
    confidence_score,
    profitability_score,
    growth_score,
    valuation_score,
    financial_strength_score,
    efficiency_score,
    sector,
    industry,
    market_cap,
    beta,
    dividend_yield,
    gpt_explanation,
    gpt_trading_suggestion,
    created_at
FROM fundamentals
ORDER BY symbol, analysis_date DESC;

-- Add comments for documentation
COMMENT ON TABLE fundamentals IS 'Comprehensive fundamentals analysis data for stocks including scoring, metrics, and AI analysis';
COMMENT ON COLUMN fundamentals.fundamentals_score IS 'Overall fundamentals score (0-100)';
COMMENT ON COLUMN fundamentals.rating IS 'Letter grade rating (A+ to F)';
COMMENT ON COLUMN fundamentals.recommendation IS 'Investment recommendation (StrongBuy, Buy, Hold, Sell, StrongSell)';
COMMENT ON COLUMN fundamentals.confidence_score IS 'Confidence in the analysis (0.0-1.0)';
COMMENT ON COLUMN fundamentals.gpt_explanation IS 'ChatGPT explanation of the fundamentals score';
COMMENT ON COLUMN fundamentals.gpt_trading_suggestion IS 'ChatGPT trading suggestion for the stock';
COMMENT ON COLUMN fundamentals.flags IS 'Array of warning flags about data quality or analysis limitations';
COMMENT ON COLUMN fundamentals.profitability_raw_data IS 'Raw JSON data from profitability API endpoint';
COMMENT ON COLUMN fundamentals.growth_raw_data IS 'Raw JSON data from growth API endpoint';
COMMENT ON COLUMN fundamentals.valuation_raw_data IS 'Raw JSON data from valuation API endpoint';
COMMENT ON COLUMN fundamentals.financial_strength_raw_data IS 'Raw JSON data from financial strength API endpoint';
COMMENT ON COLUMN fundamentals.efficiency_raw_data IS 'Raw JSON data from efficiency API endpoint';

-- Grant permissions (adjust as needed for your setup)
-- GRANT SELECT, INSERT, UPDATE, DELETE ON fundamentals TO your_app_user;
-- GRANT USAGE, SELECT ON SEQUENCE fundamentals_id_seq TO your_app_user;
-- GRANT SELECT ON latest_fundamentals TO your_app_user;
