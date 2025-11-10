-- Migration: Create regime table for storing market regime analysis and TTS scores
-- Created: 2025-01-05
-- Purpose: Store comprehensive regime analysis data including TTS scoring, market regime detection, and AI analysis

-- Create regime table for storing market regime analysis and TTS scores
CREATE TABLE IF NOT EXISTS regime (
    id BIGSERIAL PRIMARY KEY,
    symbol VARCHAR(10) NOT NULL,
    analysis_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Core TTS scoring data
    tts_score DECIMAL(5,3) NOT NULL, -- Range: -1.000 to +1.000
    trading_signal VARCHAR(20) NOT NULL, -- StrongHold, Hold, Neutral, Sell, StrongSell
    market_regime VARCHAR(20) NOT NULL, -- Bull, Bear, Sideways, Volatile, Stable, Transition
    confidence_score DECIMAL(3,2) NOT NULL, -- Range: 0.00 to 1.00
    
    -- TTS component scores (weighted components)
    momentum_score DECIMAL(5,3) NOT NULL, -- 30% weight
    volatility_score DECIMAL(5,3) NOT NULL, -- 25% weight
    volume_score DECIMAL(5,3) NOT NULL, -- 20% weight
    support_resistance_score DECIMAL(5,3) NOT NULL, -- 15% weight
    market_correlation_score DECIMAL(5,3) NOT NULL, -- 10% weight
    
    -- Technical indicators
    rsi_14 DECIMAL(8,2),
    macd DECIMAL(10,6),
    macd_signal DECIMAL(10,6),
    macd_histogram DECIMAL(10,6),
    bollinger_upper DECIMAL(10,2),
    bollinger_middle DECIMAL(10,2),
    bollinger_lower DECIMAL(10,2),
    sma_20 DECIMAL(10,2),
    sma_50 DECIMAL(10,2),
    sma_200 DECIMAL(10,2),
    atr_14 DECIMAL(10,2),
    stochastic_k DECIMAL(8,2),
    stochastic_d DECIMAL(8,2),
    williams_r DECIMAL(8,2),
    
    -- Market context data
    spy_price DECIMAL(10,2),
    spy_20d_change DECIMAL(8,4),
    spy_50d_change DECIMAL(8,4),
    vix DECIMAL(8,2),
    sector_relative_performance DECIMAL(8,4),
    market_breadth DECIMAL(8,4),
    
    -- Risk assessment
    risk_level VARCHAR(10) NOT NULL, -- Low, Medium, High, VeryHigh
    volatility_score_percent DECIMAL(8,2),
    max_drawdown_risk DECIMAL(8,2),
    stop_loss DECIMAL(10,2),
    risk_reward_ratio DECIMAL(8,2),
    position_size DECIMAL(3,2), -- Range: 0.00 to 1.00
    
    -- API endpoint tracking
    primary_api_source VARCHAR(50),
    fallback_api_source VARCHAR(50),
    api_endpoints_used JSONB, -- Array of endpoint URLs
    price_data_points INTEGER,
    market_data_points INTEGER,
    analysis_period_days INTEGER,
    current_price DECIMAL(10,2),
    
    -- Raw API response data (JSON)
    price_data_raw_response JSONB,
    market_data_raw_response JSONB,
    
    -- Data quality and flags
    flags JSONB, -- Array of warning flags
    data_quality_score DECIMAL(3,2),
    computation_time_ms BIGINT,
    
    -- AI Analysis
    chatgpt_regime_analysis TEXT, -- AI explanation of regime detection
    chatgpt_tts_interpretation TEXT, -- AI explanation of TTS score meaning
    chatgpt_trading_recommendation TEXT, -- AI trading suggestion
    chatgpt_analysis_timestamp TIMESTAMPTZ,
    chatgpt_model_used VARCHAR(50),
    
    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_regime_symbol ON regime(symbol);
CREATE INDEX IF NOT EXISTS idx_regime_analysis_date ON regime(analysis_date);
CREATE INDEX IF NOT EXISTS idx_regime_symbol_date ON regime(symbol, analysis_date DESC);
CREATE INDEX IF NOT EXISTS idx_regime_tts_score ON regime(tts_score);
CREATE INDEX IF NOT EXISTS idx_regime_market_regime ON regime(market_regime);
CREATE INDEX IF NOT EXISTS idx_regime_trading_signal ON regime(trading_signal);
CREATE INDEX IF NOT EXISTS idx_regime_risk_level ON regime(risk_level);

-- Create a composite index for common queries
CREATE INDEX IF NOT EXISTS idx_regime_symbol_regime_signal ON regime(symbol, market_regime, trading_signal);

-- Add comments for documentation
COMMENT ON TABLE regime IS 'Stores market regime analysis and Time To Sell (TTS) scores for stocks';
COMMENT ON COLUMN regime.tts_score IS 'Time To Sell score: -1.0 (Strong Sell) to +1.0 (Strong Hold)';
COMMENT ON COLUMN regime.trading_signal IS 'Derived trading signal from TTS score';
COMMENT ON COLUMN regime.market_regime IS 'Detected market regime affecting the analysis';
COMMENT ON COLUMN regime.momentum_score IS 'Price momentum component score (30% weight in TTS)';
COMMENT ON COLUMN regime.volatility_score IS 'Volatility analysis component score (25% weight in TTS)';
COMMENT ON COLUMN regime.volume_score IS 'Volume analysis component score (20% weight in TTS)';
COMMENT ON COLUMN regime.support_resistance_score IS 'Support/resistance analysis score (15% weight in TTS)';
COMMENT ON COLUMN regime.market_correlation_score IS 'Market correlation component score (10% weight in TTS)';
COMMENT ON COLUMN regime.api_endpoints_used IS 'JSON array of API endpoints used in analysis';
COMMENT ON COLUMN regime.flags IS 'JSON array of warning flags and data quality issues';
COMMENT ON COLUMN regime.chatgpt_regime_analysis IS 'AI explanation of market regime detection';
COMMENT ON COLUMN regime.chatgpt_tts_interpretation IS 'AI explanation of TTS score meaning and implications';
COMMENT ON COLUMN regime.chatgpt_trading_recommendation IS 'AI-generated trading recommendation based on analysis';

-- Create trigger to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_regime_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_regime_updated_at
    BEFORE UPDATE ON regime
    FOR EACH ROW
    EXECUTE FUNCTION update_regime_updated_at();

-- Create view for latest regime analysis per symbol
CREATE OR REPLACE VIEW latest_regime_analysis AS
SELECT DISTINCT ON (symbol)
    symbol,
    analysis_date,
    tts_score,
    trading_signal,
    market_regime,
    confidence_score,
    risk_level,
    position_size,
    flags,
    chatgpt_trading_recommendation,
    created_at
FROM regime
ORDER BY symbol, analysis_date DESC;

COMMENT ON VIEW latest_regime_analysis IS 'Latest regime analysis for each stock symbol';

-- Create view for regime analysis summary statistics
CREATE OR REPLACE VIEW regime_analysis_summary AS
SELECT 
    symbol,
    COUNT(*) as total_analyses,
    AVG(tts_score) as avg_tts_score,
    MIN(tts_score) as min_tts_score,
    MAX(tts_score) as max_tts_score,
    AVG(confidence_score) as avg_confidence,
    COUNT(DISTINCT market_regime) as regime_types_count,
    COUNT(DISTINCT trading_signal) as signal_types_count,
    MIN(analysis_date) as first_analysis,
    MAX(analysis_date) as last_analysis
FROM regime
GROUP BY symbol
ORDER BY last_analysis DESC;

COMMENT ON VIEW regime_analysis_summary IS 'Summary statistics for regime analysis per stock symbol';

