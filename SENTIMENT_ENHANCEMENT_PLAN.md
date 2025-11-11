# Sentiment (QSS) Study Enhancement Plan

## Part 1: Add API Tracking (Immediate)

### Current State
The calculator makes API calls but doesn't return URLs or raw responses in `QSSResult`.

### Changes Needed

#### 1. Add API Tracking to QSSResult
```rust
// In crates/studies/sentiment/src/models.rs

pub struct QSSResult {
    pub symbol: String,
    pub qss_score: f64,
    pub trading_signal: TradingSignal,
    pub components: QSSComponents,
    pub flags: Vec<String>,
    pub confidence_score: f64,
    pub timestamp: DateTime<Utc>,
    pub meta: QSSMeta,
    pub api_tracking: QSSApiTracking,  // NEW
}

pub struct QSSApiTracking {
    // Earnings API tracking
    pub earnings_api_url: Option<String>,
    pub earnings_api_source: String,  // "FMP" or "Alpha Vantage"
    pub earnings_raw_data: Option<serde_json::Value>,
    
    // Price data API tracking
    pub price_data_api_url: Option<String>,
    pub price_data_api_source: String,  // "FMP"
    pub price_data_raw_data: Option<serde_json::Value>,
    
    // Short interest API tracking
    pub short_interest_api_url: Option<String>,
    pub short_interest_api_source: String,  // "Finnhub" or "None"
    pub short_interest_raw_data: Option<serde_json::Value>,
    
    // Options flow API tracking
    pub options_flow_api_url: Option<String>,
    pub options_flow_api_source: String,  // "FMP" or "None"
    pub options_flow_raw_data: Option<serde_json::Value>,
}
```

#### 2. Update Calculator to Return API Tracking
Modify `calculate_qss()` to collect and return API URLs and raw responses.

#### 3. Update Storage Helper
Modify `create_sentiment_record_with_tracking()` to use returned API data instead of defaults.

## Part 2: Additional Sentiment Indicators (Enhancement)

Based on available APIs (Finnhub, Alpha Vantage, FMP), here are valuable sentiment indicators we can add:

### 🎯 High-Value Additions (Recommend)

#### 1. **Insider Trading Sentiment** (Finnhub)
**Why**: Insiders have best information about company prospects
**API**: Finnhub `/stock/insider-transactions` & `/stock/insider-sentiment`
**Data**:
- Net insider buying/selling
- Insider transaction volume
- Insider sentiment score (MSPR - Money-Weighted Sentiment Percentage Ratio)

**Weight Suggestion**: 10-15% (high signal)
**Implementation**:
```rust
pub struct InsiderSentiment {
    pub mspr: f64,  // Monthly Share Purchase Ratio
    pub change: f64,  // Change from previous month
    pub net_purchases: i64,  // Net shares purchased
}
```

#### 2. **Social Sentiment** (Finnhub)
**Why**: Reddit/Twitter sentiment drives modern market movements
**API**: Finnhub `/social-sentiment`
**Data**:
- Reddit mentions and sentiment
- Twitter mentions and sentiment
- Stocktwits sentiment

**Weight Suggestion**: 5-10% (volatile but relevant)
**Implementation**:
```rust
pub struct SocialSentiment {
    pub reddit_sentiment: f64,  // -1 to 1
    pub twitter_sentiment: f64,
    pub mention_count: i32,
    pub sentiment_trend: String,  // "increasing", "decreasing", "stable"
}
```

#### 3. **News Sentiment** (Finnhub)
**Why**: News drives investor perception and short-term movements
**API**: Finnhub `/news-sentiment`
**Data**:
- Buzz (article mentions)
- Sentiment score
- Article score (quality)
- Sentiment trend

**Weight Suggestion**: 10-15% (strong short-term signal)
**Implementation**:
```rust
pub struct NewsSentiment {
    pub sentiment_score: f64,  // 0 to 1
    pub buzz: f64,  // Article volume score
    pub article_sentiment: f64,  // Weighted by source quality
    pub week_change: f64,  // Trend
}
```

#### 4. **Institutional Ownership Changes** (Finnhub)
**Why**: Smart money follows institutional flows
**API**: Finnhub `/stock/institutional-ownership` & `/stock/ownership-change`
**Data**:
- Net institutional buying
- Ownership concentration
- Recent 13F filings

**Weight Suggestion**: 5-10% (lagging but reliable)
**Implementation**:
```rust
pub struct InstitutionalSentiment {
    pub ownership_percentage: f64,
    pub quarter_change: f64,
    pub institution_count: i32,
    pub concentrated: bool,  // Top 10 own >50%
}
```

