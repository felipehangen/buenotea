-- Migration: Update fundamentals table to use -1 to +1 scoring range
-- Created: 2025-10-05
-- Purpose: Update fundamentals table to match QSS scoring pattern (-1 to +1 range)

-- First, let's check if we need to drop and recreate the table
-- Since this is a significant schema change, we'll create a new version

-- Drop the existing table and recreate with new schema
DROP TABLE IF EXISTS fundamentals CASCADE;

-- Create fundamentals table with new scoring system
CREATE TABLE IF NOT EXISTS fundamentals (
    id BIGSERIAL PRIMARY KEY,
    symbol VARCHAR(10) NOT NULL,
    analysis_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Core scoring data (now using -1 to +1 range like QSS)
    fundamentals_score DECIMAL(3,2) NOT NULL CHECK (fundamentals_score >= -1 AND fundamentals_score <= 1),
    trading_signal VARCHAR(20) NOT NULL CHECK (trading_signal IN ('StrongBuy', 'WeakBuy', 'Hold', 'WeakSell', 'StrongSell')),
    confidence_score DECIMAL(3,2) NOT NULL CHECK (confidence_score >= 0 AND confidence_score <= 1),
    
    -- Component scores (-1 to +1 range)
    profitability_score DECIMAL(3,2) NOT NULL CHECK (profitability_score >= -1 AND profitability_score <= 1),
    growth_score DECIMAL(3,2) NOT NULL CHECK (growth_score >= -1 AND growth_score <= 1),
    valuation_score DECIMAL(3,2) NOT NULL CHECK (valuation_score >= -1 AND valuation_score <= 1),
    financial_strength_score DECIMAL(3,2) NOT NULL CHECK (financial_strength_score >= -1 AND financial_strength_score <= 1),
    efficiency_score DECIMAL(3,2) NOT NULL CHECK (efficiency_score >= -1 AND efficiency_score <= 1),
    
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
    market_cap_category VARCHAR(20),
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
    flags TEXT[], -- Array of warning flags
    
    -- API tracking for each component
    profitability_api_url TEXT,
    profitability_api_source VARCHAR(50),
    profitability_data_available BOOLEAN,
    profitability_raw_data JSONB,
    
    growth_api_url TEXT,
    growth_api_source VARCHAR(50),
    growth_data_available BOOLEAN,
    growth_raw_data JSONB,
    
    valuation_api_url TEXT,
    valuation_api_source VARCHAR(50),
    valuation_data_available BOOLEAN,
    valuation_raw_data JSONB,
    
    financial_strength_api_url TEXT,
    financial_strength_api_source VARCHAR(50),
    financial_strength_data_available BOOLEAN,
    financial_strength_raw_data JSONB,
    
    efficiency_api_url TEXT,
    efficiency_api_source VARCHAR(50),
    efficiency_data_available BOOLEAN,
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
CREATE INDEX IF NOT EXISTS idx_fundamentals_fundamentals_score ON fundamentals(fundamentals_score);
CREATE INDEX IF NOT EXISTS idx_fundamentals_trading_signal ON fundamentals(trading_signal);
CREATE INDEX IF NOT EXISTS idx_fundamentals_symbol_date ON fundamentals(symbol, analysis_date DESC);

-- Create trigger to automatically update updated_at timestamp
CREATE OR REPLACE FUNCTION update_fundamentals_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_fundamentals_updated_at
    BEFORE UPDATE ON fundamentals
    FOR EACH ROW
    EXECUTE FUNCTION update_fundamentals_updated_at();

-- Create view for latest fundamentals by symbol
CREATE OR REPLACE VIEW latest_fundamentals AS
SELECT DISTINCT ON (symbol) *
FROM fundamentals
ORDER BY symbol, analysis_date DESC;

-- Add comments for documentation
COMMENT ON TABLE fundamentals IS 'Stores comprehensive fundamentals analysis data with -1 to +1 scoring range';
COMMENT ON COLUMN fundamentals.fundamentals_score IS 'Overall fundamentals score between -1.0 and +1.0 (negative = poor, positive = good)';
COMMENT ON COLUMN fundamentals.trading_signal IS 'Trading signal based on fundamentals score (StrongBuy, WeakBuy, Hold, WeakSell, StrongSell)';
COMMENT ON COLUMN fundamentals.profitability_score IS 'Profitability component score between -1.0 and +1.0';
COMMENT ON COLUMN fundamentals.growth_score IS 'Growth component score between -1.0 and +1.0';
COMMENT ON COLUMN fundamentals.valuation_score IS 'Valuation component score between -1.0 and +1.0 (negative = expensive, positive = cheap)';
COMMENT ON COLUMN fundamentals.financial_strength_score IS 'Financial strength component score between -1.0 and +1.0';
COMMENT ON COLUMN fundamentals.efficiency_score IS 'Efficiency component score between -1.0 and +1.0';
