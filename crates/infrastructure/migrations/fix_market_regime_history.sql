-- Fix market_regime_history table structure
-- This migration adds missing columns to the existing market_regime_history table

-- Step 1: Add missing columns to market_regime_history (if they don't exist)
DO $$
BEGIN
    RAISE NOTICE 'Starting to add missing columns to market_regime_history...';
    
    -- Market Breadth Analysis columns
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                   WHERE table_schema = 'public' 
                   AND table_name = 'market_regime_history' 
                   AND column_name = 'advancing_stocks') THEN
        ALTER TABLE market_regime_history ADD COLUMN advancing_stocks INTEGER;
        RAISE NOTICE 'Added column: advancing_stocks';
    ELSE
        RAISE NOTICE 'Column advancing_stocks already exists';
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                   WHERE table_schema = 'public' 
                   AND table_name = 'market_regime_history' 
                   AND column_name = 'declining_stocks') THEN
        ALTER TABLE market_regime_history ADD COLUMN declining_stocks INTEGER;
        RAISE NOTICE 'Added column: declining_stocks';
    ELSE
        RAISE NOTICE 'Column declining_stocks already exists';
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                   WHERE table_schema = 'public' 
                   AND table_name = 'market_regime_history' 
                   AND column_name = 'unchanged_stocks') THEN
        ALTER TABLE market_regime_history ADD COLUMN unchanged_stocks INTEGER;
        RAISE NOTICE 'Added column: unchanged_stocks';
    ELSE
        RAISE NOTICE 'Column unchanged_stocks already exists';
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                   WHERE table_schema = 'public' 
                   AND table_name = 'market_regime_history' 
                   AND column_name = 'new_highs') THEN
        ALTER TABLE market_regime_history ADD COLUMN new_highs INTEGER;
        RAISE NOTICE 'Added column: new_highs';
    ELSE
        RAISE NOTICE 'Column new_highs already exists';
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                   WHERE table_schema = 'public' 
                   AND table_name = 'market_regime_history' 
                   AND column_name = 'new_lows') THEN
        ALTER TABLE market_regime_history ADD COLUMN new_lows INTEGER;
        RAISE NOTICE 'Added column: new_lows';
    ELSE
        RAISE NOTICE 'Column new_lows already exists';
    END IF;
    
    -- Other potentially missing columns
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                   WHERE table_schema = 'public' 
                   AND table_name = 'market_regime_history' 
                   AND column_name = 'sector_relative_performance') THEN
        ALTER TABLE market_regime_history ADD COLUMN sector_relative_performance DECIMAL(8,6);
        RAISE NOTICE 'Added column: sector_relative_performance';
    ELSE
        RAISE NOTICE 'Column sector_relative_performance already exists';
    END IF;
    
    RAISE NOTICE 'Finished adding columns to market_regime_history';
END $$;

-- Step 2: Ensure unique constraint exists
DO $$
BEGIN
    -- Drop old constraint if it exists
    ALTER TABLE market_regime_history DROP CONSTRAINT IF EXISTS market_regime_analysis_date_key;
    
    -- Add new composite constraint if it doesn't exist
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'market_regime_history_date_created_key'
    ) THEN
        ALTER TABLE market_regime_history 
            ADD CONSTRAINT market_regime_history_date_created_key 
            UNIQUE(analysis_date, created_at);
        RAISE NOTICE 'Added unique constraint on (analysis_date, created_at)';
    ELSE
        RAISE NOTICE 'Unique constraint already exists';
    END IF;
END $$;

-- Step 3: Create or replace view for latest market regime
CREATE OR REPLACE VIEW market_regime AS
SELECT *
FROM market_regime_history
ORDER BY analysis_date DESC, created_at DESC
LIMIT 1;

RAISE NOTICE 'Created/updated market_regime view';

-- Step 4: Create or replace helper functions
CREATE OR REPLACE FUNCTION get_market_regime_at_date(target_date TIMESTAMPTZ)
RETURNS TABLE (
    market_regime VARCHAR,
    regime_confidence DECIMAL,
    market_risk_level VARCHAR,
    analysis_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        h.market_regime,
        h.regime_confidence,
        h.market_risk_level,
        h.analysis_date,
        h.created_at
    FROM market_regime_history h
    WHERE h.analysis_date <= target_date
    ORDER BY h.analysis_date DESC, h.created_at DESC
    LIMIT 1;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION get_market_regime_history(days_back INTEGER DEFAULT 90)
RETURNS TABLE (
    market_regime VARCHAR,
    regime_confidence DECIMAL,
    spy_price DECIMAL,
    spy_20d_change DECIMAL,
    spy_50d_change DECIMAL,
    vix DECIMAL,
    market_volatility DECIMAL,
    short_term_trend VARCHAR,
    medium_term_trend VARCHAR,
    long_term_trend VARCHAR,
    trend_strength DECIMAL,
    market_risk_level VARCHAR,
    market_risk_score DECIMAL,
    analysis_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        h.market_regime,
        h.regime_confidence,
        h.spy_price,
        h.spy_20d_change,
        h.spy_50d_change,
        h.vix,
        h.market_volatility,
        h.short_term_trend,
        h.medium_term_trend,
        h.long_term_trend,
        h.trend_strength,
        h.market_risk_level,
        h.market_risk_score,
        h.analysis_date,
        h.created_at
    FROM market_regime_history h
    WHERE h.analysis_date >= NOW() - (days_back || ' days')::INTERVAL
    ORDER BY h.analysis_date DESC, h.created_at DESC;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION get_market_regime_changes(days_back INTEGER DEFAULT 30)
RETURNS TABLE (
    old_regime VARCHAR,
    new_regime VARCHAR,
    old_confidence DECIMAL,
    new_confidence DECIMAL,
    change_date TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    WITH ranked_regimes AS (
        SELECT 
            h.market_regime,
            h.regime_confidence,
            h.analysis_date,
            LAG(h.market_regime) OVER (ORDER BY h.analysis_date, h.created_at) as prev_regime,
            LAG(h.regime_confidence) OVER (ORDER BY h.analysis_date, h.created_at) as prev_confidence
        FROM market_regime_history h
        WHERE h.analysis_date >= NOW() - (days_back || ' days')::INTERVAL
    )
    SELECT 
        rr.prev_regime as old_regime,
        rr.market_regime as new_regime,
        rr.prev_confidence as old_confidence,
        rr.regime_confidence as new_confidence,
        rr.analysis_date as change_date
    FROM ranked_regimes rr
    WHERE rr.prev_regime IS NOT NULL 
      AND rr.prev_regime != rr.market_regime
    ORDER BY rr.analysis_date DESC;
END;
$$ LANGUAGE plpgsql;

-- Step 5: Create indexes if they don't exist
CREATE INDEX IF NOT EXISTS idx_market_regime_history_date_created 
    ON market_regime_history(analysis_date DESC, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_market_regime_history_regime_date 
    ON market_regime_history(market_regime, analysis_date DESC);

-- Step 6: Add comments
COMMENT ON VIEW market_regime IS 'View showing the latest market regime analysis';
COMMENT ON FUNCTION get_market_regime_at_date IS 'Returns the market regime at a specific date';
COMMENT ON FUNCTION get_market_regime_history IS 'Returns market regime history for the past N days';
COMMENT ON FUNCTION get_market_regime_changes IS 'Returns market regime changes in the past N days';

-- Complete!
DO $$
BEGIN
    RAISE NOTICE '✅ Migration complete! market_regime_history table is ready.';
    RAISE NOTICE 'Run: SELECT * FROM market_regime; to see the latest regime';
END $$;

