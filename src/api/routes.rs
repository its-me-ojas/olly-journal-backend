use axum::{Json, extract::State};
use std::sync::Arc;
use parking_lot::Mutex;
use crate::models::{ChatRequest, ChatResponse, JournalResponse, JournalRequest};
use crate::services::groq::query_groq;
use crate::services::session::SessionStore;
use crate::services::journal::generate_markdown;

#[axum::debug_handler]
pub async fn chat_handler(
    State(session_store): State<Arc<Mutex<SessionStore>>>,
    Json(payload): Json<ChatRequest>,
) -> Json<ChatResponse> {
    // Get session ID first
    let session_id = {
        let mut store = session_store.lock();
        payload.session_id.unwrap_or_else(|| store.create_session())
    };

    // Make the API call
    let response = query_groq(&payload.message, &session_id).await;

    // Update session after API call
    {
        let mut store = session_store.lock();
        store.append_to_session(&session_id, &payload.message, &response);
    }

    Json(ChatResponse {
        session_id,
        response,
    })
}

#[axum::debug_handler]
pub async fn generate_journal(
    State(session_store): State<Arc<Mutex<SessionStore>>>,
    Json(payload): Json<JournalRequest>,
) -> Json<JournalResponse> {
    let history = {
        let store = session_store.lock();
        let session_id = payload.session_id.unwrap_or_else(|| "".to_string());
        store.get_session(&session_id).map(|h| h.to_string())
    };

    match history {
        Some(h) => {
            let markdown = generate_markdown(&h);
            Json(JournalResponse { journal: markdown })
        }
        None => Json(JournalResponse { journal: "No history found.".to_string() })
    }
} 