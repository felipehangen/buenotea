-- Convert market_regime table to time-series history approach
-- This migration enables tracking of market regime changes over time
-- Note: market_regime is market-wide (no symbol), so we track by analysis_date

-- Step 1: Check if market_regime table exists, if not create it with all columns
DO $$
BEGIN
    IF NOT EXISTS (SELECT FROM pg_tables WHERE schemaname = 'public' AND tablename = 'market_regime') THEN
        CREATE TABLE market_regime (
            id SERIAL PRIMARY KEY,
            analysis_date TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            
            -- Market Regime Classification
            market_regime VARCHAR(20) NOT NULL,
            regime_confidence DECIMAL(5,4) NOT NULL,
            
            -- Market Context Data
            spy_price DECIMAL(10,2),
            spy_20d_change DECIMAL(8,6),
            spy_50d_change DECIMAL(8,6),
            vix DECIMAL(6,2),
            market_breadth DECIMAL(5,4),
            sector_relative_performance DECIMAL(8,6),
            
            -- Market Volatility Analysis
            market_volatility DECIMAL(8,6),
            volatility_percentile DECIMAL(5,2),
            
            -- Market Trend Analysis
            short_term_trend VARCHAR(20),
            medium_term_trend VARCHAR(20),
            long_term_trend VARCHAR(20),
            trend_strength DECIMAL(5,2),
            trend_consistency DECIMAL(5,2),
            
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
            fear_greed_index INTEGER,
            put_call_ratio DECIMAL(8,4),
            margin_debt_trend VARCHAR(20),
            
            -- Risk Assessment
            market_risk_level VARCHAR(20),
            market_risk_score DECIMAL(5,2),
            max_drawdown_risk DECIMAL(5,2),
            
            -- AI Analysis (Optional)
            chatgpt_regime_analysis TEXT,
            chatgpt_market_outlook TEXT,
            chatgpt_risk_assessment TEXT,
            chatgpt_model_used VARCHAR(50),
            chatgpt_analysis_timestamp TIMESTAMP WITH TIME ZONE,
            
            -- Analysis Metadata
            data_sources_used TEXT[],
            analysis_period_days INTEGER DEFAULT 250,
            computation_time_ms INTEGER,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        );
    END IF;
END $$;

-- Step 2: Add any missing columns to market_regime or market_regime_history table
DO $$
DECLARE
    target_table TEXT;
BEGIN
    -- Determine which table to use (prefer market_regime_history if it exists)
    IF EXISTS (SELECT FROM pg_tables WHERE schemaname = 'public' AND tablename = 'market_regime_history') THEN
        target_table := 'market_regime_history';
    ELSIF EXISTS (SELECT FROM pg_tables WHERE schemaname = 'public' AND tablename = 'market_regime') THEN
        target_table := 'market_regime';
    ELSE
        RETURN; -- No table exists, will be created in Step 1
    END IF;
    
    RAISE NOTICE 'Adding missing columns to table: %', target_table;
    
    -- Add columns that might be missing
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_schema = 'public' AND table_name = target_table AND column_name = 'advancing_stocks') THEN
        EXECUTE format('ALTER TABLE %I ADD COLUMN advancing_stocks INTEGER', target_table);
        RAISE NOTICE 'Added column advancing_stocks';
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_schema = 'public' AND table_name = target_table AND column_name = 'declining_stocks') THEN
        EXECUTE format('ALTER TABLE %I ADD COLUMN declining_stocks INTEGER', target_table);
        RAISE NOTICE 'Added column declining_stocks';
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_schema = 'public' AND table_name = target_table AND column_name = 'unchanged_stocks') THEN
        EXECUTE format('ALTER TABLE %I ADD COLUMN unchanged_stocks INTEGER', target_table);
        RAISE NOTICE 'Added column unchanged_stocks';
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_schema = 'public' AND table_name = target_table AND column_name = 'new_highs') THEN
        EXECUTE format('ALTER TABLE %I ADD COLUMN new_highs INTEGER', target_table);
        RAISE NOTICE 'Added column new_highs';
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_schema = 'public' AND table_name = target_table AND column_name = 'new_lows') THEN
        EXECUTE format('ALTER TABLE %I ADD COLUMN new_lows INTEGER', target_table);
        RAISE NOTICE 'Added column new_lows';
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_schema = 'public' AND table_name = target_table AND column_name = 'sector_relative_performance') THEN
        EXECUTE format('ALTER TABLE %I ADD COLUMN sector_relative_performance DECIMAL(8,6)', target_table);
        RAISE NOTICE 'Added column sector_relative_performance';
    END IF;
    
    RAISE NOTICE 'Finished adding columns to table: %', target_table;
