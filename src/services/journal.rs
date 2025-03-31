use chrono;

pub fn generate_markdown(chat_history: &str) -> String {
    format!(
        "## Journal Entry - {}\n\n{}",
        chrono::Local::now().format("%B %d, %Y"),
        chat_history.replace("\n", "\n- ")
    )
} 