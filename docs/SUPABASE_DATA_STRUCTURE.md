# Supabase Data Structure for Stock Analysis Website

This document describes the data structures stored in Supabase that will be used to create a comprehensive stock analysis website. The data is organized into several main tables, each serving a specific purpose in the stock analysis ecosystem.

## Overview

The OKTStocks2 system analyzes stocks using multiple methodologies and stores the results in Supabase for web consumption. The data is refreshed every hour via cron jobs and provides real-time insights for investment decisions.

## Database Tables

### 1. Fundamentals Table (`fundamentals`)

**Purpose**: Stores comprehensive fundamental analysis of stocks based on financial metrics and ratios.

**Key Data Points**:
- **Core Scoring**: Overall fundamentals score (0-100), letter grade rating (A+ to F), investment recommendation (StrongBuy, Buy, Hold, Sell, StrongSell), confidence score
- **Component Scores**: Profitability, growth, valuation, financial strength, and efficiency scores
- **Financial Metrics**: ROE, ROA, ROIC, profit margins, growth rates, valuation ratios (P/E, P/B, P/S, etc.)
- **Company Metadata**: Sector, industry, market cap, beta, dividend yield, shares outstanding
- **AI Analysis**: ChatGPT explanations and trading suggestions
- **Data Quality**: Computation time, data freshness, warning flags, API source tracking

**Website Use Cases**:
- Stock screening and filtering by fundamentals
- Investment recommendations dashboard
- Sector and industry analysis
- Financial health scoring
- AI-powered investment insights

### 2. Timing Table (`timing`)

**Purpose**: Stores Technical Trading Score (TTS) analysis results for optimal entry/exit timing.

**Key Data Points**:
- **TTS Analysis**: Overall TTS score (0-100), trading signal, confidence score
- **Technical Indicators**: RSI, MACD, Bollinger Bands, Moving Averages, Stochastic, Williams %R, ATR, Volume scores
- **Trend Analysis**: Short, medium, and long-term trends with strength and consistency metrics
- **Support & Resistance**: Key price levels, distances, and strength ratings
- **Volume Analysis**: Current vs average volume, volume trends, volume-price relationships
- **Risk Assessment**: Volatility scores, risk levels, stop-loss recommendations, risk-reward ratios
- **AI Analysis**: ChatGPT explanations and trading suggestions

**Website Use Cases**:
- Technical analysis dashboard
- Entry/exit timing recommendations
- Risk management tools
- Chart overlays with technical indicators
- Trading signal alerts

### 3. Market Regime Table (`market_regime`)

**Purpose**: Stores overall market regime analysis - the "vibe of the whole club" affecting all stocks.

**Key Data Points**:
- **Market Classification**: Market regime (Bull, Bear, Sideways, Volatile, Stable, Transition), confidence level
- **Market Context**: SPY price and changes, VIX, market breadth, sector performance
- **Volatility Analysis**: Market volatility levels and percentiles
- **Trend Analysis**: Market-wide trend directions and strength
- **Breadth Analysis**: Advancing vs declining stocks, new highs/lows
- **Sector Performance**: Technology, healthcare, financial, energy, consumer sector returns
- **Sentiment Indicators**: Fear & Greed Index, put/call ratios, margin debt trends
- **Risk Assessment**: Market risk levels and maximum drawdown risks
- **AI Analysis**: ChatGPT market outlook and regime analysis

**Website Use Cases**:
- Market overview dashboard
- Market regime detection and alerts
- Sector rotation analysis
- Market sentiment indicators
- Risk assessment for portfolio positioning

### 4. Regime Table (`regime`)

**Purpose**: Stores individual stock regime analysis combining TTS scores with market regime context.

**Key Data Points**:
- **TTS Scoring**: Time To Sell scores (-1.0 to +1.0), trading signals, market regime context
- **Component Scores**: Momentum, volatility, volume, support/resistance, market correlation scores
- **Technical Indicators**: RSI, MACD, Bollinger Bands, SMAs, ATR, Stochastic, Williams %R
- **Market Context**: SPY data, VIX, sector performance, market breadth
- **Risk Assessment**: Risk levels, volatility scores, stop-loss levels, position sizing
- **API Tracking**: Data sources, endpoints used, raw response data
- **AI Analysis**: ChatGPT regime analysis, TTS interpretation, trading recommendations

**Website Use Cases**:
- Individual stock regime analysis
- Position sizing recommendations
- Market correlation analysis
- Risk-adjusted scoring
- AI-powered trading insights

### 5. Invite List Table (`invite_list`)

**Purpose**: Stores S&P 500 stocks with safety analysis for trading eligibility.

**Key Data Points**:
- **Stock Information**: Symbol, company name, sector, industry, market cap, current price
- **Safety Analysis**: Safety score (0.00-1.00), safety reasoning, trading eligibility
- **Financial Health**: Recent earnings, positive revenue, stable price, sufficient volume, analyst coverage
- **Risk Assessment**: Risk levels (Low, Medium, High, VeryHigh), volatility and liquidity ratings
- **Data Quality**: Data source tracking, freshness scores, warning flags
- **Raw Data**: Company data, financial data, price data in JSON format

**Website Use Cases**:
- Stock universe filtering
- Safety screening tools
- Sector and risk analysis
- Data quality indicators
- Watchlist management

## Data Relationships

The tables work together to provide a comprehensive stock analysis system:

1. **Invite List** → Defines the universe of tradeable stocks
2. **Fundamentals** → Provides fundamental analysis for each stock
3. **Timing** → Offers technical analysis and entry/exit timing
4. **Regime** → Combines individual stock analysis with market context
5. **Market Regime** → Provides overall market context affecting all stocks

## Website Data Consumption Patterns

### Dashboard Views
- **Market Overview**: Latest market regime, key indicators, sector performance
- **Stock Universe**: Safe stocks list with basic information and safety scores
- **Individual Stock**: Complete analysis combining fundamentals, timing, and regime data
- **Sector Analysis**: Grouped analysis by sector with performance metrics
- **Risk Management**: Risk levels, stop-loss recommendations, position sizing

### Filtering and Search
- Filter by fundamentals score ranges
- Filter by trading signals (Buy, Sell, Hold)
- Filter by market regime
- Filter by safety status
- Filter by sector and industry
- Filter by risk levels

### Real-time Updates
- Data refreshed every hour via cron jobs
- Real-time sentiment data from database [[memory:8152540]]
- Dashboard displays real information (no mockups) [[memory:5997050]]

## API Endpoints for Website

The website will consume data through Supabase's REST API with endpoints like:

- `GET /fundamentals?symbol=AAPL&limit=1` - Latest fundamentals for a stock
- `GET /timing?symbol=AAPL&limit=1` - Latest timing analysis
- `GET /market_regime?limit=1` - Current market regime
- `GET /invite_list?is_safe_to_trade=true` - Safe stocks list
- `GET /regime?symbol=AAPL&limit=1` - Latest regime analysis

## Data Quality and Reliability

- All tables include data quality metrics and warning flags
- API source tracking for data provenance
- Computation time and data freshness indicators
- AI-generated explanations for human-readable insights
- Comprehensive error handling and fallback data sources

## Security and Access

- Supabase handles authentication and authorization
- Row-level security policies can be implemented
- API keys for secure data access
- Rate limiting and usage monitoring

This data structure provides a solid foundation for building a comprehensive stock analysis website with real-time data, AI-powered insights, and multiple analysis methodologies.

