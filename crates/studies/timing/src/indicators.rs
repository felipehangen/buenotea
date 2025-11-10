// Technical indicators calculations for TTS
// All indicators now return scores between -1.0 (strong sell) and +1.0 (strong buy)

use super::models::{PricePoint, IndicatorValues};
use buenotea_core::Result;

/// Calculate RSI (Relative Strength Index)
pub fn calculate_rsi(prices: &[f64], period: usize) -> Result<f64> {
    if prices.len() < period + 1 {
        return Ok(50.0); // Neutral RSI if insufficient data
    }

    let mut gains = Vec::new();
    let mut losses = Vec::new();

    for i in 1..prices.len() {
        let change = prices[i] - prices[i - 1];
        if change > 0.0 {
            gains.push(change);
            losses.push(0.0);
        } else {
            gains.push(0.0);
            losses.push(-change);
        }
    }

    if gains.len() < period {
        return Ok(50.0);
    }

    // Calculate initial averages
    let avg_gain: f64 = gains[..period].iter().sum::<f64>() / period as f64;
    let avg_loss: f64 = losses[..period].iter().sum::<f64>() / period as f64;

    // Calculate smoothed averages
    let mut smoothed_gain = avg_gain;
    let mut smoothed_loss = avg_loss;

    for i in period..gains.len() {
        smoothed_gain = (smoothed_gain * (period - 1) as f64 + gains[i]) / period as f64;
        smoothed_loss = (smoothed_loss * (period - 1) as f64 + losses[i]) / period as f64;
    }

    if smoothed_loss == 0.0 {
        return Ok(100.0);
    }

    let rs = smoothed_gain / smoothed_loss;
    let rsi = 100.0 - (100.0 / (1.0 + rs));

    Ok(rsi)
}

/// Calculate MACD (Moving Average Convergence Divergence)
pub fn calculate_macd(prices: &[f64], fast_period: usize, slow_period: usize, _signal_period: usize) -> Result<(f64, f64, f64)> {
    if prices.len() < slow_period {
        return Ok((0.0, 0.0, 0.0));
    }

    let ema_fast = calculate_ema(prices, fast_period)?;
    let ema_slow = calculate_ema(prices, slow_period)?;
    
    let macd_line = ema_fast - ema_slow;
    
    // For signal line, we need MACD values over time, but we'll approximate
    let signal_line = macd_line * 0.9; // Simplified signal line
    let histogram = macd_line - signal_line;

    Ok((macd_line, signal_line, histogram))
}

/// Calculate EMA (Exponential Moving Average)
pub fn calculate_ema(prices: &[f64], period: usize) -> Result<f64> {
    if prices.is_empty() {
        return Ok(0.0);
    }

    if prices.len() < period {
        // Use SMA if not enough data for EMA
        return Ok(prices.iter().sum::<f64>() / prices.len() as f64);
    }

    let multiplier = 2.0 / (period + 1) as f64;
    let mut ema = prices[..period].iter().sum::<f64>() / period as f64;

    for &price in &prices[period..] {
        ema = (price * multiplier) + (ema * (1.0 - multiplier));
    }

    Ok(ema)
}

/// Calculate SMA (Simple Moving Average)
pub fn calculate_sma(prices: &[f64], period: usize) -> Result<f64> {
    if prices.len() < period {
        return Ok(prices.iter().sum::<f64>() / prices.len() as f64);
    }

    let sum: f64 = prices[prices.len() - period..].iter().sum();
    Ok(sum / period as f64)
}

/// Calculate Bollinger Bands
pub fn calculate_bollinger_bands(prices: &[f64], period: usize, std_dev: f64) -> Result<(f64, f64, f64)> {
    if prices.len() < period {
        let current_price = prices.last().unwrap_or(&0.0);
        return Ok((*current_price, *current_price, *current_price));
    }

    let sma = calculate_sma(prices, period)?;
    let recent_prices = &prices[prices.len() - period..];
    
    let variance: f64 = recent_prices.iter()
        .map(|&price| (price - sma).powi(2))
        .sum::<f64>() / period as f64;
    
    let standard_deviation = variance.sqrt();
    
    let upper_band = sma + (std_dev * standard_deviation);
    let lower_band = sma - (std_dev * standard_deviation);

    Ok((upper_band, sma, lower_band))
}

