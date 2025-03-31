use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct ChatRequest {
    pub session_id: Option<String>,
    pub message: String,
}

#[derive(Deserialize)]
pub struct JournalRequest {
    pub session_id: Option<String>,
}

#[derive(Serialize)]
pub struct ChatResponse {
    pub session_id: String,
    pub response: String,
}

#[derive(Serialize)]
pub struct JournalResponse {
    pub journal: String,
} 