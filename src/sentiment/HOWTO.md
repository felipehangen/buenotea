# Sentiment Score Implementation Guide

## Overview

This guide explains how to implement and use the Quantitative Sentiment Score (QSS) for making buy/sell decisions on stocks. The QSS combines multiple data sources into a single score between [-1, +1] that indicates market sentiment and momentum.

## 🎯 Score Formula

```
QSS = 0.40 × Earnings_Revisions + 0.30 × Relative_Strength + 0.20 × Short_Interest + 0.10 × Options_Flow
```

**Score Interpretation:**
- **+0.6 to +1.0**: 🟢 **BUY** - Very Bullish signals
- **+0.2 to +0.6**: 🟡 **WEAK BUY** - Mildly Bullish conditions
- **-0.2 to +0.2**: ⚪ **HOLD** - Neutral, mixed signals
- **-0.6 to -0.2**: 🟠 **WEAK SELL** - Mildly Bearish conditions
- **-1.0 to -0.6**: 🔴 **SELL** - Very Bearish signals

## 📊 Required Data Sources

### 1. Earnings Revisions (40% Weight) - Multi-Source API Integration
**Purpose**: Track analyst estimate changes over time

**Data Sources (in order of preference):**
- **Alpha Vantage**: Earnings Estimates (`EARNINGS_ESTIMATES`) + Earnings History (`EARNINGS`)
- **Financial Modeling Prep**: Analyst Estimates + Income Statements
- **Finnhub**: Company Earnings Estimates

**Key Endpoints:**
```
# Alpha Vantage (Primary)
GET https://www.alphavantage.co/query?function=EARNINGS_ESTIMATES&symbol={SYMBOL}&apikey={ALPHA_VANTAGE_API_KEY}
GET https://www.alphavantage.co/query?function=EARNINGS&symbol={SYMBOL}&apikey={ALPHA_VANTAGE_API_KEY}

# Financial Modeling Prep (Fallback)
GET https://financialmodelingprep.com/api/v3/analyst-estimates/{SYMBOL}?apikey={FMP_API_KEY}
GET https://financialmodelingprep.com/api/v3/income-statement/{SYMBOL}?apikey={FMP_API_KEY}

# Finnhub (Additional)
GET https://finnhub.io/api/v1/company-earnings-estimates?symbol={SYMBOL}&token={FINNHUB_API_KEY}
```

**Key Data Points:**
- `estimatedEps` / `reportedEPS` - Earnings per share estimates
- `totalRevenue` / `revenue` - Revenue estimates and actuals
- `date` - Estimate publication date

**Processing Steps:**
1. Try Alpha Vantage earnings estimates first
2. Fallback to Alpha Vantage earnings history if estimates unavailable
3. Fallback to FMP analyst estimates
4. Calculate EPS change: `(current_eps - previous_eps) / previous_eps`
5. Calculate revenue change: `(current_revenue - previous_revenue) / previous_revenue`
6. Normalize to [-1, 1] range for sentiment scoring

### 2. Relative Strength (30% Weight) - Multi-Source API Integration
**Purpose**: Measure stock performance vs market and sector using RSI and momentum analysis

**Data Sources:**
- **Financial Modeling Prep**: Historical price data + Technical indicators
- **Alpha Vantage**: Quote data + Technical indicators

**Key Endpoints:**
```
# Financial Modeling Prep (Primary)
GET https://financialmodelingprep.com/api/v3/historical-price-full/{SYMBOL}?apikey={FMP_API_KEY}
GET https://financialmodelingprep.com/api/v3/quote/{SYMBOL}?apikey={FMP_API_KEY}
GET https://financialmodelingprep.com/api/v3/technical-indicator/{SYMBOL}?period=14&type=rsi&apikey={FMP_API_KEY}

# Alpha Vantage (Fallback)
GET https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={SYMBOL}&apikey={ALPHA_VANTAGE_API_KEY}
```

**Key Data Points:**
- `close` - Historical closing prices (30+ days for RSI calculation)
- `volume` - Trading volume data
- `changesPercentage` - Current price change percentage

**Processing Steps:**
1. Fetch 30 days of historical price data
2. Calculate 14-period RSI using closing prices
3. Calculate price momentum (15-day and 30-day returns)
4. Convert RSI to sentiment: RSI > 70 = bearish, RSI < 30 = bullish
5. Combine RSI sentiment (70%) + momentum sentiment (30%)
6. Normalize to [-1, 1] range
### 3. Market Benchmark Analysis - Comprehensive Market Data
**Purpose**: Calculate relative performance vs market and sector benchmarks

