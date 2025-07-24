use anyhow::{Result, Context};
use reqwest::multipart::{Form, Part};
use serde::Deserialize;
use std::path::PathBuf;
use tracing::{debug, info, error};

#[derive(Debug, Deserialize)]
pub struct TranscriptionResponse {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

#[derive(Debug, Deserialize)]
pub struct ErrorDetail {
    pub message: String,
    pub r#type: Option<String>,
    pub code: Option<String>,
}

pub struct OpenAIClient {
    client: reqwest::Client,
    api_key: String,
    endpoint: String,
}

impl OpenAIClient {
    pub fn new(api_key: String, endpoint: Option<String>) -> Result<Self> {
        let client = reqwest::Client::new();
        let endpoint = endpoint.unwrap_or_else(|| {
            "https://api.openai.com/v1/audio/transcriptions".to_string()
        });
        
        info!("Initialized OpenAI client with endpoint: {}", endpoint);
        
        Ok(Self {
            client,
            api_key,
            endpoint,
        })
    }
    
    pub async fn transcribe(&self, audio_path: &PathBuf, model: &str, language: &str) -> Result<String> {
        info!("Transcribing audio file via OpenAI API: {:?}", audio_path);
        
        // Read audio file
        let audio_data = tokio::fs::read(audio_path).await
            .context("Failed to read audio file")?;
        
        let filename = audio_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("audio.wav");
        
        // Create multipart form
        let audio_part = Part::bytes(audio_data)
            .file_name(filename.to_string())
            .mime_str("audio/wav")
            .context("Failed to set MIME type")?;
        
        let mut form = Form::new()
            .part("file", audio_part)
            .text("model", model.to_string());
        
        // Add language if specified and not "auto"
        if !language.is_empty() && language != "auto" {
            form = form.text("language", language.to_string());
        }
        
        // Add response format
        form = form.text("response_format", "json");
        
        debug!("Sending request to OpenAI API with model: {}, language: {}", model, language);
        
        // Send request
        let response = self.client
            .post(&self.endpoint)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await
            .context("Failed to send request to OpenAI API")?;
        
        let status = response.status();
        let response_text = response.text().await
            .context("Failed to read response body")?;
        
        if !status.is_success() {
            error!("OpenAI API request failed with status {}: {}", status, response_text);
            
            // Try to parse error response
            if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&response_text) {
                return Err(anyhow::anyhow!(
                    "OpenAI API error: {} (type: {:?}, code: {:?})",
                    error_response.error.message,
                    error_response.error.r#type,
                    error_response.error.code
                ));
            }
            
            return Err(anyhow::anyhow!(
                "OpenAI API request failed with status {}: {}",
                status,
                response_text
            ));
        }
        
        // Parse successful response
        let transcription: TranscriptionResponse = serde_json::from_str(&response_text)
            .context("Failed to parse transcription response")?;
        
        let text = transcription.text.trim().to_string();
        info!("Transcription complete: {} chars", text.len());
        debug!("Raw transcription: {}", text);
        
        Ok(text)
    }
}