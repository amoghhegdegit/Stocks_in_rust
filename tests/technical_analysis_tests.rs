#[cfg(test)]
mod tests {
    use stock_analyst::analysis::indicators::*;
    
    #[test]
    fn test_rsi_calculation() {
        let prices = vec![100.0, 102.0, 101.0, 103.0, 104.0, 102.0, 105.0];
        let rsi = RsiIndicator::new(14);
        let result = rsi.calculate(&prices);
        
        assert!(result >= 0.0 && result <= 100.0);
    }
    
    #[test]
    fn test_macd_calculation() {
        let prices = vec![100.0, 102.0, 101.0, 103.0, 104.0, 102.0, 105.0];
        let macd = MacdIndicator::new(12, 26, 9);
        let (macd_line, signal_line, histogram) = macd.calculate(&prices);
        
        assert!(macd_line.is_finite());
        assert!(signal_line.is_finite());
        assert!(histogram.is_finite());
    }
}