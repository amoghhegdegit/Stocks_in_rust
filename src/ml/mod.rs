pub mod models;
pub mod training;
pub mod evaluation;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prediction {
    pub symbol: String,
    pub prediction_date: chrono::DateTime<chrono::Utc>,
    pub target_date: chrono::DateTime<chrono::Utc>,
    pub predicted_price: f64,
    pub confidence: f64,
    pub model_type: String,
    pub features_used: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPerformance {
    pub model_type: String,
    pub mse: f64,
    pub mae: f64,
    pub rmse: f64,
    pub r_squared: f64,
    pub accuracy: f64,
    pub training_date: chrono::DateTime<chrono::Utc>,
}