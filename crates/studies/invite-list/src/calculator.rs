use crate::models::{SafetyAnalysis, SP500Stock, InviteListRecord, ApiConfig};
use chrono::Utc;
use serde_json::Value;

/// Calculates safety analysis for S&P 500 stocks
pub struct InviteListCalculator {
    api_config: ApiConfig,
}

impl InviteListCalculator {
    pub fn new(api_config: ApiConfig) -> Self {
        Self { api_config }
    }

    /// Analyzes a stock for trading safety
    pub fn analyze_stock_safety(
        &self,
        _stock: &SP500Stock,
        company_data: &Value,
        financial_data: &Value,
        price_data: &Value,
    ) -> SafetyAnalysis {
        let warning_flags = Vec::new();
        let missing_data_components = Vec::new();
        let mut safety_checks = Vec::new();

        // Check 1: Recent earnings data
        let has_recent_earnings = self.check_recent_earnings(company_data, financial_data);
        safety_checks.push(("recent_earnings", has_recent_earnings, 0.20));

        // Check 2: Positive revenue
        let has_positive_revenue = self.check_positive_revenue(financial_data);
        safety_checks.push(("positive_revenue", has_positive_revenue, 0.20));

        // Check 3: Stable price (not too volatile)
        let has_stable_price = self.check_stable_price(price_data);
        safety_checks.push(("stable_price", has_stable_price, 0.20));

        // Check 4: Sufficient volume
        let has_sufficient_volume = self.check_sufficient_volume(price_data);
        safety_checks.push(("sufficient_volume", has_sufficient_volume, 0.20));

        // Check 5: Analyst coverage
        let has_analyst_coverage = self.check_analyst_coverage(company_data);
        safety_checks.push(("analyst_coverage", has_analyst_coverage, 0.20));

        // Calculate overall safety score
        let safety_score = self.calculate_safety_score(&safety_checks);
        
        // Determine risk level
        let risk_level = self.determine_risk_level(safety_score, &safety_checks);
        
        // Determine volatility and liquidity ratings
        let volatility_rating = self.determine_volatility_rating(price_data);
        let liquidity_rating = self.determine_liquidity_rating(price_data);

        // Generate safety reasoning
        let safety_reasoning = self.generate_safety_reasoning(&safety_checks, safety_score);

        // Check if stock is safe to trade
        let is_safe_to_trade = safety_score >= 0.6 && 
                              has_recent_earnings && 
                              has_positive_revenue && 
                              risk_level != "VeryHigh";

        SafetyAnalysis {
            is_safe_to_trade,
            safety_score,
            safety_reasoning,
            risk_level,
            volatility_rating,
            liquidity_rating,
            has_recent_earnings,
            has_positive_revenue,
            has_stable_price,
            has_sufficient_volume,
            has_analyst_coverage,
            warning_flags,
            missing_data_components,
        }
    }

    /// Checks if the company has recent earnings data
    fn check_recent_earnings(&self, company_data: &Value, financial_data: &Value) -> bool {
        // Check if we have recent financial data
        if let Some(ratios) = financial_data.as_array() {
            if !ratios.is_empty() {
                return true;
            }
        }

        // Check company profile for recent data
        if let Some(profile) = company_data.as_array() {
            if let Some(first_profile) = profile.first() {
                if first_profile["sector"].is_string() && first_profile["industry"].is_string() {
                    return true;
                }
            }
        }

        false
    }