**Data Sources:**
- **Financial Modeling Prep**: S&P 500 (SPY) and sector ETF data
- **Sector Mapping**: Technology (XLK), Financials (XLF), Healthcare (XLV), etc.

**Key Endpoints:**
```
# Market Benchmarks
GET https://financialmodelingprep.com/api/v3/historical-price-full/SPY?apikey={FMP_API_KEY}

# Sector ETFs (based on stock symbol)
GET https://financialmodelingprep.com/api/v3/historical-price-full/XLK?apikey={FMP_API_KEY}  # Tech sector
GET https://financialmodelingprep.com/api/v3/historical-price-full/XLF?apikey={FMP_API_KEY}  # Financials
GET https://financialmodelingprep.com/api/v3/historical-price-full/XLV?apikey={FMP_API_KEY}  # Healthcare
```

**Sector Mapping:**
- **Technology**: AAPL, MSFT, GOOGL, NVDA, TSLA, META, AMZN → XLK
- **Financials**: JPM, BAC, WFC, GS → XLF
- **Healthcare**: JNJ, PFE, UNH → XLV
- **Energy**: XOM, CVX, COP → XLE
- **Consumer**: KO, PEP, WMT → XLP/XLY

**Processing Steps:**
1. Fetch 30 days of S&P 500 (SPY) historical data
2. Fetch 30 days of relevant sector ETF data
3. Calculate 15-day returns for stock, market, and sector
4. Calculate relative performance: `stock_return - market_return`
5. Calculate sector performance: `stock_return - sector_return`
6. Store benchmark data for comprehensive analysis

### 4. Short Interest & Options Flow (20% + 10% Weight) - Future Premium APIs
**Purpose**: Track institutional sentiment and options activity

**Current Status**: Using analyst recommendations and news sentiment as proxies
**Future Implementation**: Premium APIs required for real short interest and options flow data

**Proxies Currently Used:**
- **Finnhub**: Company news sentiment analysis
- **FMP**: Analyst stock recommendations
- **Alpha Vantage**: Company news and sentiment

**Future Premium APIs:**
- **EODHD**: Short interest data ($9.99/month)
- **Unusual Whales**: Options flow data ($29.99/month)
- **Polygon**: Options data (premium tier)

**Key Data Points:**
- Daily OHLCV data for stock, SPY, and sector ETF (XLK)
- RSI(14) technical indicator
- Volume data for volume ratio calculation

**Processing Steps:**
1. Fetch 100 days of price data for stock, SPY, and XLK
2. Calculate RSI(14): `RSI = 100 - (100 / (1 + RS))` where `RS = avg_gain / avg_loss`
3. Calculate volume ratio: `recent_5day_avg_volume / 20day_avg_volume`
4. Calculate 1M return: `(price_15d_ago - price_now) / price_15d_ago`
5. Calculate 3M return: `(price_30d_ago - price_now) / price_30d_ago`
6. Calculate relative performance: `stock_return - benchmark_return`
7. Apply volatility adjustment using realized volatility
8. Use conservative approach: `min(vs_market, vs_sector)`
9. Apply exponential decay: `value * exp(-ln(2) / 20)`

### 3. Short Interest (20% Weight) - FMP API (Premium Required)
**Purpose**: Track short selling activity and sentiment

**Required Endpoints:**
```
GET https://financialmodelingprep.com/api/v3/short-interest/{SYMBOL}?apikey={FMP_API_KEY}
```

**Key Data Points:**
- `short_volume` - Daily short selling volume
- `total_volume` - Total trading volume
- `date` - Trading date

**Processing Steps:**
1. Fetch 120 days of short volume data
2. Calculate short ratio: `short_volume / total_volume`
3. Calculate recent vs historical: `recent_avg - historical_avg`
4. Apply sector z-score normalization
5. Use trend-gated logic: bullish only in uptrends, bearish only in downtrends
6. Apply exponential decay: `value * exp(-ln(2) / 20)`

### 4. Options Flow (10% Weight) - Polygon.io API (Premium Required)
**Purpose**: Analyze options trading activity for sentiment

**Required Endpoints:**
```
GET https://api.polygon.io/v2/snapshot/options/{SYMBOL}?apikey={POLYGON_API_KEY}
```

**Key Data Points:**
- `call_volume` - Call option trading volume
- `put_volume` - Put option trading volume
- `call_premium` - Call premium volume
- `put_premium` - Put premium volume
- `dte` - Days to expiration

