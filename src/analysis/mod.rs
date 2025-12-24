pub mod indicators;
pub mod patterns;
pub mod signals;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalIndicators {
    pub symbol: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub rsi: Option<f64>,
    pub macd: Option<MacdData>,
    pub bollinger_bands: Option<BollingerBands>,
    pub moving_averages: Option<MovingAverages>,
    pub stochastic: Option<StochasticData>,
    pub adx: Option<f64>,
    pub atr: Option<f64>,
    pub obv: Option<f64>,
    pub vwap: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacdData {
    pub macd: f64,
    pub signal: f64,
    pub histogram: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BollingerBands {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovingAverages {
    pub sma_20: f64,
    pub sma_50: f64,
    pub sma_200: f64,
    pub ema_12: f64,
    pub ema_26: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StochasticData {
    pub k: f64,
    pub d: f64,
}