/// Calculate Stochastic Oscillator
pub fn calculate_stochastic(price_points: &[PricePoint], k_period: usize, _d_period: usize) -> Result<(f64, f64)> {
    if price_points.len() < k_period {
        return Ok((50.0, 50.0));
    }

    let recent_prices = &price_points[price_points.len() - k_period..];
    let current_close = recent_prices.last().unwrap().close;
    
    let highest_high = recent_prices.iter().map(|p| p.high).fold(0.0, f64::max);
    let lowest_low = recent_prices.iter().map(|p| p.low).fold(f64::INFINITY, f64::min);
    
    let k_percent = if highest_high != lowest_low {
        100.0 * (current_close - lowest_low) / (highest_high - lowest_low)
    } else {
        50.0
    };

    // Simplified D calculation (normally would be SMA of K values)
    let d_percent = k_percent * 0.8; // Simplified

    Ok((k_percent, d_percent))
}

/// Calculate Williams %R
pub fn calculate_williams_r(price_points: &[PricePoint], period: usize) -> Result<f64> {
    if price_points.len() < period {
        return Ok(-50.0);
    }

    let recent_prices = &price_points[price_points.len() - period..];
    let current_close = recent_prices.last().unwrap().close;
    
    let highest_high = recent_prices.iter().map(|p| p.high).fold(0.0, f64::max);
    let lowest_low = recent_prices.iter().map(|p| p.low).fold(f64::INFINITY, f64::min);
    
    if highest_high != lowest_low {
        let williams_r = -100.0 * (highest_high - current_close) / (highest_high - lowest_low);
        Ok(williams_r)
    } else {
        Ok(-50.0)
    }
}

/// Calculate ATR (Average True Range)
pub fn calculate_atr(price_points: &[PricePoint], period: usize) -> Result<f64> {
    if price_points.len() < 2 {
        return Ok(0.0);
    }

    let mut true_ranges = Vec::new();
    
    for i in 1..price_points.len() {
        let tr = price_points[i].true_range(price_points[i - 1].close);
        true_ranges.push(tr);
    }

    if true_ranges.len() < period {
        return Ok(true_ranges.iter().sum::<f64>() / true_ranges.len() as f64);
    }

    let atr = true_ranges[true_ranges.len() - period..].iter().sum::<f64>() / period as f64;
    Ok(atr)
}

