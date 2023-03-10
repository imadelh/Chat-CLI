use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::VecDeque;
use std::env::VarError;
use std::fs::File;
use std::io::Error;
use std::io::{BufWriter, Write};
use termion::{clear, color, cursor};

pub const OPENAI_DEFAULT_ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum ChatError {
    #[error("Missing environement variables - define OPENAI_TOKEN")]
    MissingEnvVariable,
    #[error("API Error. Message content {0}")]
    APIError(String),
    #[error("JSON decoding error: {0}")]
    JsonError(String),
}

// A message struct
#[derive(Serialize, Deserialize, Debug)]
pub struct ChatGPTMessage {
    pub role: String, // TODO: switch to enum
    pub content: String,
}

impl ChatGPTMessage {
    pub fn save(&self, file: &File) -> Result<(), Error> {
        // Save a message to a file
        let json_str = serde_json::to_string(&self)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(&"\n".as_bytes())?;
        writer.write_all(json_str.as_bytes())?;
        writer.flush()
    }
}

// API Call struct and implementation
#[derive(Serialize, Deserialize, Debug)]
pub struct ChatGPTCall {
    pub model: String,
    pub messages: VecDeque<ChatGPTMessage>,
    pub temperature: f32,
}

impl ChatGPTCall {
    pub fn push(&mut self, item: ChatGPTMessage) {
        static MAX_SIZE_CONVERSATION: usize = 5;

        self.messages.push_back(item);
        if self.messages.len() > MAX_SIZE_CONVERSATION {
            self.messages.pop_front();
        }
    }

    pub fn api_call(
        &self,
        endpoint: &str,
        token: &Result<String, VarError>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        println!("{}Running...", color::Fg(color::Magenta));
        let client = Client::new();

        let response: reqwest::blocking::Response = match endpoint {
            OPENAI_DEFAULT_ENDPOINT => {
                // Use the token in case of OpenAI endpoint
                let s = match token.as_ref() {
                    Ok(s) => s,
                    Err(_e) => return Err(ChatError::MissingEnvVariable.to_string().into()),
                };

                let response = client
                    .post(endpoint)
                    .header("Authorization", format!("Bearer {}", s))
                    .json(&json!(&self))
                    .send();

                match response {
                    Ok(s) => s,
                    Err(e) => return Err(Box::new(ChatError::APIError(e.to_string()))),
                }
            }
            _ => {
                // No token for other APIs
                let response = client.post(endpoint).json(&json!(&self)).send();
                match response {
                    Ok(s) => s,
                    Err(e) => return Err(Box::new(ChatError::APIError(e.to_string()))),
                }
            }
        };
        let body = response.text()?;
        let json_body: Value = match serde_json::from_str(&body) {
            Ok(s) => s,
            Err(_e) => return Err(Box::new(ChatError::JsonError(body))),
        };

        let content_response = json_body["choices"][0]["message"]["content"]
            .as_str()
            .unwrap();
        print!("{}{}", cursor::Up(1), clear::CurrentLine);
        Ok(content_response.to_string())
    }
}