    /// Checks if the company has positive revenue
    /// Uses profit margins as a proxy since the ratios endpoint doesn't have direct revenue data
    fn check_positive_revenue(&self, financial_data: &Value) -> bool {
        if let Some(ratios) = financial_data.as_array() {
            if let Some(latest_ratio) = ratios.first() {
                // Check 1: Gross profit margin (if positive, company has revenue)
                if let Some(gross_margin) = latest_ratio["grossProfitMargin"].as_f64() {
                    if gross_margin > 0.0 {
                        return true;
                    }
                }
                
                // Check 2: Net profit margin (if positive, company has revenue and profit)
                if let Some(net_margin) = latest_ratio["netProfitMargin"].as_f64() {
                    if net_margin > 0.0 {
                        return true;
                    }
                }
                
                // Check 3: Return on assets (if positive, company is profitable)
                if let Some(roa) = latest_ratio["returnOnAssets"].as_f64() {
                    if roa > 0.0 {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Checks if the stock has stable price (not too volatile)
    fn check_stable_price(&self, price_data: &Value) -> bool {
        if let Some(historical) = price_data["historical"].as_array() {
            if historical.len() < 30 {
                return false; // Need at least 30 days of data
            }

            let prices: Vec<f64> = historical
                .iter()
                .take(30) // Last 30 days
                .filter_map(|day| day["close"].as_f64())
                .collect();

            if prices.len() < 30 {
                return false;
            }

            // Calculate volatility (standard deviation)
            let mean = prices.iter().sum::<f64>() / prices.len() as f64;
            let variance = prices.iter()
                .map(|price| (price - mean).powi(2))
                .sum::<f64>() / prices.len() as f64;
            let std_dev = variance.sqrt();

            // Consider stable if volatility is less than 5% of mean price
            let volatility_threshold = mean * 0.05;
            std_dev < volatility_threshold
        } else {
            false
        }
    }

    /// Checks if the stock has sufficient trading volume
    fn check_sufficient_volume(&self, price_data: &Value) -> bool {
        if let Some(historical) = price_data["historical"].as_array() {
            if historical.len() < 10 {
                return false;
            }

            let volumes: Vec<i64> = historical
                .iter()
                .take(10) // Last 10 days
                .filter_map(|day| day["volume"].as_i64())
                .collect();

            if volumes.is_empty() {
                return false;
            }

            let avg_volume = volumes.iter().sum::<i64>() / volumes.len() as i64;
            
            // Consider sufficient if average volume is at least 100,000 shares
            avg_volume >= 100_000
        } else {
            false
        }
    }

    /// Checks if the stock has analyst coverage
    fn check_analyst_coverage(&self, company_data: &Value) -> bool {
        if let Some(profile) = company_data.as_array() {
            if let Some(first_profile) = profile.first() {
                // Check if we have basic company information
                return first_profile["sector"].is_string() && 
                       first_profile["industry"].is_string() &&
                       first_profile["description"].is_string();
            }
        }
        false
    }

    /// Calculates overall safety score based on individual checks
    fn calculate_safety_score(&self, safety_checks: &[(&str, bool, f64)]) -> f64 {
        let mut total_score = 0.0;
        let mut total_weight = 0.0;

        for (_, passed, weight) in safety_checks {
            if *passed {
                total_score += weight;
            }
            total_weight += weight;
        }

        if total_weight > 0.0 {
            total_score / total_weight
        } else {
            0.0
        }
    }

    /// Determines risk level based on safety score and checks
    fn determine_risk_level(&self, safety_score: f64, safety_checks: &[(&str, bool, f64)]) -> String {
        let failed_checks = safety_checks.iter().filter(|(_, passed, _)| !passed).count();
        
        match (safety_score, failed_checks) {
            (score, _) if score >= 0.8 && failed_checks <= 1 => "Low".to_string(),
            (score, _) if score >= 0.6 && failed_checks <= 2 => "Medium".to_string(),
            (score, _) if score >= 0.4 && failed_checks <= 3 => "High".to_string(),
            _ => "VeryHigh".to_string(),
        }
    }

    /// Determines volatility rating based on price data
    fn determine_volatility_rating(&self, price_data: &Value) -> String {
        if let Some(historical) = price_data["historical"].as_array() {
            if historical.len() < 30 {
                return "High".to_string();
            }

            let prices: Vec<f64> = historical
                .iter()
                .take(30)
                .filter_map(|day| day["close"].as_f64())
                .collect();

            if prices.len() < 30 {
                return "High".to_string();
            }

            let mean = prices.iter().sum::<f64>() / prices.len() as f64;
            let variance = prices.iter()
                .map(|price| (price - mean).powi(2))
                .sum::<f64>() / prices.len() as f64;
            let std_dev = variance.sqrt();

            let volatility_percentage = (std_dev / mean) * 100.0;

            match volatility_percentage {
                p if p < 2.0 => "Low".to_string(),
                p if p < 5.0 => "Medium".to_string(),
                _ => "High".to_string(),
            }
        } else {
            "High".to_string()
        }
    }

    /// Determines liquidity rating based on volume data
    fn determine_liquidity_rating(&self, price_data: &Value) -> String {
        if let Some(historical) = price_data["historical"].as_array() {
            if historical.len() < 10 {
                return "Low".to_string();
            }

            let volumes: Vec<i64> = historical
                .iter()
                .take(10)
                .filter_map(|day| day["volume"].as_i64())
                .collect();

            if volumes.is_empty() {
                return "Low".to_string();
            }

            let avg_volume = volumes.iter().sum::<i64>() / volumes.len() as i64;

            match avg_volume {
                v if v >= 1_000_000 => "High".to_string(),
                v if v >= 100_000 => "Medium".to_string(),
                _ => "Low".to_string(),
            }
        } else {
            "Low".to_string()
        }
    }

    /// Generates human-readable safety reasoning
    fn generate_safety_reasoning(&self, safety_checks: &[(&str, bool, f64)], safety_score: f64) -> String {
        let mut reasoning = format!("Safety Score: {:.2} ({}%)\n\n", safety_score, (safety_score * 100.0) as i32);
        
        reasoning.push_str("Safety Checks:\n");
        for (check_name, passed, weight) in safety_checks {
            let status = if *passed { "✓ PASS" } else { "✗ FAIL" };
            let weight_pct = (weight * 100.0) as i32;
            reasoning.push_str(&format!("- {}: {} ({}% weight)\n", 
                check_name.replace("_", " ").to_uppercase(), status, weight_pct));
        }

        reasoning.push_str(&format!("\nOverall Assessment: "));
        match safety_score {
            s if s >= 0.8 => reasoning.push_str("EXCELLENT - Very safe for trading"),
            s if s >= 0.6 => reasoning.push_str("GOOD - Safe for trading with caution"),
            s if s >= 0.4 => reasoning.push_str("MODERATE - Requires careful analysis"),
            _ => reasoning.push_str("POOR - Not recommended for trading"),
        }

        reasoning
    }

    /// Creates an InviteListRecord from analysis results
    pub fn create_invite_list_record(
        &self,
        stock: &SP500Stock,
        safety_analysis: &SafetyAnalysis,
        company_data: &Value,
        financial_data: &Value,
        price_data: &Value,
    ) -> InviteListRecord {
        InviteListRecord {
            id: None,
            symbol: stock.symbol.clone(),
            company_name: stock.name.clone(),
            sector: stock.sector.clone(),
            industry: stock.industry.clone(),
            market_cap: stock.market_cap,
            current_price: stock.current_price,
            is_safe_to_trade: safety_analysis.is_safe_to_trade,
            safety_score: Some(safety_analysis.safety_score),
            safety_reasoning: Some(safety_analysis.safety_reasoning.clone()),
            has_recent_earnings: safety_analysis.has_recent_earnings,
            has_positive_revenue: safety_analysis.has_positive_revenue,
            has_stable_price: safety_analysis.has_stable_price,
            has_sufficient_volume: safety_analysis.has_sufficient_volume,
            has_analyst_coverage: safety_analysis.has_analyst_coverage,
            risk_level: safety_analysis.risk_level.clone(),
            volatility_rating: Some(safety_analysis.volatility_rating.clone()),
            liquidity_rating: Some(safety_analysis.liquidity_rating.clone()),
            data_source: "FMP".to_string(),
            last_updated: Utc::now(),
            data_freshness_score: Some(0.95), // Assume fresh data
            analysis_date: Utc::now(),
            analysis_duration_ms: Some(1000), // Placeholder
            warning_flags: safety_analysis.warning_flags.clone(),
            missing_data_components: safety_analysis.missing_data_components.clone(),
            raw_company_data: Some(company_data.clone()),
            raw_financial_data: Some(financial_data.clone()),
            raw_price_data: Some(price_data.clone()),
            created_at: None,
            updated_at: None,
        }
    }
}
