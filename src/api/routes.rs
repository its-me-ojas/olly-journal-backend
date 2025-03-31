use axum::{Json, extract::State};
use std::sync::Arc;
use parking_lot::Mutex;
use crate::models::{ChatRequest, ChatResponse, JournalResponse, JournalRequest};
use crate::services::groq::{query_groq, generate_journal_from_conversation};
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
        Some(h) if !h.trim().is_empty() => {
            // Try using AI-powered journal generation first
            println!("Attempting AI-powered journal generation");
            match generate_journal_from_conversation(&h).await {
                journal if journal.contains("Error") || journal.contains("error") => {
                    // If Groq fails, fall back to simple markdown formatting
                    println!("AI generation failed, using fallback formatting");
                    let fallback_journal = generate_markdown(&h);
                    Json(JournalResponse { journal: fallback_journal })
                }
                journal => {
                    // AI generation succeeded
                    println!("AI journal generation successful");
                    Json(JournalResponse { journal })
                }
            }
        }
        Some(_) => {
            // Empty history case
            Json(JournalResponse { 
                journal: "# Empty Journal\n\nNo conversation history found. Start chatting with Olly to create a journal entry.".to_string() 
            })
        }
        None => {
            // No session found case
            Json(JournalResponse { 
                journal: "# Session Not Found\n\nNo valid session was found. Please start a new conversation.".to_string() 
            })
        }
    }
} 