**Processing Steps:**
1. Fetch 60 days of options flow data
2. Filter out 0DTE/1DTE options
3. Check for activity spikes: `current_volume > 2 * avg_volume`
4. Calculate premium skew: `call_premium / (call_premium + put_premium)`
5. Verify trade size significance (top 10% percentile)
6. Apply trend-gated logic with conflict detection
7. Apply exponential decay: `value * exp(-ln(2) / 5)` (5-day half-life)

## 🔧 Implementation Steps

### Step 1: Set Up Data Collection
```rust
use sentiment_backend::sentiment::QSSCalculator;

// Initialize QSS calculator (automatically uses API keys from environment variables)
let calculator = QSSCalculator::new();

// The calculator will automatically use API keys from these environment variables:
// - POLYGON_API_KEY
// - FMP_API_KEY  
// - FINNHUB_API_KEY
// - ALPHA_VANTAGE_API_KEY

// If no API keys are found, the system will use mock data for demonstration
```

### Step 2: Fetch Earnings Data
```rust
// Fetch analyst estimates from FMP
let estimates = fmp_client.get_analyst_estimates("AAPL").await?;

// Calculate revisions
let eps_change = calculate_eps_change(&estimates, 30)?;
let revenue_change = calculate_revenue_change(&estimates, 30)?;
let rec_change = calculate_recommendation_change(&estimates, 90)?;

// Apply processing
let eps_score = normalize_and_decay(eps_change, 20.0)?;
let revenue_score = normalize_and_decay(revenue_change, 20.0)?;
let rec_score = normalize_and_decay(rec_change, 20.0)?;

let revisions_score = 0.5 * eps_score + 0.35 * revenue_score + 0.15 * rec_score;
```

### Step 3: Calculate Relative Strength
```rust
// Fetch price data
let stock_prices = fmp_client.get_historical_prices("AAPL", 100).await?;
let spy_prices = fmp_client.get_historical_prices("SPY", 100).await?;
let xlk_prices = fmp_client.get_historical_prices("XLK", 100).await?;

// Calculate RSI
let rsi = calculate_rsi(&stock_prices, 14)?;

// Calculate relative performance
let vs_spy_1m = calculate_relative_return(&stock_prices, &spy_prices, 15)?;
let vs_sector_1m = calculate_relative_return(&stock_prices, &xlk_prices, 15)?;

// Use conservative approach
let relative_strength = vs_spy_1m.min(vs_sector_1m);
let rs_score = normalize_and_decay(relative_strength, 20.0)?;
```

### Step 4: Process Short Interest (if available)
```rust
// Fetch short volume data
let short_data = fmp_client.get_short_interest("AAPL", 120).await?;

// Calculate short ratio changes
let short_ratio_change = calculate_short_ratio_change(&short_data)?;

// Apply trend gating
let shorts_score = if is_uptrend(&stock_prices) {
    normalize_and_decay(short_ratio_change, 20.0)?
} else {
    0.0 // Only use in uptrends
};
```

### Step 5: Analyze Options Flow (if available)
```rust
// Fetch options data
let options_data = polygon_client.get_options_flow("AAPL", 60).await?;

// Calculate premium skew
let premium_skew = calculate_premium_skew(&options_data)?;

// Check for unusual activity
let unusual_activity = detect_unusual_options_activity(&options_data)?;

let options_score = if unusual_activity {
    normalize_and_decay(premium_skew, 5.0)?
} else {
    0.0
};
```

### Step 6: Calculate Final QSS Score
```rust
// Calculate weighted QSS score
let qss = 0.40 * revisions_score + 
          0.30 * rs_score + 
          0.20 * shorts_score + 
          0.10 * options_score;

// Apply final normalization
let final_qss = qss.tanh(); // Ensures [-1, +1] range
```

### Step 7: Generate Trading Signal
```rust
let signal = match final_qss {
    qss if qss >= 0.6 => TradingSignal::StrongBuy,
    qss if qss >= 0.2 => TradingSignal::WeakBuy,
    qss if qss >= -0.2 => TradingSignal::Hold,
    qss if qss >= -0.6 => TradingSignal::WeakSell,
    _ => TradingSignal::StrongSell,
};

// Generate flags for context
let mut flags = Vec::new();
if earnings_within_2_days { flags.push("earnings_window"); }
if no_estimates_available { flags.push("no_estimates"); }
if insufficient_data { flags.push("insufficient_data"); }
```

