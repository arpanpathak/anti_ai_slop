use anyhow::{Context, Result};
use serde_json::{json, Value};
use std::fs;
use ureq;

/// A client for interacting with the LLM API.
pub struct ApiClient {
    /// The endpoint URL for the chat completions.
    pub url: String,
    /// The bearer token for authentication.
    pub api_key: String,
}

impl ApiClient {
    /// Creates a new `ApiClient`.
    ///
    /// # Arguments
    /// * `url` - The API endpoint URL.
    /// * `api_key` - The authentication token.
    pub fn new(url: String, api_key: String) -> Self {
        Self { url, api_key }
    }

    /// Sends a conversation to the LLM and returns the raw text response.
    ///
    /// # Arguments
    /// * `conversation` - A slice of JSON values representing the message history.
    ///
    /// # Returns
    /// A `Result` containing the string content of the assistant's reply.
    pub fn send(&self, conversation: &[Value]) -> Result<String> {
        let request = json!({
            "model": crate::models::MODEL,
            "messages": conversation,
            "temperature": 0.2
        });

        let request_path = format!("{}/request-latest.json", crate::models::WORK_DIR);
        fs::write(&request_path, serde_json::to_string_pretty(&request)?)?;

        let response: Value = ureq::post(&self.url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send_json(request)?
            .into_body()
            .read_json()?;

        let content = response["choices"][0]["message"]["content"]
            .as_str()
            .context("the reply has no content")?;

        Ok(content.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_json() {
        let reply = r#"{"files": [{"file_name": "test.rs", "code": "fn main() {}", "check": "cargo run"}]}"#;
        let answer = crate::models::Answer::parse(reply).unwrap();
        assert_eq!(answer.files.len(), 1);
        assert_eq!(answer.files[0].file_name, "test.rs");
    }

    #[test]
    fn test_parse_json_with_markdown() {
        let reply = r#"Here is the code:
```json
{"files": [{"file_name": "test.rs", "code": "fn main() {}", "check": "cargo run"}]}
```"#;
        let answer = crate::models::Answer::parse(reply).unwrap();
        assert_eq!(answer.files.len(), 1);
        assert_eq!(answer.files[0].file_name, "test.rs");
    }

    #[test]
    fn test_parse_invalid_json() {
        let reply = r#"{"invalid": "json"}"#;
        let result = crate::models::Answer::parse(reply);
        assert!(result.is_err());
    }
}
