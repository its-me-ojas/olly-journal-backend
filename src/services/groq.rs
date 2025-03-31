use reqwest::Client;
use serde_json::json;
use std::env;

const GROQ_URL: &str = "https://api.groq.com/openai/v1/chat/completions";

// Olly's persona as a system prompt
const SYSTEM_PROMPT: &str = "You are Olly, a warm, friendly, and emotionally intelligent AI designed to be a personal companion for daily reflections. You chat like a close friend—empathetic, engaging, and thoughtful. Your responses feel human-like, caring, and supportive, rather than robotic or generic.

Your personality traits:
- Friendly, warm, and understanding
- Encouraging but not overly positive—realistic and thoughtful
- Uses casual yet articulate language (like a friend who really listens)
- Occasionally adds light humor, emojis, or affirmations to create warmth
- Avoids cold, factual responses—always adds a personal touch

In your responses:
- Always ask gentle follow-up questions to keep the conversation natural
- Occasionally reference past conversations (if context is available)
- Encourage self-reflection, but never force advice—let the user lead
- Use emojis sparingly but effectively to add warmth

For journal entries, create Markdown-formatted summaries in a journal-friendly style that capture the essence of the conversation.";

// Journal Generation System Prompt
const JOURNAL_SYSTEM_PROMPT: &str = "You are Olly, an AI journaling assistant. Your task is to transform a conversation into a thoughtful, reflective journal entry written ENTIRELY from the user's perspective in the first person.

CRITICAL INSTRUCTIONS:
1. NEVER include the back-and-forth conversation format in the journal
2. NEVER use 'Olly' or 'you' as a speaker or reference the AI directly
3. Write EXCLUSIVELY as if the user is writing about their own thoughts and feelings
4. Create a PERSONAL journal entry that reads as if written by the user themselves
5. Extract key themes, emotions, and insights from what the USER said in the conversation
6. Format as a proper Markdown document with meaningful sections

The journal should be structured like:
- A title with the date
- A personal reflection section (how the user feels, what they thought about)
- Key insights or takeaways in the user's own words
- Forward-looking thoughts

The journal should read as if the user wrote it themselves after reflecting on their day, with no indication that it came from a conversation with an AI. Maintain the user's authentic voice, perspective, and emotional tone throughout.

NEVER include any indication of a conversation format (like 'Me:' or 'Olly:') in the journal.";

pub async fn query_groq(
    user_input: &str,
    _session_id: &str, // Keep for future use with conversation history
) -> String {
    let client = Client::new();
    let groq_api_key = env::var("GROQ_API_KEY").expect("GROQ_API_KEY not set");

    println!("🔍 Making Groq API call with message: {}", user_input);

    let payload = json!({
        "model": "gemma2-9b-it",
        "messages": [
            {
                "role": "system",
                "content": SYSTEM_PROMPT
            },
            {
                "role": "user",
                "content": user_input
            }
        ],
        "temperature": 0.7,
        "max_tokens": 1024
    });

    println!(
        "📤 Payload: {}",
        serde_json::to_string_pretty(&payload).unwrap()
    );

    let res = client
        .post(GROQ_URL)
        .header("Authorization", format!("Bearer {}", groq_api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<serde_json::Value>().await {
                    Ok(response_text) => {
                        println!(
                            "📥 Response: {}",
                            serde_json::to_string_pretty(&response_text).unwrap()
                        );

                        if let Some(content) = response_text
                            .get("choices")
                            .and_then(|choices| choices.get(0))
                            .and_then(|choice| choice.get("message"))
                            .and_then(|message| message.get("content"))
                            .and_then(|content| content.as_str())
                        {
                            content.to_string()
                        } else {
                            let error_msg = "Unable to parse AI response structure".to_string();
                            println!("❌ Error: {}", error_msg);
                            error_msg
                        }
                    }
                    Err(e) => {
                        let error_msg = format!("Failed to parse JSON response: {}", e);
                        println!("❌ Error: {}", error_msg);
                        error_msg
                    }
                }
            } else {
                let error_msg = format!("API returned error status: {}", response.status());
                println!("❌ Error: {}", error_msg);

                // Try to get error message from response
                match response.text().await {
                    Ok(text) => {
                        println!("Error response: {}", text);
                        format!("API error: {}", text)
                    }
                    Err(_) => error_msg,
                }
            }
        }
        Err(e) => {
            let error_msg = format!("Error contacting AI: {}", e);
            println!("❌ Error: {}", error_msg);
            error_msg
        }
    }
}

// Function to generate a journal entry from conversation history
pub async fn generate_journal_from_conversation(conversation_history: &str) -> String {
    let client = Client::new();
    let groq_api_key = env::var("GROQ_API_KEY").expect("GROQ_API_KEY not set");
    let date = chrono::Local::now().format("%B %d, %Y").to_string();

    println!("🔍 Generating journal from conversation history");

    // Format the prompt to include instructions for journal creation
    let journal_prompt = format!(
        "Based on the following conversation, create a journal entry from the user's perspective dated {}. \
        Include insights, reflections, and emotions expressed during our conversation. \
        Format it as a well-structured Markdown document with appropriate sections.\n\n\
        Here's the conversation:\n{}", 
        date, 
        conversation_history
    );

    let payload = json!({
        "model": "gemma2-9b-it",
        "messages": [
            {
                "role": "system",
                "content": JOURNAL_SYSTEM_PROMPT
            },
            {
                "role": "user",
                "content": journal_prompt
            }
        ],
        "temperature": 0.7,
        "max_tokens": 2048  // Larger token limit for journal entries
    });

    println!(
        "📤 Journal Generation Payload: {}",
        serde_json::to_string_pretty(&payload).unwrap()
    );

    let res = client
        .post(GROQ_URL)
        .header("Authorization", format!("Bearer {}", groq_api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<serde_json::Value>().await {
                    Ok(response_text) => {
                        if let Some(content) = response_text
                            .get("choices")
                            .and_then(|choices| choices.get(0))
                            .and_then(|choice| choice.get("message"))
                            .and_then(|message| message.get("content"))
                            .and_then(|content| content.as_str())
                        {
                            // Ensure we have a proper title and footer
                            let journal_content = format!(
                                "{}\n\n---\n\n*Generated by Olly, your AI journaling companion*",
                                content
                            );
                            journal_content
                        } else {
                            let error_msg = "Unable to parse AI journal response".to_string();
                            println!("❌ Error: {}", error_msg);
                            error_msg
                        }
                    }
                    Err(e) => {
                        let error_msg = format!("Failed to parse journal JSON response: {}", e);
                        println!("❌ Error: {}", error_msg);
                        error_msg
                    }
                }
            } else {
                let error_msg = format!("API returned error status for journal: {}", response.status());
                println!("❌ Error: {}", error_msg);

                // Try to get error message from response
                match response.text().await {
                    Ok(text) => {
                        println!("Journal error response: {}", text);
                        format!("API error generating journal: {}", text)
                    }
                    Err(_) => error_msg,
                }
            }
        }
        Err(e) => {
            let error_msg = format!("Error contacting AI for journal: {}", e);
            println!("❌ Error: {}", error_msg);
            error_msg
        }
    }
}
