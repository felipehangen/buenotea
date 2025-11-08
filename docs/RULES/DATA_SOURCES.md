# 📘 DATA SOURCE RULES – OKTStocks2

## 🚀 CURRENT IMPLEMENTATION STATUS

### ✅ **FULLY IMPLEMENTED & WORKING**
- **Earnings Data**: Alpha Vantage + FMP + Finnhub integration
- **Revenue Data**: Alpha Vantage Income Statements + FMP Income Statements  
- **Price Data**: FMP historical prices with RSI calculation
- **Market Benchmarks**: S&P 500 (SPY) and sector ETF data
- **Relative Performance**: Stock vs market vs sector calculations
- **Volume Analysis**: Current volume vs 30-day average ratios
- **Analyst Data**: FMP analyst recommendations count
- **Technical Analysis**: 14-period RSI with momentum analysis

### ⚠️ **PARTIALLY IMPLEMENTED**
- **Short Interest**: Using Finnhub news sentiment as proxy
- **Options Flow**: Using FMP analyst recommendations as proxy

### ❌ **NOT IMPLEMENTED (Premium APIs Required)**
- **Real Short Interest Data**: EODHD ($9.99/month) or similar
- **Real Options Flow Data**: Unusual Whales ($29.99/month) or Polygon premium

### 📊 **DATA COVERAGE ACHIEVED**
- **EPS Estimates**: ✅ Real data from multiple sources
- **Revenue Data**: ✅ Real data from financial statements  
- **Price & RSI**: ✅ Real technical analysis
- **Market Benchmarks**: ✅ Real S&P 500 and sector comparisons
- **Volume Analysis**: ✅ Real volume ratio calculations
- **Analyst Coverage**: ✅ Real analyst recommendation counts

## CURRENT SUBSCRIPTIONS & ACCESS

### **FMP (Financial Modeling Prep) - FREE TIER**
- ✅ **Available Endpoints**:
  - Historical Prices: `/api/v3/historical-price-full/{symbol}`
  - Historical Price Lists: `/api/v3/historical-price-full`
  - Market Benchmarks: SPY, XLK, QQQ, etc.
  - Company Profiles: `/api/v3/profile/{symbol}`
  - Financial Ratios: `/api/v3/ratios/{symbol}`
  - Enterprise Values: `/api/v3/enterprise-values/{symbol}`
  - Market Capitalization: `/api/v3/market-capitalization/{symbol}`
  - Key Metrics: `/api/v3/key-metrics/{symbol}`
  - Income Statements: `/api/v3/income-statement/{symbol}`
  - Balance Sheets: `/api/v3/balance-sheet-statement/{symbol}`
  - Cash Flow Statements: `/api/v3/cash-flow-statement/{symbol}`
  - Financial Growth: `/api/v3/financial-growth/{symbol}`
  - Stock Screener: `/api/v3/stock-screener`
  - Stock List: `/api/v3/stock/list`
  - S&P 500 Companies: `/api/v3/sp500_constituent`
  - NASDAQ Companies: `/api/v3/nasdaq_constituent`
  - Dow Jones Companies: `/api/v3/dowjones_constituent`
  - Sector Performance: `/api/v3/sector-performance`
  - Most Active Stocks: `/api/v3/stock/most-active`
  - Most Gainer Stocks: `/api/v3/stock/gainers`
  - Most Loser Stocks: `/api/v3/stock/losers`
  - Rate Limit: 250 calls/day
- ❌ **Premium Required** (not subscribed):
  - Earnings Estimates: `/api/v3/analyst-estimates/{symbol}`
  - Short Interest: `/api/v3/short-interest/{symbol}`
  - Insider Trading: `/api/v3/insider-trading`
  - Institutional Holders: `/api/v3/institutional-holder/{symbol}`

### **Alpha Vantage - FREE TIER**
- ✅ **Core Stock APIs**:
  - Daily Time Series: `TIME_SERIES_DAILY`
  - Weekly Time Series: `TIME_SERIES_WEEKLY`
  - Monthly Time Series: `TIME_SERIES_MONTHLY`
  - Intraday Time Series: `TIME_SERIES_INTRADAY`
  - Quote Endpoint: `GLOBAL_QUOTE`
  - Ticker Search: `SYMBOL_SEARCH`
  - Market Status: `MARKET_STATUS`