/// Calculate all technical indicators for a series of price points
pub fn calculate_all_indicators(price_points: &[PricePoint]) -> Result<IndicatorValues> {
    if price_points.is_empty() {
        return Ok(IndicatorValues {
            rsi_14: None,
            macd: None,
            macd_signal: None,
            macd_histogram: None,
            bollinger_upper: None,
            bollinger_middle: None,
            bollinger_lower: None,
            sma_20: None,
            sma_50: None,
            sma_200: None,
            ema_12: None,
            ema_26: None,
            stochastic_k: None,
            stochastic_d: None,
            williams_r: None,
            atr_14: None,
        });
    }

    let closes: Vec<f64> = price_points.iter().map(|p| p.close).collect();

    let rsi_14 = calculate_rsi(&closes, 14).ok();
    let (macd, macd_signal, macd_histogram) = calculate_macd(&closes, 12, 26, 9).unwrap_or((0.0, 0.0, 0.0));
    let (bb_upper, bb_middle, bb_lower) = calculate_bollinger_bands(&closes, 20, 2.0).unwrap_or((0.0, 0.0, 0.0));
    let sma_20 = calculate_sma(&closes, 20).ok();
    let sma_50 = calculate_sma(&closes, 50).ok();
    let sma_200 = calculate_sma(&closes, 200).ok();
    let ema_12 = calculate_ema(&closes, 12).ok();
    let ema_26 = calculate_ema(&closes, 26).ok();
    let (stoch_k, stoch_d) = calculate_stochastic(price_points, 14, 3).unwrap_or((50.0, 50.0));
    let williams_r = calculate_williams_r(price_points, 14).ok();
    let atr_14 = calculate_atr(price_points, 14).ok();

    Ok(IndicatorValues {
        rsi_14,
        macd: Some(macd),
        macd_signal: Some(macd_signal),
        macd_histogram: Some(macd_histogram),
        bollinger_upper: Some(bb_upper),
        bollinger_middle: Some(bb_middle),
        bollinger_lower: Some(bb_lower),
        sma_20,
        sma_50,
        sma_200,
        ema_12,
        ema_26,
        stochastic_k: Some(stoch_k),
        stochastic_d: Some(stoch_d),
        williams_r,
        atr_14,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_prices() -> Vec<PricePoint> {
        vec![
            PricePoint { date: Utc::now(), open: 100.0, high: 105.0, low: 98.0, close: 103.0, volume: 1000000 },
            PricePoint { date: Utc::now(), open: 103.0, high: 107.0, low: 101.0, close: 105.0, volume: 1200000 },
            PricePoint { date: Utc::now(), open: 105.0, high: 108.0, low: 103.0, close: 106.0, volume: 1100000 },
            PricePoint { date: Utc::now(), open: 106.0, high: 109.0, low: 104.0, close: 108.0, volume: 1300000 },
            PricePoint { date: Utc::now(), open: 108.0, high: 110.0, low: 106.0, close: 109.0, volume: 1150000 },
        ]
    }

    #[test]
    fn test_rsi_calculation() {
        let prices = vec![100.0, 102.0, 101.0, 103.0, 105.0, 104.0, 106.0, 108.0];
        let rsi = calculate_rsi(&prices, 4).unwrap();
        assert!(rsi >= 0.0 && rsi <= 100.0);
    }

    #[test]
    fn test_sma_calculation() {
        let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let sma = calculate_sma(&prices, 3).unwrap();
        assert_eq!(sma, 4.0); // (3+4+5)/3 = 4
    }

    #[test]
    fn test_ema_calculation() {
        let prices = vec![100.0, 102.0, 101.0, 103.0, 105.0];
        let ema = calculate_ema(&prices, 3).unwrap();
        assert!(ema > 0.0);
    }

    #[test]
    fn test_bollinger_bands() {
        let prices = vec![100.0, 102.0, 101.0, 103.0, 105.0, 104.0, 106.0, 108.0];
        let (upper, middle, lower) = calculate_bollinger_bands(&prices, 5, 2.0).unwrap();
        assert!(upper > middle);
        assert!(middle > lower);
    }

    #[test]
    fn test_stochastic() {
        let price_points = create_test_prices();
        let (k, d) = calculate_stochastic(&price_points, 3, 2).unwrap();
        assert!(k >= 0.0 && k <= 100.0);
        assert!(d >= 0.0 && d <= 100.0);
    }

    #[test]
    fn test_williams_r() {
        let price_points = create_test_prices();
        let williams_r = calculate_williams_r(&price_points, 3).unwrap();
        assert!(williams_r >= -100.0 && williams_r <= 0.0);
    }

    #[test]
    fn test_atr() {
        let price_points = create_test_prices();
        let atr = calculate_atr(&price_points, 3).unwrap();
        assert!(atr >= 0.0);
    }

    #[test]
    fn test_all_indicators() {
        let price_points = create_test_prices();
        let indicators = calculate_all_indicators(&price_points).unwrap();
        
        // Some indicators should have values
        assert!(indicators.macd.is_some());
        assert!(indicators.bollinger_upper.is_some());
        assert!(indicators.stochastic_k.is_some());
    }
}

// ============================================================================
// NEW SCORING FUNCTIONS: Convert raw indicators to -1.0 to +1.0 scale
// ============================================================================

/// Convert RSI to -1.0 to +1.0 score
/// RSI > 70 = overbought (negative score)
/// RSI < 30 = oversold (positive score)
/// RSI 50 = neutral (0.0)
pub fn score_rsi(rsi: f64) -> f64 {
    match rsi {
        r if r >= 70.0 => -((r - 70.0) / 30.0), // -1.0 at RSI 100
        r if r <= 30.0 => (30.0 - r) / 30.0,    // +1.0 at RSI 0
        _ => {
            // Linear interpolation between 30 and 70
            if rsi > 50.0 {
                -(rsi - 50.0) / 20.0 // -1.0 at RSI 70
            } else {
                (50.0 - rsi) / 20.0  // +1.0 at RSI 30
            }
        }
    }
}

/// Convert MACD to -1.0 to +1.0 score
/// Positive MACD = bullish (positive score)
/// Negative MACD = bearish (negative score)
pub fn score_macd(macd: f64, signal: f64) -> f64 {
    let macd_diff = macd - signal;
    
    // Normalize MACD difference to -1.0 to +1.0
    // This is a simplified approach - in practice you'd want to use historical volatility
    match macd_diff {
        d if d > 1.0 => 1.0,
        d if d < -1.0 => -1.0,
        _ => macd_diff
    }
}

/// Convert Bollinger Bands position to -1.0 to +1.0 score
/// Above upper band = overbought (negative score)
/// Below lower band = oversold (positive score)
/// Near middle = neutral (0.0)
pub fn score_bollinger_bands(price: f64, upper: f64, middle: f64, lower: f64) -> f64 {
    let band_width = upper - lower;
    if band_width == 0.0 {
        return 0.0;
    }
    
    let position = (price - middle) / band_width;
    
    match position {
        p if p > 0.5 => -1.0,  // Above upper band
        p if p < -0.5 => 1.0,  // Below lower band
        _ => -position * 2.0   // Linear scaling within bands
    }
}

/// Convert Moving Averages to -1.0 to +1.0 score
/// Price above all MAs = bullish (positive score)
/// Price below all MAs = bearish (negative score)
pub fn score_moving_averages(price: f64, sma_20: f64, sma_50: f64, sma_200: f64) -> f64 {
    let mut score = 0.0;
    
    // Weight shorter-term MAs more heavily
    if price > sma_20 { score += 0.5; } else { score -= 0.5; }
    if price > sma_50 { score += 0.3; } else { score -= 0.3; }
    if price > sma_200 { score += 0.2; } else { score -= 0.2; }
    
    // Normalize to -1.0 to +1.0
    score
}

/// Convert Stochastic to -1.0 to +1.0 score
/// %K > 80 = overbought (negative score)
/// %K < 20 = oversold (positive score)
pub fn score_stochastic(k: f64, d: f64) -> f64 {
    let avg_stoch = (k + d) / 2.0;
    
    match avg_stoch {
        s if s >= 80.0 => -((s - 80.0) / 20.0), // -1.0 at 100
        s if s <= 20.0 => (20.0 - s) / 20.0,    // +1.0 at 0
        _ => {
            if avg_stoch > 50.0 {
                -(avg_stoch - 50.0) / 30.0 // -1.0 at 80
            } else {
                (50.0 - avg_stoch) / 30.0  // +1.0 at 20
            }
        }
    }
}

/// Convert Williams %R to -1.0 to +1.0 score
/// Williams %R > -20 = overbought (negative score)
/// Williams %R < -80 = oversold (positive score)
pub fn score_williams_r(williams_r: f64) -> f64 {
    match williams_r {
        w if w >= -20.0 => (w + 20.0) / 20.0,  // +1.0 at 0
        w if w <= -80.0 => (w + 80.0) / 20.0,  // -1.0 at -100
        _ => {
            if williams_r > -50.0 {
                (williams_r + 50.0) / 30.0 // +1.0 at -20
            } else {
                (williams_r + 50.0) / 30.0 // -1.0 at -80
            }
        }
    }
}

/// Convert ATR to -1.0 to +1.0 score
/// High volatility = risk (negative score for position sizing)
/// Low volatility = opportunity (positive score for position sizing)
pub fn score_atr(atr: f64, price: f64, avg_atr: f64) -> f64 {
    let atr_ratio = atr / avg_atr;
    let price_change = atr / price;
    
    // High volatility is generally negative for position sizing
    match atr_ratio {
        r if r > 2.0 => -1.0,  // Very high volatility
        r if r < 0.5 => 0.5,   // Low volatility (opportunity)
        _ => -(atr_ratio - 1.0)        // Linear scaling around 1.0
    }
}

/// Convert Volume to -1.0 to +1.0 score
/// High volume with price movement = confirmation (positive score)
/// Low volume = lack of conviction (negative score)
pub fn score_volume(current_volume: u64, avg_volume: u64, price_change: f64) -> f64 {
    let volume_ratio = current_volume as f64 / avg_volume as f64;
    
    // Volume should confirm price movement
    if price_change > 0.0 {
        // Price going up
        match volume_ratio {
            r if r > 1.5 => 1.0,   // High volume confirms uptrend
            r if r < 0.7 => -0.5,  // Low volume weakens uptrend
            r => (r - 1.0) * 2.0   // Linear scaling
        }
    } else if price_change < 0.0 {
        // Price going down
        match volume_ratio {
            r if r > 1.5 => -1.0,  // High volume confirms downtrend
            r if r < 0.7 => 0.5,   // Low volume weakens downtrend
            r => -(r - 1.0) * 2.0  // Linear scaling (inverted)
        }
    } else {
        // No price change
        match volume_ratio {
            r if r > 1.2 => 0.2,   // Slight positive for high volume
            r if r < 0.8 => -0.2,  // Slight negative for low volume
            _ => 0.0
        }
    }
}
