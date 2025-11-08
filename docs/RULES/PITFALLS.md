# Known Pitfalls

Record **mistakes, regressions, or traps** so they never repeat.  
Keep entries short, one-line lessons from real issues.

---

## [2025-10-04] Unwraps in Production
- Problem: Using `unwrap()` on Option caused Lambda panic in production.
- Fix: Replaced with `?` and proper error propagation via anyhow.
- Affected: core/handlers/

## [2025-10-04] Missing AWS_REGION
- Problem: Lambda failed startup when AWS_REGION env var absent.
- Fix: Added default load from environment with fallback check.
- Affected: bootstrap.rs

## [2025-10-05] Null Database Fields from Mock Data
- Problem: Database fields showing null values because calculator was using hardcoded mock data instead of real API responses
- Fix: Implemented comprehensive API data collection and populated all database fields with real data
- Affected: src/sentiment/calculator.rs, src/database/models.rs
- Lesson: Always verify data sources are actually fetching real data, not just mock values

## [2025-10-05] Single API Source Failures
- Problem: Relying on single API source (Alpha Vantage) caused frequent data gaps and null values
- Fix: Implemented multi-source fallback system: Alpha Vantage → FMP → Finnhub for each data type
- Affected: src/sentiment/calculator.rs (all data collection methods)
- Lesson: Financial data APIs are unreliable - always implement fallback sources

## [2025-10-05] Missing Market Context
- Problem: Stock sentiment analysis without market context is meaningless (stock could be up 10% but market up 15%)
- Fix: Added S&P 500 and sector ETF benchmark data for relative performance analysis
- Affected: src/sentiment/calculator.rs (collect_market_benchmark_data method)
- Lesson: Always include market/sector context for stock analysis

## [2025-10-05] Inconsistent Data Types
- Problem: Mixing mock data from test examples with real data caused inconsistent results in database
- Fix: Cleaned up test data and ensured all examples use real API data collection
- Affected: examples/ directory, database records
- Lesson: Separate test data from production data collection - never mix them in database

## [2025-10-05] API Rate Limiting Without Fallbacks
- Problem: Alpha Vantage rate limits (5 calls/minute) caused frequent API failures
- Fix: Implemented multiple API sources and better error handling for rate limits
- Affected: src/sentiment/calculator.rs (API call methods)
- Lesson: Always implement rate limit handling and multiple data sources for production systems

## [2025-10-05] Missing Error Context in Logs
- Problem: Generic error messages made debugging API failures difficult
- Fix: Added detailed logging with API source, endpoint, and specific error context
- Affected: src/sentiment/calculator.rs (all API methods)
- Lesson: Financial data debugging requires detailed error context - log everything

## [2025-10-05] TTS Implementation Falling Back to Mock Data
- Problem: Created mock data example instead of implementing real API integration for TTS calculations
- Fix: Implemented real data collection from FMP/Alpha Vantage APIs and removed mock example
- Affected: examples/tts_aapl_mock_example.rs (deleted), src/timing/calculator.rs (fixed)
- Lesson: NEVER create mock data examples for production features - always implement real API integration first, then create examples that work with real data

## [2025-10-05] Database Storage Implementation Compilation Errors
- Problem: Multiple compilation errors when implementing timing database storage due to mismatched database patterns
- Fix: Added timing methods to DatabaseClient and used existing REST API pattern from sentiment storage
- Affected: src/database/timing_storage.rs, src/database/client.rs, src/ai/chatgpt_service.rs
- Errors Fixed:
  1. `tokio_postgres::types::ToSql` not found - removed direct PostgreSQL dependencies
  2. Private field `client` access - added timing methods to DatabaseClient instead of direct access
  3. Type mismatches in query parameters - used proper method signatures in DatabaseClient
  4. Response borrowing issues in ChatGPT service - extracted status before consuming response
  5. Unused imports - cleaned up all unused imports after fixing compilation
- Lesson: Always follow existing patterns in codebase - extend existing classes rather than duplicating patterns

## [2025-01-05] Regime Module Compilation Errors
- Problem: Multiple compilation errors when implementing regime analysis module due to type mismatches and error handling issues
- Fix: Fixed error handling, type conversions, and variable naming issues
- Affected: src/regime/calculator.rs
- Errors Fixed:
  1. `error::Error::ApiError` struct syntax - changed to tuple variant syntax
  2. Volume type mismatch (f64 vs u64) - added explicit cast to u64
  3. Ambiguous numeric types in clamp() calls - added explicit f64 type annotations
  4. Unused imports (warn) - removed unused tracing import
  5. Unused variable warnings - prefixed with underscore or used in calculations
- Lesson: Always check error types in existing codebase before creating new error instances, and be explicit with numeric types in mathematical operations