#### 5. **Analyst Upgrade/Downgrade Trends** (Finnhub)
**Why**: Analyst changes signal shifting consensus
**API**: Finnhub `/recommendation-trends`
**Data**:
- Strong buy count
- Buy count
- Hold count
- Sell count
- Strong sell count
- Trend direction

**Weight Suggestion**: 10-15% (professional consensus)
**Implementation**:
```rust
pub struct AnalystTrend {
    pub consensus: String,  // "Buy", "Hold", "Sell"
    pub strong_buy: i32,
    pub buy: i32,
    pub hold: i32,
    pub sell: i32,
    pub strong_sell: i32,
    pub month_change: i32,  // Net upgrades/downgrades
}
```

#### 6. **Price Target Sentiment** (Finnhub)
**Why**: Shows analyst expectations vs current price
**API**: Finnhub `/price-target`
**Data**:
- Target high
- Target low
- Target median
- Upside potential

**Weight Suggestion**: 5-10% (forward-looking)
**Implementation**:
```rust
pub struct PriceTargetSentiment {
    pub median_target: f64,
    pub upside_percentage: f64,  // (target - current) / current
    pub analyst_count: i32,
    pub target_trend: String,  // "increasing", "decreasing", "stable"
}
```

### 📊 Medium-Value Additions (Consider)

#### 7. **Earnings Surprise Trend** (FMP + Alpha Vantage)
**Why**: Consistent beats/misses signal company trajectory
**Current**: We fetch this data but don't use it for sentiment
**Enhancement**: Add 3-quarter trend analysis
**Weight Suggestion**: 5-10%

#### 8. **Revenue Surprise Trend** (FMP)
**Why**: Top-line growth surprises show momentum
**API**: `/company-revenue-estimates`
**Weight Suggestion**: 5%

#### 9. **Sector Rotation Signals** (Current data + enhancement)
**Why**: Sector momentum drives individual stocks
**Current**: We calculate relative to sector
**Enhancement**: Add sector momentum score
**Weight Suggestion**: 5%

### 🔄 Data We Already Have But Don't Use Fully

1. **Analyst Count** - Currently tracked but not weighted
2. **Earnings Surprises** - Fetched but not analyzed for trends
3. **Revenue Data** - Fetched but underutilized
4. **Volume Ratio** - Calculated but not integrated into scoring

## Recommended New QSS Formula

### Current (4 components):
```
QSS = 0.40 * earnings_revisions
    + 0.30 * relative_strength
    + 0.20 * short_interest
    + 0.10 * options_flow
```

### Proposed (10 components):
```
QSS = 0.20 * earnings_revisions       (was 40%, still important)
    + 0.15 * relative_strength         (was 30%, still core)
    + 0.10 * short_interest            (was 20%, keep if available)
    + 0.05 * options_flow              (was 10%, reduce)
    + 0.15 * news_sentiment            (NEW - strong short-term signal)
    + 0.10 * insider_sentiment         (NEW - high quality signal)
    + 0.10 * analyst_trends            (NEW - professional consensus)
    + 0.05 * institutional_flows       (NEW - smart money)
    + 0.05 * social_sentiment          (NEW - retail sentiment)
    + 0.05 * price_target_upside       (NEW - forward looking)
```

**Total**: 100%

### Rationale for Weight Changes:
1. **Diluted existing components** to make room for new signals
2. **Earnings still highest** (20%) - fundamentals matter most
3. **News sentiment high** (15%) - drives short-term action
4. **Relative strength** remains core (15%) - momentum works
5. **Multiple sentiment sources** (40% total) - diversification
6. **Professional signals** (25% total) - insiders + analysts + institutions
7. **Retail sentiment** (5%) - acknowledgment of Reddit era

## Implementation Phases

### Phase 1: API Tracking (Immediate - Today)
**Time**: 2-3 hours
**Changes**:
1. Add `QSSApiTracking` struct
2. Modify calculator to collect API URLs/responses
3. Update storage helper
4. Test with 10 stocks
5. Verify database stores all URLs

**Result**: Can debug which APIs were called for each analysis

### Phase 2: News Sentiment (High Priority)
**Time**: 3-4 hours
**Changes**:
1. Add Finnhub news sentiment fetching
2. Calculate sentiment score
3. Integrate into QSS formula
4. Update weights
5. Test

**Result**: QSS = 5 components (add 15%)

### Phase 3: Insider + Institutional (High Priority)
**Time**: 4-5 hours
**Changes**:
1. Add Finnhub insider transaction analysis
2. Add institutional ownership tracking
3. Calculate sentiment scores
4. Integrate into QSS
5. Test

