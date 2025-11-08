-- Create timing table for Technical Trading Score (TTS) data
-- This table stores comprehensive TTS analysis results with API source tracking

CREATE TABLE IF NOT EXISTS timing (
    id SERIAL PRIMARY KEY,
    
    -- Stock identification
    symbol VARCHAR(10) NOT NULL,
    
    -- TTS Analysis Results
    tts_score DECIMAL(5,2) NOT NULL,
    trading_signal VARCHAR(20) NOT NULL,
    confidence_score DECIMAL(3,2) NOT NULL,
    
    -- Technical Indicators Scores (0-100)
    rsi_score DECIMAL(5,2) NOT NULL,
    macd_score DECIMAL(5,2) NOT NULL,
    bollinger_score DECIMAL(5,2) NOT NULL,
    ma_score DECIMAL(5,2) NOT NULL,
    stochastic_score DECIMAL(5,2) NOT NULL,
    williams_score DECIMAL(5,2) NOT NULL,
    atr_score DECIMAL(5,2) NOT NULL,
    volume_score DECIMAL(5,2) NOT NULL,
    
    -- Trend Analysis
    short_term_trend VARCHAR(20) NOT NULL,
    medium_term_trend VARCHAR(20) NOT NULL,
    long_term_trend VARCHAR(20) NOT NULL,
    trend_strength DECIMAL(5,2) NOT NULL,
    trend_consistency DECIMAL(5,2) NOT NULL,
    
    -- Support & Resistance
    support_level DECIMAL(10,2) NOT NULL,
    resistance_level DECIMAL(10,2) NOT NULL,
    support_distance DECIMAL(5,2) NOT NULL,
    resistance_distance DECIMAL(5,2) NOT NULL,
    support_strength DECIMAL(5,2) NOT NULL,
    resistance_strength DECIMAL(5,2) NOT NULL,
    
    -- Volume Analysis
    current_volume BIGINT NOT NULL,
    avg_volume BIGINT NOT NULL,
    volume_ratio DECIMAL(8,4) NOT NULL,
    volume_trend VARCHAR(20) NOT NULL,
    vp_relationship VARCHAR(20) NOT NULL,
    
    -- Risk Assessment
    volatility_score DECIMAL(5,2) NOT NULL,
    risk_level VARCHAR(20) NOT NULL,
    max_drawdown_risk DECIMAL(5,2) NOT NULL,
    stop_loss DECIMAL(10,2) NOT NULL,
    risk_reward_ratio DECIMAL(8,4) NOT NULL,
    
    -- API Source Tracking
    primary_api_source VARCHAR(50) NOT NULL,
    fallback_api_source VARCHAR(50),
    api_endpoints_used JSONB NOT NULL,
    raw_api_responses JSONB,
    
    -- Price Data Used for Analysis
    price_data_points INTEGER NOT NULL,
    analysis_period_days INTEGER NOT NULL,
    current_price DECIMAL(10,2) NOT NULL,
    
    -- AI Analysis
    chatgpt_explanation TEXT,
    trading_suggestion TEXT,
    
    -- Metadata
    flags JSONB DEFAULT '[]'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_timing_symbol ON timing(symbol);
CREATE INDEX IF NOT EXISTS idx_timing_created_at ON timing(created_at);
CREATE INDEX IF NOT EXISTS idx_timing_tts_score ON timing(tts_score);
CREATE INDEX IF NOT EXISTS idx_timing_trading_signal ON timing(trading_signal);
CREATE INDEX IF NOT EXISTS idx_timing_symbol_created_at ON timing(symbol, created_at);

-- Create a function to update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_timing_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to automatically update updated_at
CREATE TRIGGER trigger_update_timing_updated_at
    BEFORE UPDATE ON timing
    FOR EACH ROW
    EXECUTE FUNCTION update_timing_updated_at();

-- Add comments for documentation
COMMENT ON TABLE timing IS 'Stores Technical Trading Score (TTS) analysis results with comprehensive technical indicators and AI explanations';
COMMENT ON COLUMN timing.symbol IS 'Stock symbol (e.g., AAPL, MSFT)';
COMMENT ON COLUMN timing.tts_score IS 'Overall TTS score from 0-100';
COMMENT ON COLUMN timing.trading_signal IS 'Trading signal: StrongBuy, Buy, Neutral, Sell, StrongSell';
COMMENT ON COLUMN timing.api_endpoints_used IS 'JSON array of API endpoints that were called';
COMMENT ON COLUMN timing.raw_api_responses IS 'JSON object containing raw API responses for debugging';
COMMENT ON COLUMN timing.chatgpt_explanation IS 'AI-generated explanation of what the TTS score reveals';
COMMENT ON COLUMN timing.trading_suggestion IS 'AI-generated trading suggestion based on the analysis';
