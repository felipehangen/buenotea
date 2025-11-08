-- Market Regime Analysis Table
-- Stores overall market regime analysis (the "vibe of the whole club")
-- This is separate from individual stock timing analysis

CREATE TABLE IF NOT EXISTS market_regime (
    id SERIAL PRIMARY KEY,
    analysis_date TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    -- Market Regime Classification
    market_regime VARCHAR(20) NOT NULL, -- 'Bull', 'Bear', 'Sideways', 'Volatile', 'Stable', 'Transition'
    regime_confidence DECIMAL(5,4) NOT NULL, -- 0.0000 to 1.0000
    
    -- Market Context Data
    spy_price DECIMAL(10,2),
    spy_20d_change DECIMAL(8,6), -- Percentage change
    spy_50d_change DECIMAL(8,6), -- Percentage change
    vix DECIMAL(6,2),
    market_breadth DECIMAL(5,4), -- 0.0000 to 1.0000 (advancing vs declining)
    sector_relative_performance DECIMAL(8,6), -- Percentage vs market
    
    -- Market Volatility Analysis
    market_volatility DECIMAL(8,6), -- Overall market volatility
    volatility_percentile DECIMAL(5,2), -- Volatility percentile (0-100)
    
    -- Market Trend Analysis
    short_term_trend VARCHAR(20), -- 'StrongBullish', 'Bullish', 'Neutral', 'Bearish', 'StrongBearish'
    medium_term_trend VARCHAR(20),
    long_term_trend VARCHAR(20),
    trend_strength DECIMAL(5,2), -- 0-100
    trend_consistency DECIMAL(5,2), -- 0-100
    
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
    fear_greed_index INTEGER, -- 0-100
    put_call_ratio DECIMAL(8,4),
    margin_debt_trend VARCHAR(20), -- 'Increasing', 'Decreasing', 'Stable'
    
    -- Risk Assessment
    market_risk_level VARCHAR(20), -- 'Low', 'Medium', 'High', 'VeryHigh'
    market_risk_score DECIMAL(5,2), -- 0-100
    max_drawdown_risk DECIMAL(5,2), -- Percentage
    
    -- AI Analysis (Optional)
    chatgpt_regime_analysis TEXT,
    chatgpt_market_outlook TEXT,
    chatgpt_risk_assessment TEXT,
    chatgpt_model_used VARCHAR(50),
    chatgpt_analysis_timestamp TIMESTAMP WITH TIME ZONE,
    
    -- Analysis Metadata
    data_sources_used TEXT[], -- Array of API sources used
    analysis_period_days INTEGER DEFAULT 250,
    computation_time_ms INTEGER,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes for efficient querying
CREATE INDEX IF NOT EXISTS idx_market_regime_date ON market_regime(analysis_date DESC);
CREATE INDEX IF NOT EXISTS idx_market_regime_type ON market_regime(market_regime);
CREATE INDEX IF NOT EXISTS idx_market_regime_confidence ON market_regime(regime_confidence);

-- Create a view for the latest market regime
CREATE OR REPLACE VIEW latest_market_regime AS
SELECT * FROM market_regime 
ORDER BY analysis_date DESC 
LIMIT 1;

-- Create a view for market regime history
CREATE OR REPLACE VIEW market_regime_history AS
SELECT 
    analysis_date,
    market_regime,
    regime_confidence,
    spy_price,
    spy_20d_change * 100 as spy_20d_change_pct,
    spy_50d_change * 100 as spy_50d_change_pct,
    vix,
    market_breadth * 100 as market_breadth_pct,
    market_volatility * 100 as market_volatility_pct,
    trend_strength,
    trend_consistency,
    market_risk_level,
    market_risk_score,
    created_at
FROM market_regime 
ORDER BY analysis_date DESC;

-- Add comments for documentation
COMMENT ON TABLE market_regime IS 'Stores overall market regime analysis - the "vibe of the whole club"';
COMMENT ON COLUMN market_regime.market_regime IS 'Overall market regime: Bull, Bear, Sideways, Volatile, Stable, Transition';
COMMENT ON COLUMN market_regime.regime_confidence IS 'Confidence in the regime classification (0.0 to 1.0)';
COMMENT ON COLUMN market_regime.market_breadth IS 'Percentage of advancing vs declining stocks (0.0 to 1.0)';
COMMENT ON COLUMN market_regime.fear_greed_index IS 'Market sentiment indicator (0-100, 0=extreme fear, 100=extreme greed)';
COMMENT ON COLUMN market_regime.put_call_ratio IS 'Put/Call ratio for sentiment analysis';
COMMENT ON COLUMN market_regime.margin_debt_trend IS 'Trend in margin debt levels';