- ✅ **Fundamental Data**:
  - Company Overview: `OVERVIEW`
  - Income Statement: `INCOME_STATEMENT`
  - Balance Sheet: `BALANCE_SHEET`
  - Cash Flow: `CASH_FLOW`
  - Earnings History: `EARNINGS`
  - Earnings Estimates: `EARNINGS_ESTIMATES`
  - Earnings Calendar: `EARNINGS_CALENDAR`
  - IPO Calendar: `IPO_CALENDAR`
- ✅ **Technical Indicators** (50+ indicators):
  - Simple Moving Average: `SMA`
  - Exponential Moving Average: `EMA`
  - MACD: `MACD`
  - RSI: `RSI`
  - Bollinger Bands: `BBANDS`
  - Stochastic: `STOCH`
  - Williams %R: `WILLR`
  - Average Directional Index: `ADX`
  - Commodity Channel Index: `CCI`
  - Money Flow Index: `MFI`
  - On Balance Volume: `OBV`
  - Accumulation/Distribution: `AD`
  - Chaikin A/D Oscillator: `ADOSC`
  - Aroon: `AROON`
  - Aroon Oscillator: `AROONOSC`
  - Price Rate of Change: `ROC`
  - Rate of Change Ratio: `ROCR`
  - Momentum: `MOM`
  - Balance of Power: `BOP`
  - Ultimate Oscillator: `ULTOSC`
  - Plus Directional Indicator: `PLUS_DI`
  - Minus Directional Indicator: `MINUS_DI`
  - Plus Directional Movement: `PLUS_DM`
  - Minus Directional Movement: `MINUS_DM`
  - Average True Range: `ATR`
  - True Range: `TRANGE`
  - Directional Movement Index: `DX`
  - Parabolic SAR: `SAR`
  - Triple Exponential Moving Average: `TEMA`
  - Double Exponential Moving Average: `DEMA`
  - Triple Exponential Average: `TRIX`
  - Commodity Channel Index: `CMO`
  - Relative Strength Index: `STOCHRSI`
  - Stochastic Fast: `STOCHF`
  - Hilbert Transform Trendline: `HT_TRENDLINE`
  - Hilbert Transform Sine Wave: `HT_SINE`
  - Hilbert Transform Trend vs Cycle Mode: `HT_TRENDMODE`
  - Hilbert Transform Dominant Cycle Period: `HT_DCPERIOD`
  - Hilbert Transform Dominant Cycle Phase: `HT_DCPHASE`
  - Hilbert Transform Phasor Components: `HT_PHASOR`
  - Midpoint: `MIDPOINT`
  - Midprice: `MIDPRICE`
  - Weighted Moving Average: `WMA`
  - Double Exponential Moving Average: `DEMA`
  - Triple Exponential Moving Average: `TEMA`
  - Triangular Moving Average: `TRIMA`
  - Kaufman Adaptive Moving Average: `KAMA`
  - MESA Adaptive Moving Average: `MAMA`
  - Volume Weighted Average Price: `VWAP`
  - Triple Smooth Exponential Moving Average: `T3`
  - MACD with controllable MA type: `MACDEXT`
  - Normalized Average True Range: `NATR`
  - Hilbert Transform Instantaneous Trendline: `HT_TRENDLINE`
- ✅ **Economic Indicators**:
  - Real GDP: `REAL_GDP`
  - Real GDP per Capita: `REAL_GDP_PER_CAPITA`
  - Treasury Yield: `TREASURY_YIELD`
  - Federal Funds Rate: `FEDERAL_FUNDS_RATE`
  - CPI: `CPI`
  - Inflation: `INFLATION`
  - Retail Sales: `RETAIL_SALES`
  - Durable Goods Orders: `DURABLES`
  - Unemployment Rate: `UNEMPLOYMENT`
  - Nonfarm Payroll: `NONFARM_PAYROLL`
- ✅ **Forex & Crypto**:
  - Forex Exchange Rates: `FX_DAILY`
  - Crypto Exchange Rates: `DIGITAL_CURRENCY_DAILY`
  - Rate Limit: 5 calls/minute

