use reqwest::Client;
use serde_json::json;
use std::env;

const GROQ_URL: &str = "https://api.groq.com/openai/v1/chat/completions";

pub async fn query_groq(
    user_input: &str,
    _session_id: &str, // Keep for future use with conversation history
) -> String {
    let client = Client::new();
    let groq_api_key = env::var("GROQ_API_KEY").expect("GROQ_API_KEY not set");

    println!("🔍 Making Groq API call with message: {}", user_input);

    let payload = json!({
        "model": "gemma2-9b-it",  // Try gemma-7b-it model instead
        "messages": [{ "role": "user", "content": user_input }],
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