END $$;

-- Step 3: Rename current table to market_regime_history (if not already renamed)
-- Skip this step entirely if market_regime_history already exists
DO $$
DECLARE
    history_exists BOOLEAN;
    regime_exists BOOLEAN;
BEGIN
    -- Check both tables
    SELECT EXISTS (SELECT FROM pg_tables WHERE schemaname = 'public' AND tablename = 'market_regime_history') INTO history_exists;
    SELECT EXISTS (SELECT FROM pg_tables WHERE schemaname = 'public' AND tablename = 'market_regime') INTO regime_exists;
    
    -- Decide what to do based on what exists
    IF history_exists THEN
        RAISE NOTICE 'market_regime_history already exists, skipping rename step';
    ELSIF regime_exists THEN
        ALTER TABLE market_regime RENAME TO market_regime_history;
        RAISE NOTICE 'Renamed market_regime to market_regime_history';
    ELSE
        RAISE NOTICE 'Neither market_regime nor market_regime_history exists, skipping rename';
    END IF;
END $$;

-- Step 4: Drop any existing unique constraint on analysis_date (if exists)
-- Only proceed if market_regime_history table exists
DO $$
BEGIN
    IF EXISTS (SELECT FROM pg_tables WHERE schemaname = 'public' AND tablename = 'market_regime_history') THEN
        ALTER TABLE market_regime_history 
            DROP CONSTRAINT IF EXISTS market_regime_analysis_date_key;
    END IF;
END $$;

-- Step 5: Add composite unique constraint (analysis_date + created_at)
-- This ensures we don't accidentally insert duplicate records for the same timestamp
DO $$
BEGIN
    IF EXISTS (SELECT FROM pg_tables WHERE schemaname = 'public' AND tablename = 'market_regime_history') THEN
        ALTER TABLE market_regime_history 
            DROP CONSTRAINT IF EXISTS market_regime_history_date_created_key;
        
        -- Only add constraint if it doesn't already exist
        IF NOT EXISTS (
            SELECT 1 FROM pg_constraint 
            WHERE conname = 'market_regime_history_date_created_key'
        ) THEN
            ALTER TABLE market_regime_history 
                ADD CONSTRAINT market_regime_history_date_created_key 
                UNIQUE(analysis_date, created_at);
        END IF;
    END IF;
END $$;

-- Step 6: Create view for latest market regime (backwards compatibility)
-- This view always shows the most recent market regime analysis
CREATE OR REPLACE VIEW market_regime AS
SELECT *
FROM market_regime_history
ORDER BY analysis_date DESC, created_at DESC
LIMIT 1;

-- Step 7: Create function to get market regime at a specific date
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

-- Step 8: Create function to get market regime history over the past N days
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

-- Step 9: Create function to detect regime changes (flips from Bull to Bear, etc.)
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

-- Step 10: Update the existing latest_market_regime view to use history table
DROP VIEW IF EXISTS latest_market_regime;
CREATE OR REPLACE VIEW latest_market_regime AS
SELECT *
FROM market_regime_history
ORDER BY analysis_date DESC, created_at DESC
LIMIT 1;

-- Step 11: Update the existing market_regime_history view (rename to avoid conflict)
DROP VIEW IF EXISTS market_regime_history_view;
CREATE OR REPLACE VIEW market_regime_history_view AS
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
FROM market_regime_history 
ORDER BY analysis_date DESC;

-- Step 12: Add helpful comments
COMMENT ON VIEW market_regime IS 'View showing the latest market regime analysis. Query this view for current market regime.';
COMMENT ON FUNCTION get_market_regime_at_date IS 'Returns the market regime that was valid at a specific date in the past';
COMMENT ON FUNCTION get_market_regime_history IS 'Returns all market regime analyses over the past N days';
COMMENT ON FUNCTION get_market_regime_changes IS 'Detects when the market regime changed (e.g., Bull → Bear)';

-- Step 13: Create indexes for performance on history table
CREATE INDEX IF NOT EXISTS idx_market_regime_history_date_created ON market_regime_history(analysis_date DESC, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_market_regime_history_regime_date ON market_regime_history(market_regime, analysis_date DESC);

-- Migration complete!
-- Usage examples:
-- 1. Get latest market regime: SELECT * FROM market_regime;
-- 2. Get market regime 30 days ago: SELECT * FROM get_market_regime_at_date(NOW() - INTERVAL '30 days');
-- 3. Get market regime history: SELECT * FROM get_market_regime_history(90);
-- 4. Find regime changes: SELECT * FROM get_market_regime_changes(30);
