use statrs::statistics::{Mean, Variance};

pub struct RsiIndicator {
    period: usize,
}

impl RsiIndicator {
    pub fn new(period: usize) -> Self {
        Self { period }
    }
    
    pub fn calculate(&self, prices: &[f64]) -> f64 {
        if prices.len() < self.period + 1 {
            return 50.0; // Neutral RSI when not enough data
        }
        
        let mut gains = 0.0;
        let mut losses = 0.0;
        
        for i in 1..=self.period {
            let change = prices[i] - prices[i - 1];
            if change > 0.0 {
                gains += change;
            } else {
                losses += -change;
            }
        }
        
        let avg_gain = gains / self.period as f64;
        let avg_loss = losses / self.period as f64;
        
        if avg_loss == 0.0 {
            return 100.0;
        }
        
        let rs = avg_gain / avg_loss;
        100.0 - (100.0 / (1.0 + rs))
    }
}

pub struct MacdIndicator {
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
}

impl MacdIndicator {
    pub fn new(fast_period: usize, slow_period: usize, signal_period: usize) -> Self {
        Self { fast_period, slow_period, signal_period }
    }
    
    pub fn calculate(&self, prices: &[f64]) -> (f64, f64, f64) {
        let fast_ema = self.calculate_ema(prices, self.fast_period);
        let slow_ema = self.calculate_ema(prices, self.slow_period);
        let macd = fast_ema - slow_ema;
        
        // Simplified signal line calculation
        let signal = macd * 0.15; // Approximation
        let histogram = macd - signal;
        
        (macd, signal, histogram)
    }
    
    fn calculate_ema(&self, prices: &[f64], period: usize) -> f64 {
        if prices.is_empty() {
            return 0.0;
        }
        
        let multiplier = 2.0 / (period as f64 + 1.0);
        let mut ema = prices[0];
        
        for price in prices.iter().skip(1) {
            ema = (*price - ema) * multiplier + ema;
        }
        
        ema
    }
}