use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub id: uuid::Uuid,
    pub name: String,
    pub user_id: uuid::Uuid,
    pub total_value: f64,
    pub total_gain_loss: f64,
    pub total_gain_loss_percent: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioHolding {
    pub id: uuid::Uuid,
    pub portfolio_id: uuid::Uuid,
    pub symbol: String,
    pub shares: f64,
    pub average_price: f64,
    pub current_price: f64,
    pub total_value: f64,
    pub gain_loss: f64,
    pub gain_loss_percent: f64,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskMetrics {
    pub portfolio_id: uuid::Uuid,
    pub var_95: f64,
    pub var_99: f64,
    pub sharpe_ratio: f64,
    pub beta: f64,
    pub alpha: f64,
    pub max_drawdown: f64,
    pub volatility: f64,
    pub calculated_at: chrono::DateTime<chrono::Utc>,
}