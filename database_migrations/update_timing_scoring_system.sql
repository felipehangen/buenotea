-- Update timing table to use new -1.0 to +1.0 scoring system
-- This migration updates the scoring system from 0-100 to -1.0 to +1.0

-- First, let's add comments to document the new scoring system
COMMENT ON COLUMN timing.tts_score IS 'Overall TTS score from -1.0 to +1.0 (+1.0=StrongBuy, +0.5=Buy, 0.0=Neutral, -0.5=Sell, -1.0=StrongSell)';
COMMENT ON COLUMN timing.rsi_score IS 'RSI score from -1.0 to +1.0 (negative=overbought, positive=oversold)';
COMMENT ON COLUMN timing.macd_score IS 'MACD score from -1.0 to +1.0 (positive=bullish, negative=bearish)';
COMMENT ON COLUMN timing.bollinger_score IS 'Bollinger Bands score from -1.0 to +1.0 (negative=above bands, positive=below bands)';
COMMENT ON COLUMN timing.ma_score IS 'Moving Averages score from -1.0 to +1.0 (positive=above MAs, negative=below MAs)';
COMMENT ON COLUMN timing.stochastic_score IS 'Stochastic score from -1.0 to +1.0 (negative=overbought, positive=oversold)';
COMMENT ON COLUMN timing.williams_score IS 'Williams %R score from -1.0 to +1.0 (negative=overbought, positive=oversold)';
COMMENT ON COLUMN timing.atr_score IS 'ATR score from -1.0 to +1.0 (negative=high volatility risk, positive=low volatility opportunity)';
COMMENT ON COLUMN timing.volume_score IS 'Volume score from -1.0 to +1.0 (positive=high volume confirms trend, negative=low volume weakens trend)';

-- Update the trading signal thresholds in comments
COMMENT ON COLUMN timing.trading_signal IS 'Trading signal: StrongBuy (>=+0.6), Buy (>=+0.2), Neutral (-0.2 to +0.2), Sell (<=-0.2), StrongSell (<=-0.6)';

-- Note: The existing data will need to be converted if there are any records
-- This is a schema documentation update - the actual column types remain the same
-- but the interpretation of values changes from 0-100 scale to -1.0 to +1.0 scale

-- Add a new view to show the scoring system interpretation
CREATE OR REPLACE VIEW timing_score_interpretation AS
SELECT 
    id,
    symbol,
    tts_score,
    trading_signal,
    CASE 
        WHEN tts_score >= 0.6 THEN 'Strong Buy Signal'
        WHEN tts_score >= 0.2 THEN 'Buy Signal'
        WHEN tts_score >= -0.2 THEN 'Neutral Signal'
        WHEN tts_score >= -0.6 THEN 'Sell Signal'
        ELSE 'Strong Sell Signal'
    END as signal_interpretation,
    CASE 
        WHEN tts_score >= 0.6 THEN 1.0
        WHEN tts_score >= 0.2 THEN 0.5
        WHEN tts_score >= -0.2 THEN 0.0
        WHEN tts_score >= -0.6 THEN -0.5
        ELSE -1.0
    END as position_size_recommendation,
    created_at
FROM timing;

-- Add index for the new scoring system
CREATE INDEX IF NOT EXISTS idx_timing_tts_score_range ON timing(tts_score) WHERE tts_score >= 0.6 OR tts_score <= -0.6;

-- Update table comment
COMMENT ON TABLE timing IS 'Stores Technical Trading Score (TTS) analysis results with -1.0 to +1.0 scoring system and comprehensive technical indicators';

-- Function to convert old 0-100 scores to new -1.0 to +1.0 scale (for data migration if needed)
CREATE OR REPLACE FUNCTION convert_score_to_new_scale(old_score DECIMAL(5,2))
RETURNS DECIMAL(5,2) AS $$
BEGIN
    -- Convert from 0-100 scale to -1.0 to +1.0 scale
    -- 100 -> +1.0, 80 -> +0.6, 60 -> +0.2, 50 -> 0.0, 40 -> -0.2, 20 -> -0.6, 0 -> -1.0
    RETURN ((old_score - 50.0) / 50.0);
END;
$$ LANGUAGE plpgsql;

-- Function to get position size from TTS score
CREATE OR REPLACE FUNCTION get_position_size(tts_score DECIMAL(5,2))
RETURNS DECIMAL(3,2) AS $$
BEGIN
    CASE 
        WHEN tts_score >= 0.6 THEN RETURN 1.0;   -- Maximum long position
        WHEN tts_score >= 0.2 THEN RETURN 0.5;   -- Moderate long position
        WHEN tts_score >= -0.2 THEN RETURN 0.0;  -- No change
        WHEN tts_score >= -0.6 THEN RETURN -0.5; -- Moderate short position
        ELSE RETURN -1.0;                        -- Maximum short position
    END CASE;
END;
$$ LANGUAGE plpgsql;
