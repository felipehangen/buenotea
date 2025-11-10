-- Convert timing table to time-series history approach
-- This migration enables tracking of timing signals over time

-- Step 1: Rename current table to timing_history
ALTER TABLE timing 
    RENAME TO timing_history;

-- Step 2: Drop any existing unique constraint on symbol
ALTER TABLE timing_history 
    DROP CONSTRAINT IF EXISTS timing_symbol_key;

-- Step 3: Add composite unique constraint (symbol + created_at)
-- This ensures we don't accidentally insert duplicate records for the same timestamp
ALTER TABLE timing_history 
    ADD CONSTRAINT timing_history_symbol_created_key 
    UNIQUE(symbol, created_at);

-- Step 4: Create view for latest timing data (backwards compatibility)
-- This view always shows the most recent timing analysis for each symbol
CREATE OR REPLACE VIEW timing AS
SELECT DISTINCT ON (symbol) *
FROM timing_history
ORDER BY symbol, created_at DESC;

-- Step 5: Create function to get historical timing data at a specific date
CREATE OR REPLACE FUNCTION get_timing_at_date(target_date TIMESTAMPTZ)
RETURNS TABLE (
    symbol VARCHAR,
    tts_score DECIMAL,
    trading_signal VARCHAR,
    created_at TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    SELECT DISTINCT ON (h.symbol)
        h.symbol,
        h.tts_score,
        h.trading_signal,
        h.created_at
    FROM timing_history h
    WHERE h.created_at <= target_date
    ORDER BY h.symbol, h.created_at DESC;
END;
$$ LANGUAGE plpgsql;

-- Step 6: Create function to get timing history for a specific stock
CREATE OR REPLACE FUNCTION get_timing_history(stock_symbol VARCHAR, days_back INTEGER DEFAULT 90)
RETURNS TABLE (
    symbol VARCHAR,
    tts_score DECIMAL,
    trading_signal VARCHAR,
    confidence_score DECIMAL,
    short_term_trend VARCHAR,
    medium_term_trend VARCHAR,
    long_term_trend VARCHAR,
    risk_level VARCHAR,
    created_at TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        h.symbol,
        h.tts_score,
        h.trading_signal,
        h.confidence_score,
        h.short_term_trend,
        h.medium_term_trend,
        h.long_term_trend,
        h.risk_level,
        h.created_at
    FROM timing_history h
    WHERE h.symbol = stock_symbol
      AND h.created_at >= NOW() - (days_back || ' days')::INTERVAL
    ORDER BY h.created_at DESC;
END;
$$ LANGUAGE plpgsql;

-- Step 7: Create function to detect signal changes (flips from Buy to Sell, etc.)
CREATE OR REPLACE FUNCTION get_timing_signal_changes(days_back INTEGER DEFAULT 7)
RETURNS TABLE (
    symbol VARCHAR,
    old_signal VARCHAR,
    new_signal VARCHAR,
    old_tts_score DECIMAL,
    new_tts_score DECIMAL,
    change_date TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    WITH ranked_signals AS (
        SELECT 
            h.symbol,
            h.trading_signal,
            h.tts_score,
            h.created_at,
            LAG(h.trading_signal) OVER (PARTITION BY h.symbol ORDER BY h.created_at) as prev_signal,
            LAG(h.tts_score) OVER (PARTITION BY h.symbol ORDER BY h.created_at) as prev_score
        FROM timing_history h
        WHERE h.created_at >= NOW() - (days_back || ' days')::INTERVAL
    )
    SELECT 
        rs.symbol,
        rs.prev_signal as old_signal,
        rs.trading_signal as new_signal,
        rs.prev_score as old_tts_score,
        rs.tts_score as new_tts_score,
        rs.created_at as change_date
    FROM ranked_signals rs
    WHERE rs.prev_signal IS NOT NULL 
      AND rs.prev_signal != rs.trading_signal
    ORDER BY rs.created_at DESC;
END;
$$ LANGUAGE plpgsql;

-- Step 8: Create function to get stocks with specific signal
CREATE OR REPLACE FUNCTION get_stocks_by_signal(signal_type VARCHAR)
RETURNS TABLE (
    symbol VARCHAR,
    tts_score DECIMAL,
    confidence_score DECIMAL,
    risk_level VARCHAR,
    created_at TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    SELECT DISTINCT ON (t.symbol)
        t.symbol,
        t.tts_score,
        t.confidence_score,
        t.risk_level,
        t.created_at
    FROM timing_history t
    WHERE t.trading_signal = signal_type
    ORDER BY t.symbol, t.created_at DESC;
END;
$$ LANGUAGE plpgsql;

-- Step 9: Add helpful comments
COMMENT ON VIEW timing IS 'View showing the latest timing analysis for each symbol. Queries this view for current signals.';
COMMENT ON FUNCTION get_timing_at_date IS 'Returns the timing analysis that was valid at a specific date in the past';
COMMENT ON FUNCTION get_timing_history IS 'Returns all timing analyses for a specific stock over the past N days';
COMMENT ON FUNCTION get_timing_signal_changes IS 'Detects when stocks changed their trading signal (e.g., Buy → Sell)';
COMMENT ON FUNCTION get_stocks_by_signal IS 'Returns latest analysis for all stocks with a specific signal (StrongBuy, Buy, etc.)';

-- Step 10: Create indexes for performance on history table
CREATE INDEX IF NOT EXISTS idx_timing_history_symbol_created ON timing_history(symbol, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_timing_history_signal_created ON timing_history(trading_signal, created_at DESC);

-- Migration complete!
-- Usage examples:
-- 1. Get latest timing for all stocks: SELECT * FROM timing;
-- 2. Get AAPL timing 30 days ago: SELECT * FROM get_timing_at_date(NOW() - INTERVAL '30 days') WHERE symbol = 'AAPL';
-- 3. Get AAPL history: SELECT * FROM get_timing_history('AAPL', 90);
-- 4. Find signal changes: SELECT * FROM get_timing_signal_changes(7);
-- 5. Get all Buy signals: SELECT * FROM get_stocks_by_signal('Buy');