**Result**: QSS = 7 components (add 15%)

### Phase 4: Analyst + Social (Medium Priority)
**Time**: 3-4 hours
**Changes**:
1. Add Finnhub analyst trends
2. Add social sentiment
3. Add price targets
4. Integrate into QSS
5. Test

**Result**: QSS = 10 components (add 15%)

### Phase 5: Historical Trend Analysis (Future)
**Time**: 2-3 hours
**Changes**:
1. Add 3-quarter earnings surprise trend
2. Add revenue surprise trend
3. Weight consistency
4. Test

## Database Schema Changes

### Add New Fields to `sentiment_history`:
```sql
-- News sentiment
news_sentiment_score DECIMAL(8,4),
news_buzz DECIMAL(8,4),
news_article_count INTEGER,
news_api_url TEXT,
news_raw_data JSONB,

-- Insider sentiment
insider_mspr DECIMAL(8,4),
insider_net_purchases BIGINT,
insider_transaction_count INTEGER,
insider_api_url TEXT,
insider_raw_data JSONB,

-- Institutional sentiment
institutional_ownership_pct DECIMAL(5,2),
institutional_quarter_change DECIMAL(8,4),
institutional_count INTEGER,
institutional_api_url TEXT,
institutional_raw_data JSONB,

-- Analyst trends
analyst_consensus VARCHAR(20),
analyst_strong_buy INTEGER,
analyst_buy INTEGER,
analyst_hold INTEGER,
analyst_sell INTEGER,
analyst_strong_sell INTEGER,
analyst_month_change INTEGER,
analyst_api_url TEXT,
analyst_raw_data JSONB,

-- Social sentiment
social_reddit_sentiment DECIMAL(8,4),
social_twitter_sentiment DECIMAL(8,4),
social_mention_count INTEGER,
social_api_url TEXT,
social_raw_data JSONB,

-- Price targets
price_target_median DECIMAL(10,2),
price_target_upside_pct DECIMAL(8,4),
price_target_analyst_count INTEGER,
price_target_api_url TEXT,
price_target_raw_data JSONB,
```

## Benefits of Enhancements

### 1. API Tracking (Immediate)
- ✅ Full transparency on data sources
- ✅ Better debugging
- ✅ Historical API tracking
- ✅ Compliance/audit trail

### 2. Additional Sentiment Indicators
- ✅ **More accurate signals**: 10 data sources vs 4
- ✅ **Better diversification**: Professional + retail + news
- ✅ **Shorter-term signals**: News/social catch momentum
- ✅ **Higher conviction**: Multiple signals confirm
- ✅ **Edge over competitors**: Most sentiment tools use 2-3 sources

### 3. Real-World Edge
- News sentiment catches catalysts early
- Insider buying predicts upturns
- Social sentiment catches meme momentum
- Institutional flows show smart money
- Analyst trends show professional consensus

## Risks & Mitigation

### Risk 1: API Rate Limits
**Mitigation**: 
- Finnhub free tier: 60 calls/minute
- We need 6 calls per stock (news, insider, institutional, analyst, social, price target)
- Process 10 stocks/minute = safe rate
- 501 stocks = ~50 minutes

### Risk 2: Data Quality
**Mitigation**:
- Make all new components optional
- QSS still works if some data missing
- Weight available components proportionally
- Track data availability flags

### Risk 3: Overfitting
**Mitigation**:
- Use sensible fixed weights (not optimized)
- Diversify signal sources
- Track out-of-sample performance
- Regular backtesting

### Risk 4: Cost
**Mitigation**:
- All additions use FREE tier APIs
- No premium subscriptions needed
- Only rate limit consideration

## Recommendation: Phased Approach

### Start with Phase 1 (Today)
✅ Add API tracking - No risk, pure benefit

### Then Phase 2 (This Week)
✅ Add news sentiment - Highest value/effort ratio

### Evaluate before Phase 3+
📊 Check if news sentiment improves signal quality
📊 Measure impact on trading decisions
📊 Then decide on insider/institutional/social additions

## Summary

### API Tracking: **DO IT**
- Low effort, high value
- No downside
- Matches timing study quality

### Additional Indicators: **RECOMMEND NEWS FIRST**
- News sentiment = biggest bang for buck
- Then insider/institutional if news works well
- Social sentiment last (most volatile)

### Formula Changes: **TEST INCREMENTALLY**
- Start with 5 components (current 4 + news)
- Validate improvement
- Then add more

Would you like me to:
1. **Just add API tracking** (simple, safe)
2. **Add API tracking + news sentiment** (good value)
3. **Full enhancement with all 10 components** (comprehensive)

