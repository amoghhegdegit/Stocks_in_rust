use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EducationalContent {
    pub id: uuid::Uuid,
    pub title: String,
    pub topic: String,
    pub difficulty: DifficultyLevel,
    pub content: String,
    pub estimated_time_minutes: u32,
    pub prerequisites: Vec<String>,
    pub tags: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DifficultyLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Simulation {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub initial_balance: f64,
    pub current_balance: f64,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProgress {
    pub user_id: uuid::Uuid,
    pub content_id: uuid::Uuid,
    pub completed: bool,
    pub score: Option<f64>,
    pub time_spent_minutes: u32,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}