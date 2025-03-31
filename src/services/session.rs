use std::collections::HashMap;
use uuid::Uuid;

pub struct SessionStore {
    sessions: HashMap<String, String>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self { sessions: HashMap::new() }
    }

    pub fn create_session(&mut self) -> String {
        let session_id = Uuid::new_v4().to_string();
        self.sessions.insert(session_id.clone(), "".to_string());
        session_id
    }

    pub fn get_session(&self, session_id: &str) -> Option<&String> {
        self.sessions.get(session_id)
    }

    pub fn append_to_session(&mut self, session_id: &str, user_msg: &str, ai_msg: &str) {
        if let Some(history) = self.sessions.get_mut(session_id) {
            *history = format!("{}\nUser: {}\nAI: {}", history, user_msg, ai_msg);
        }
    }
} 