-- Complete Database Rebuild Migration
-- This drops ALL existing tables and recreates them with the correct time-series structure
-- Created: 2025-11-10
-- Purpose: Clean slate for all 5 studies with historical tracking

-- ============================================================================
-- STEP 1: DROP EVERYTHING (Functions, Views, Tables)
-- ============================================================================

-- Drop all functions
DROP FUNCTION IF EXISTS get_invite_list_at_date(VARCHAR, TIMESTAMPTZ) CASCADE;
DROP FUNCTION IF EXISTS get_invite_list_history(INTEGER) CASCADE;
DROP FUNCTION IF EXISTS get_timing_at_date(VARCHAR, TIMESTAMPTZ) CASCADE;
DROP FUNCTION IF EXISTS get_timing_history(VARCHAR, INTEGER) CASCADE;
DROP FUNCTION IF EXISTS get_timing_changes(VARCHAR, INTEGER) CASCADE;
DROP FUNCTION IF EXISTS get_market_regime_at_date(TIMESTAMPTZ) CASCADE;
DROP FUNCTION IF EXISTS get_market_regime_history(INTEGER) CASCADE;
DROP FUNCTION IF EXISTS get_market_regime_changes(INTEGER) CASCADE;
DROP FUNCTION IF EXISTS get_fundamentals_at_date(VARCHAR, TIMESTAMPTZ) CASCADE;
DROP FUNCTION IF EXISTS get_fundamentals_history(VARCHAR, INTEGER) CASCADE;
DROP FUNCTION IF EXISTS get_fundamentals_changes(VARCHAR, INTEGER) CASCADE;
DROP FUNCTION IF EXISTS get_sentiment_at_date(VARCHAR, TIMESTAMPTZ) CASCADE;
DROP FUNCTION IF EXISTS get_sentiment_history(VARCHAR, INTEGER) CASCADE;
DROP FUNCTION IF EXISTS get_sentiment_changes(VARCHAR, INTEGER) CASCADE;

-- Drop all tables and views (handling both cases with exception handling)
DO $$ 
DECLARE
    r RECORD;
BEGIN
    -- Drop all objects that might be either views or tables
    FOR r IN (
        SELECT tablename FROM pg_tables 
        WHERE schemaname = 'public' 
        AND tablename IN (
            'invite_list', 'invite_list_history', 'latest_invite_list',
            'timing', 'timing_history', 'latest_timing',
            'market_regime', 'market_regime_history', 'latest_market_regime',
            'fundamentals', 'fundamentals_history', 'latest_fundamentals',
            'sentiment', 'sentiment_history', 'latest_sentiment',
            'regime'
        )
    ) LOOP
        EXECUTE 'DROP TABLE IF EXISTS ' || quote_ident(r.tablename) || ' CASCADE';
    END LOOP;
    
    FOR r IN (
        SELECT viewname FROM pg_views 
        WHERE schemaname = 'public' 
        AND viewname IN (
            'invite_list', 'latest_invite_list',
            'timing', 'latest_timing',
            'market_regime', 'latest_market_regime',
            'fundamentals', 'latest_fundamentals',
            'sentiment', 'latest_sentiment'
        )
    ) LOOP
        EXECUTE 'DROP VIEW IF EXISTS ' || quote_ident(r.viewname) || ' CASCADE';
    END LOOP;
END $$;

-- ============================================================================
-- STEP 2: CREATE INVITE_LIST_HISTORY TABLE
-- ============================================================================