### **Finnhub - FREE TIER**
- ✅ **Stock Market Data**:
  - Company Profile: `/stock/profile2`
  - Company Peers: `/stock/peers`
  - Company Executives: `/stock/executive`
  - Financial Statements: `/stock/financials`
  - Financial Metrics: `/stock/metric`
  - Real-time Quote: `/quote`
  - Historical Candles: `/stock/candle`
  - Earnings Data: `/stock/earnings`
  - Earnings Estimates: `/company-earnings-estimates`
  - Revenue Estimates: `/company-revenue-estimates`
  - Recommendation Trends: `/recommendation-trends`
  - Price Target: `/price-target`
  - Dividends: `/stock/dividend`
  - Stock Splits: `/stock/split`
  - Insider Transactions: `/stock/insider-transactions`
  - Insider Sentiment: `/stock/insider-sentiment`
  - Institutional Ownership: `/stock/institutional-ownership`
  - Ownership Change: `/stock/ownership-change`
  - Market Status: `/stock/market-status`
  - Exchange: `/stock/exchange`
  - Symbol Lookup: `/search`
  - IPO Calendar: `/calendar/ipo`
  - Earnings Calendar: `/calendar/earnings`
  - Economic Calendar: `/calendar/economic`
  - Economic Data: `/economic`
  - Economic Code: `/economic/code`
- ✅ **Alternative Data**:
  - Earnings Call Transcripts: `/stock/transcripts`
  - COVID-19 Data: `/covid19/us`
  - Real Estate Data: `/real-estate`
  - Merger & Acquisitions: `/merger`
  - Supply Chain: `/supply-chain`
- ✅ **News & Sentiment**:
  - Company News: `/company-news`
  - Market News: `/news`
  - News Sentiment: `/news-sentiment`
  - Social Sentiment: `/social-sentiment`
- ✅ **Technical Analysis**:
  - Pattern Recognition: `/scan/pattern`
  - Support/Resistance: `/scan/support-resistance`
  - Aggregate Indicators: `/scan/technical-indicator`
- ✅ **Forex & Crypto**:
  - Forex Exchange Rates: `/forex/rates`
  - Forex Candles: `/forex/candle`
  - Crypto Exchange Rates: `/crypto/exchange`
  - Crypto Candles: `/crypto/candle`
  - Crypto Symbols: `/crypto/symbol`
- ✅ **Utilities**:
  - Country Metadata: `/country`
  - FDA Calendar: `/fda-calendar`
  - Airline Price Index: `/airline-index`
  - Sector Metrics: `/stock/sector-performance`
  - Press Releases: `/press-releases`
  - Rate Limit: 60 calls/minute

### **Polygon.io - FREE TIER**
- ✅ **Available Endpoints**:
  - Historical Aggregates: `/v2/aggs/ticker/{symbol}/range`
  - Previous Close: `/v2/aggs/ticker/{symbol}/prev`
  - Daily Open/Close: `/v1/open-close/{symbol}/{date}`
  - Grouped Daily: `/v2/aggs/grouped/locale/us/market/stocks/{date}`
  - Ticker Details: `/v3/reference/tickers/{symbol}`
  - Ticker News: `/v2/reference/news`
  - Ticker Types: `/v3/reference/tickers/types`
  - Markets: `/v3/reference/markets`
  - Locales: `/v3/reference/locales`
  - Market Status: `/v1/marketstatus/now`
  - Market Holidays: `/v1/marketstatus/upcoming`
  - Rate Limit: 5 calls/minute
- ❌ **Premium Required** (not subscribed):
  - Real-time Options Flow
  - Dark Pool Data
  - Level 2 Market Data
  - Real-time Quotes

---

## CURRENT DATA COVERAGE

### **Working Components**
- ✅ **Earnings Revisions** (40% weight): Alpha Vantage
- ✅ **Relative Strength** (30% weight): FMP + Alpha Vantage

### **Missing Components**
- ❌ **Short Interest** (20% weight): Requires FMP Premium
- ❌ **Options Flow** (10% weight): Requires Polygon Premium

---

## IMPLEMENTATION STATUS

### **Integrated & Working**
- **FMP**: Historical prices, company profiles, financial statements
- **Alpha Vantage**: Earnings estimates, technical indicators, economic data

### **Available but Not Integrated**
- **Finnhub**: Comprehensive stock data, news sentiment, insider transactions
- **Polygon**: Basic aggregates, news, market status

---

*Last Updated: January 4, 2025*
*Focus: Current subscription access only*