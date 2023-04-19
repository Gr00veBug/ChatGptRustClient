use reqwest::Client;
use serde_json::{json, Value};
use std::fmt;
use std::result::Result;
use tokio;
use std::io;
//Add API key below
const API_KEY: &str = "API_Key_Here";

#[derive(Debug)]
struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CustomError: {}", self.0)
    }
}

struct GptClient {
    api_key: String,
    client: Client,
}

impl GptClient {
    fn new(api_key: String) -> Self {
        GptClient {
            api_key,
            client: Client::new(),
        }
    }

    async fn generate_text(&self, prompt: &str) -> Result<String, CustomError> {
        let url = "https://api.openai.com/v1/completions";
        let headers = self.build_headers();
        let body = self.build_body(prompt);

        let response = self
            .client
            .post(url)
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(|e| CustomError(format!("Request error: {}", e)))?;

        let json: Value = response.json().await.map_err(|e| CustomError(format!("Parse error: {}", e)))?;
        let text = json["choices"][0]["text"]
            .as_str()
            .ok_or_else(|| CustomError("Failed to parse text".to_string()))?
            .to_string();

        Ok(text)
    }

    fn build_headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.api_key).parse().unwrap(),
        );
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers
    }
//You can change these parameters for different temp, different model, ect.
    fn build_body(&self, prompt: &str) -> Value {
        json!({
            "model": "text-davinci-003",
            "prompt": prompt,
            "temperature": 0.7,
            "max_tokens": 256,
            "top_p": 1,
            "frequency_penalty": 0,
            "presence_penalty": 0
        })
    }
}

#[tokio::main]
async fn main() {
    let gpt = GptClient::new(API_KEY.to_string());

    loop {
        println!("Enter your question (type 'exit' to quit):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();

        if input.to_lowercase() == "exit" {
            break;
        }

        let generated_text = gpt
            .generate_text(input)
            .await
            .expect("Failed to generate text. Make sure the API key and URL are correct and the API is returning the expected response format.");
        println!("Generated text: {}", generated_text);
    }
}