CREATE TABLE invite_list_history (
    id BIGSERIAL PRIMARY KEY,
    symbol VARCHAR(10) NOT NULL,
    analysis_date TIMESTAMPTZ NOT NULL,
    
    -- Company info
    company_name VARCHAR(255) NOT NULL,
    sector VARCHAR(100),
    industry VARCHAR(100),
    market_cap BIGINT,
    current_price DECIMAL(10,2),
    
    -- Safety analysis results
    is_safe_to_trade BOOLEAN NOT NULL,
    safety_score DECIMAL(5,4),
    safety_reasoning TEXT,
    
    -- Basic financial health checks
    has_recent_earnings BOOLEAN NOT NULL,
    has_positive_revenue BOOLEAN NOT NULL,
    has_stable_price BOOLEAN NOT NULL,
    has_sufficient_volume BOOLEAN NOT NULL,
    has_analyst_coverage BOOLEAN NOT NULL,
    
    -- Risk assessment
    risk_level VARCHAR(20) NOT NULL,
    volatility_rating VARCHAR(20),
    liquidity_rating VARCHAR(20),
    
    -- Data quality and source tracking
    data_source VARCHAR(50) NOT NULL,
    last_updated TIMESTAMPTZ NOT NULL,
    data_freshness_score DECIMAL(5,4),
    
    -- Analysis metadata
    analysis_duration_ms INTEGER,
    warning_flags TEXT[],
    missing_data_components TEXT[],
    
    -- Raw API response data (JSON)
    raw_company_data JSONB,
    raw_financial_data JSONB,
    raw_price_data JSONB,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Unique constraint
    CONSTRAINT unique_invite_list_symbol_date UNIQUE (symbol, analysis_date)
);

CREATE INDEX idx_invite_list_history_symbol ON invite_list_history(symbol);
CREATE INDEX idx_invite_list_history_date ON invite_list_history(analysis_date DESC);
CREATE INDEX idx_invite_list_history_safe ON invite_list_history(is_safe_to_trade);
CREATE INDEX idx_invite_list_history_created ON invite_list_history(created_at DESC);

-- Create view for latest invite list
CREATE VIEW invite_list AS
SELECT DISTINCT ON (symbol)
    id, symbol, analysis_date, company_name, sector, industry,
    market_cap, current_price, is_safe_to_trade, safety_score, safety_reasoning,
    has_recent_earnings, has_positive_revenue, has_stable_price,
    has_sufficient_volume, has_analyst_coverage,
    risk_level, volatility_rating, liquidity_rating,
    data_source, last_updated, data_freshness_score,
    analysis_duration_ms, warning_flags, missing_data_components,
    raw_company_data, raw_financial_data, raw_price_data,
    created_at, updated_at
FROM invite_list_history
ORDER BY symbol, analysis_date DESC, created_at DESC;