## 📋 Data Requirements Summary

| Component | Weight | Data Source | Required Fields | Update Frequency |
|-----------|--------|-------------|-----------------|------------------|
| **Earnings Revisions** | 40% | FMP | EPS, Revenue, Recommendations | Daily |
| **Relative Strength** | 30% | FMP + Alpha Vantage | OHLCV, RSI, Volume | Daily |
| **Short Interest** | 20% | FMP Premium | Short Volume, Total Volume | Daily |
| **Options Flow** | 10% | Polygon Premium | Call/Put Volume, Premiums | Daily |

## 🚨 Important Considerations

### Data Quality Flags
Always check for these flags that indicate data quality issues:
- `earnings_window`: Earnings announcement within 2 days
- `no_estimates`: Insufficient analyst estimates
- `insufficient_data`: Less than minimum required data points
- `low_confidence`: High dispersion in estimates

### Fallback Behavior
- Missing components default to 0.0 score
- System continues to function with available data
- Weights are re-normalized for missing components
- Flags indicate data availability issues

### API Limitations
- **FMP**: 250 calls/day on premium plan
- **Alpha Vantage**: 75 calls/minute on paid plan
- **Polygon.io**: 5 calls/minute on developer plan
- **Finnhub**: 60 calls/minute on paid plans

### Rate Limiting
Implement proper rate limiting and caching:
```rust
// Cache data for 24 hours to minimize API calls
let cached_data = cache.get_or_fetch(key, || fetch_data()).await?;

// Respect rate limits
tokio::time::sleep(Duration::from_millis(1000)).await;
```

## 🎯 Trading Strategy Integration

### Position Sizing
```rust
let position_size = match signal {
    TradingSignal::StrongBuy => 0.10,  // 10% of portfolio
    TradingSignal::WeakBuy => 0.05,    // 5% of portfolio
    TradingSignal::Hold => 0.0,        // No change
    TradingSignal::WeakSell => -0.05,  // Reduce by 5%
    TradingSignal::StrongSell => -0.10, // Reduce by 10%
};
```

### Risk Management
```rust
// Only trade if confidence is high
if confidence_score < 0.7 {
    return TradingSignal::Hold;
}

// Check for conflicting signals
if options_flow_contradicts_price_trend {
    flags.push("flow_conflict");
    return TradingSignal::Hold;
}
```

### Portfolio Integration
```rust
// Calculate portfolio-level sentiment
let portfolio_sentiment = calculate_weighted_portfolio_sentiment(positions, qss_scores)?;

// Adjust overall market exposure based on portfolio sentiment
let market_exposure = if portfolio_sentiment > 0.3 {
    1.2 // Increase market exposure
} else if portfolio_sentiment < -0.3 {
    0.8 // Reduce market exposure
} else {
    1.0 // Neutral exposure
};
```

## 🔍 Example Implementation

### Available Examples:

**`examples/sentiment_to_supabase.rs`** - Complete sentiment analysis with database storage:
- Real API calls to Alpha Vantage, FMP, and Finnhub
- Comprehensive data collection (EPS, revenue, RSI, benchmarks)
- Supabase database integration
- GPT-generated explanations
- Full sentiment analysis for multiple stocks

**`examples/check_detailed_apple_data.rs`** - Detailed data inspection:
- View all collected database fields
- Check data quality and completeness
- Verify API data sources
- Monitor null vs populated fields

**`examples/test_real_api.rs`** - API testing:
- Test individual stock sentiment calculation
- Verify API connectivity
- Check data collection performance

**`examples/sentiment_example.rs`** - Basic sentiment calculation:
- Simple sentiment score calculation
- Component breakdown
- Trading signal interpretation

### Quick Start:
```bash
# Run complete sentiment analysis and save to database
cargo run --example sentiment_to_supabase

# Check detailed data for Apple stock
cargo run --example check_detailed_apple_data

# Test API calls for single stock
cargo run --example test_real_api
```

## 📈 Performance Monitoring

Track these metrics to ensure system effectiveness:
- **Accuracy**: Compare QSS predictions to actual stock performance
- **Response Time**: Monitor API call latencies
- **Data Quality**: Track missing data and error rates
- **Signal Quality**: Measure signal-to-noise ratio

The QSS system provides a quantitative framework for combining multiple sentiment and momentum indicators into a single actionable score, helping you make more informed buy/sell decisions based on comprehensive market data analysis.