-- Helper function
CREATE OR REPLACE FUNCTION get_invite_list_history(days_back INTEGER DEFAULT 90)
RETURNS TABLE (
    symbol VARCHAR,
    analysis_date TIMESTAMPTZ,
    is_safe_to_trade BOOLEAN,
    safety_score DECIMAL,
    risk_level VARCHAR,
    created_at TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    SELECT h.symbol, h.analysis_date, h.is_safe_to_trade, h.safety_score, h.risk_level, h.created_at
    FROM invite_list_history h
    WHERE h.analysis_date >= NOW() - (days_back || ' days')::INTERVAL
    ORDER BY h.analysis_date DESC, h.created_at DESC;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- STEP 3: CREATE TIMING_HISTORY TABLE
-- ============================================================================

CREATE TABLE timing_history (
    id SERIAL PRIMARY KEY,
    symbol VARCHAR(10) NOT NULL,
    analysis_date TIMESTAMPTZ NOT NULL,
    
    -- TTS Analysis Results
    tts_score DECIMAL(5,2) NOT NULL,
    trading_signal VARCHAR(20) NOT NULL,
    confidence_score DECIMAL(3,2) NOT NULL,
    
    -- Technical Indicators Scores
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
    
    -- Price Data
    price_data_points INTEGER NOT NULL,
    analysis_period_days INTEGER NOT NULL,
    current_price DECIMAL(10,2) NOT NULL,
    
    -- AI Analysis
    chatgpt_explanation TEXT,
    trading_suggestion TEXT,
    
    -- Metadata
    flags JSONB DEFAULT '[]'::jsonb,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Unique constraint
    CONSTRAINT unique_timing_symbol_date UNIQUE (symbol, analysis_date)
);

CREATE INDEX idx_timing_history_symbol ON timing_history(symbol);
CREATE INDEX idx_timing_history_date ON timing_history(analysis_date DESC);
CREATE INDEX idx_timing_history_score ON timing_history(tts_score);
CREATE INDEX idx_timing_history_signal ON timing_history(trading_signal);
CREATE INDEX idx_timing_history_created ON timing_history(created_at DESC);

-- Create view for latest timing
CREATE VIEW timing AS
SELECT DISTINCT ON (symbol) *
FROM timing_history
ORDER BY symbol, analysis_date DESC, created_at DESC;

-- Helper functions
CREATE OR REPLACE FUNCTION get_timing_history(target_symbol VARCHAR, days_back INTEGER DEFAULT 90)
RETURNS TABLE (
    id INTEGER,
    symbol VARCHAR,
    analysis_date TIMESTAMPTZ,
    tts_score DECIMAL,
    trading_signal VARCHAR,
    confidence_score DECIMAL,
    created_at TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    SELECT h.id, h.symbol, h.analysis_date, h.tts_score, h.trading_signal, h.confidence_score, h.created_at
    FROM timing_history h
    WHERE h.symbol = target_symbol
      AND h.analysis_date >= NOW() - (days_back || ' days')::INTERVAL
    ORDER BY h.analysis_date DESC, h.created_at DESC;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION get_timing_changes(target_symbol VARCHAR, days_back INTEGER DEFAULT 30)
RETURNS TABLE (
    symbol VARCHAR,
    old_signal VARCHAR,
    new_signal VARCHAR,
    change_date TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    WITH ranked_timing AS (
        SELECT
            h.symbol,
            h.trading_signal,
            h.analysis_date,
            LAG(h.trading_signal) OVER (ORDER BY h.analysis_date, h.created_at) as prev_signal
        FROM timing_history h
        WHERE h.symbol = target_symbol
          AND h.analysis_date >= NOW() - (days_back || ' days')::INTERVAL
    )
    SELECT rt.symbol, rt.prev_signal as old_signal, rt.trading_signal as new_signal, rt.analysis_date as change_date
    FROM ranked_timing rt
    WHERE rt.prev_signal IS NOT NULL AND rt.prev_signal != rt.trading_signal
    ORDER BY rt.analysis_date DESC;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- STEP 4: CREATE MARKET_REGIME_HISTORY TABLE
-- ============================================================================

CREATE TABLE market_regime_history (
    id SERIAL PRIMARY KEY,
    analysis_date TIMESTAMPTZ NOT NULL,
    
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
    
    -- AI Analysis
    chatgpt_regime_analysis TEXT,
    chatgpt_market_outlook TEXT,
    chatgpt_risk_assessment TEXT,
    chatgpt_model_used VARCHAR(50),
    chatgpt_analysis_timestamp TIMESTAMPTZ,
    
    -- Analysis Metadata
    data_sources_used TEXT[],
    analysis_period_days INTEGER DEFAULT 250,
    computation_time_ms INTEGER,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_market_regime_history_date ON market_regime_history(analysis_date DESC);
CREATE INDEX idx_market_regime_history_type ON market_regime_history(market_regime);
CREATE INDEX idx_market_regime_history_created ON market_regime_history(created_at DESC);

-- Create view for latest market regime
CREATE VIEW market_regime AS
SELECT *
FROM market_regime_history
ORDER BY analysis_date DESC, created_at DESC
LIMIT 1;

-- Helper functions
CREATE OR REPLACE FUNCTION get_market_regime_history(days_back INTEGER DEFAULT 90)
RETURNS TABLE (
    market_regime VARCHAR,
    regime_confidence DECIMAL,
    spy_price DECIMAL,
    vix DECIMAL,
    market_risk_level VARCHAR,
    analysis_date TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    SELECT h.market_regime, h.regime_confidence, h.spy_price, h.vix, h.market_risk_level, h.analysis_date
    FROM market_regime_history h
    WHERE h.analysis_date >= NOW() - (days_back || ' days')::INTERVAL
    ORDER BY h.analysis_date DESC, h.created_at DESC;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION get_market_regime_changes(days_back INTEGER DEFAULT 30)
RETURNS TABLE (
    old_regime VARCHAR,
    new_regime VARCHAR,
    change_date TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    WITH ranked_regimes AS (
        SELECT
            h.market_regime,
            h.analysis_date,
            LAG(h.market_regime) OVER (ORDER BY h.analysis_date, h.created_at) as prev_regime
        FROM market_regime_history h
        WHERE h.analysis_date >= NOW() - (days_back || ' days')::INTERVAL
    )
    SELECT rr.prev_regime as old_regime, rr.market_regime as new_regime, rr.analysis_date as change_date
    FROM ranked_regimes rr
    WHERE rr.prev_regime IS NOT NULL AND rr.prev_regime != rr.market_regime
    ORDER BY rr.analysis_date DESC;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- STEP 5: CREATE FUNDAMENTALS_HISTORY TABLE
-- ============================================================================

CREATE TABLE fundamentals_history (
    id BIGSERIAL PRIMARY KEY,
    symbol VARCHAR(10) NOT NULL,
    analysis_date TIMESTAMPTZ NOT NULL,
    
    -- Core scoring data
    fundamentals_score DECIMAL(5,2) NOT NULL,
    trading_signal VARCHAR(20) NOT NULL,
    confidence_score DECIMAL(3,2) NOT NULL,
    
    -- Component scores
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
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Unique constraint
    CONSTRAINT unique_fundamentals_symbol_date UNIQUE (symbol, analysis_date)
);

CREATE INDEX idx_fundamentals_history_symbol ON fundamentals_history(symbol);
CREATE INDEX idx_fundamentals_history_date ON fundamentals_history(analysis_date DESC);
CREATE INDEX idx_fundamentals_history_score ON fundamentals_history(fundamentals_score);
CREATE INDEX idx_fundamentals_history_signal ON fundamentals_history(trading_signal);
CREATE INDEX idx_fundamentals_history_created ON fundamentals_history(created_at DESC);

-- Create view for latest fundamentals
CREATE VIEW fundamentals AS
SELECT DISTINCT ON (symbol) *
FROM fundamentals_history
ORDER BY symbol, analysis_date DESC, created_at DESC;

-- Helper functions
CREATE OR REPLACE FUNCTION get_fundamentals_history(target_symbol VARCHAR, days_back INTEGER DEFAULT 365)
RETURNS TABLE (
    id BIGINT,
    symbol VARCHAR,
    analysis_date TIMESTAMPTZ,
    fundamentals_score DECIMAL,
    trading_signal VARCHAR,
    confidence_score DECIMAL,
    created_at TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    SELECT h.id, h.symbol, h.analysis_date, h.fundamentals_score, h.trading_signal, h.confidence_score, h.created_at
    FROM fundamentals_history h
    WHERE h.symbol = target_symbol
      AND h.analysis_date >= NOW() - (days_back || ' days')::INTERVAL
    ORDER BY h.analysis_date DESC, h.created_at DESC;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION get_fundamentals_changes(target_symbol VARCHAR, days_back INTEGER DEFAULT 90)
RETURNS TABLE (
    symbol VARCHAR,
    old_score DECIMAL,
    new_score DECIMAL,
    score_change DECIMAL,
    change_date TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    WITH ranked_fundamentals AS (
        SELECT
            h.symbol,
            h.fundamentals_score,
            h.analysis_date,
            LAG(h.fundamentals_score) OVER (ORDER BY h.analysis_date, h.created_at) as prev_score
        FROM fundamentals_history h
        WHERE h.symbol = target_symbol
          AND h.analysis_date >= NOW() - (days_back || ' days')::INTERVAL
    )
    SELECT rf.symbol, rf.prev_score as old_score, rf.fundamentals_score as new_score,
           (rf.fundamentals_score - rf.prev_score) as score_change, rf.analysis_date as change_date
    FROM ranked_fundamentals rf
    WHERE rf.prev_score IS NOT NULL AND ABS(rf.fundamentals_score - rf.prev_score) >= 5.0
    ORDER BY rf.analysis_date DESC;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- STEP 6: CREATE SENTIMENT_HISTORY TABLE
-- ============================================================================

CREATE TABLE sentiment_history (
    id BIGSERIAL PRIMARY KEY,
    symbol VARCHAR(10) NOT NULL,
    analysis_date TIMESTAMPTZ NOT NULL,
    
    -- Core QSS scoring data
    qss_score DECIMAL(5,3) NOT NULL,
    trading_signal VARCHAR(20) NOT NULL,
    confidence_score DECIMAL(3,2) NOT NULL,
    
    -- Component scores
    earnings_revisions_score DECIMAL(5,3) NOT NULL,
    relative_strength_score DECIMAL(5,3) NOT NULL,
    short_interest_score DECIMAL(5,3) NOT NULL,
    options_flow_score DECIMAL(5,3) NOT NULL,
    
    -- Component weights
    earnings_weight DECIMAL(3,2) NOT NULL DEFAULT 0.40,
    relative_strength_weight DECIMAL(3,2) NOT NULL DEFAULT 0.30,
    short_interest_weight DECIMAL(3,2) NOT NULL DEFAULT 0.20,
    options_flow_weight DECIMAL(3,2) NOT NULL DEFAULT 0.10,
    
    -- API endpoint information
    earnings_api_url TEXT,
    earnings_api_source VARCHAR(50),
    earnings_data_available BOOLEAN DEFAULT FALSE,
    
    price_data_api_url TEXT,
    price_data_api_source VARCHAR(50),
    price_data_available BOOLEAN DEFAULT FALSE,
    
    short_interest_api_url TEXT,
    short_interest_api_source VARCHAR(50),
    short_interest_data_available BOOLEAN DEFAULT FALSE,
    
    options_flow_api_url TEXT,
    options_flow_api_source VARCHAR(50),
    options_flow_data_available BOOLEAN DEFAULT FALSE,
    
    -- Raw API response data
    earnings_raw_data JSONB,
    price_data_raw_data JSONB,
    short_interest_raw_data JSONB,
    options_flow_raw_data JSONB,
    
    -- Data quality metrics
    data_coverage_percentage DECIMAL(5,2) NOT NULL,
    computation_time_ms INTEGER NOT NULL,
    data_points_count INTEGER NOT NULL,
    trend_direction DECIMAL(5,3) NOT NULL,
    data_freshness_score DECIMAL(3,2) NOT NULL,
    
    -- Warning flags
    warning_flags TEXT[],
    missing_data_components TEXT[],
    
    -- GPT explanation
    gpt_explanation TEXT NOT NULL,
    gpt_explanation_timestamp TIMESTAMPTZ,
    
    -- Technical indicators
    rsi_14 DECIMAL(8,2),
    rsi_source VARCHAR(50),
    
    -- Market context
    market_benchmark_return DECIMAL(8,4),
    sector_benchmark_return DECIMAL(8,4),
    relative_to_market DECIMAL(8,4),
    relative_to_sector DECIMAL(8,4),
    
    -- Earnings data
    current_eps_estimate DECIMAL(10,2),
    previous_eps_estimate DECIMAL(10,2),
    eps_change_percentage DECIMAL(8,4),
    current_revenue_estimate BIGINT,
    previous_revenue_estimate BIGINT,
    revenue_change_percentage DECIMAL(8,4),
    analyst_count INTEGER,
    
    -- Price data
    current_price DECIMAL(10,2),
    price_15d_ago DECIMAL(10,2),
    price_30d_ago DECIMAL(10,2),
    return_15d DECIMAL(8,4),
    return_30d DECIMAL(8,4),
    volume_ratio DECIMAL(8,4),
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Unique constraint
    CONSTRAINT unique_sentiment_symbol_date UNIQUE (symbol, analysis_date)
);

CREATE INDEX idx_sentiment_history_symbol ON sentiment_history(symbol);
CREATE INDEX idx_sentiment_history_date ON sentiment_history(analysis_date DESC);
CREATE INDEX idx_sentiment_history_score ON sentiment_history(qss_score);
CREATE INDEX idx_sentiment_history_signal ON sentiment_history(trading_signal);
CREATE INDEX idx_sentiment_history_created ON sentiment_history(created_at DESC);

-- Create view for latest sentiment
CREATE VIEW sentiment AS
SELECT DISTINCT ON (symbol) *
FROM sentiment_history
ORDER BY symbol, analysis_date DESC, created_at DESC;

-- Helper functions
CREATE OR REPLACE FUNCTION get_sentiment_history(target_symbol VARCHAR, days_back INTEGER DEFAULT 90)
RETURNS TABLE (
    id BIGINT,
    symbol VARCHAR,
    analysis_date TIMESTAMPTZ,
    qss_score DECIMAL,
    trading_signal VARCHAR,
    confidence_score DECIMAL,
    created_at TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    SELECT h.id, h.symbol, h.analysis_date, h.qss_score, h.trading_signal, h.confidence_score, h.created_at
    FROM sentiment_history h
    WHERE h.symbol = target_symbol
      AND h.analysis_date >= NOW() - (days_back || ' days')::INTERVAL
    ORDER BY h.analysis_date DESC, h.created_at DESC;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION get_sentiment_changes(target_symbol VARCHAR, days_back INTEGER DEFAULT 30)
RETURNS TABLE (
    symbol VARCHAR,
    old_score DECIMAL,
    new_score DECIMAL,
    score_change DECIMAL,
    change_date TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    WITH ranked_sentiment AS (
        SELECT
            h.symbol,
            h.qss_score,
            h.analysis_date,
            LAG(h.qss_score) OVER (ORDER BY h.analysis_date, h.created_at) as prev_score
        FROM sentiment_history h
        WHERE h.symbol = target_symbol
          AND h.analysis_date >= NOW() - (days_back || ' days')::INTERVAL
    )
    SELECT rs.symbol, rs.prev_score as old_score, rs.qss_score as new_score,
           (rs.qss_score - rs.prev_score) as score_change, rs.analysis_date as change_date
    FROM ranked_sentiment rs
    WHERE rs.prev_score IS NOT NULL AND ABS(rs.qss_score - rs.prev_score) >= 0.10
    ORDER BY rs.analysis_date DESC;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- STEP 7: ADD COMMENTS
-- ============================================================================

COMMENT ON TABLE invite_list_history IS 'Time-series storage of invite list analysis. Tracks which stocks are safe to trade.';
COMMENT ON TABLE timing_history IS 'Time-series storage of timing (TTS) analysis. Technical trading signals.';
COMMENT ON TABLE market_regime_history IS 'Time-series storage of market regime analysis. Overall market conditions.';
COMMENT ON TABLE fundamentals_history IS 'Time-series storage of fundamentals analysis. Financial health scores.';
COMMENT ON TABLE sentiment_history IS 'Time-series storage of sentiment (QSS) analysis. Market sentiment scores.';

COMMENT ON VIEW invite_list IS 'Latest invite list analysis per symbol.';
COMMENT ON VIEW timing IS 'Latest timing analysis per symbol.';
COMMENT ON VIEW market_regime IS 'Latest market regime analysis.';
COMMENT ON VIEW fundamentals IS 'Latest fundamentals analysis per symbol.';
COMMENT ON VIEW sentiment IS 'Latest sentiment analysis per symbol.';

-- ============================================================================
-- MIGRATION COMPLETE!
-- ============================================================================
-- 
-- All 5 studies now have:
-- 1. History tables (invite_list_history, timing_history, market_regime_history, fundamentals_history, sentiment_history)
-- 2. Views for latest data (invite_list, timing, market_regime, fundamentals, sentiment)
-- 3. Helper functions for querying history and changes
-- 4. Unique constraints to prevent duplicates
-- 5. Proper indexes for performance
--
-- Usage:
-- - Query latest: SELECT * FROM timing WHERE symbol = 'AAPL';
-- - Query history: SELECT * FROM get_timing_history('AAPL', 90);
-- - Find changes: SELECT * FROM get_timing_changes('AAPL', 30);

