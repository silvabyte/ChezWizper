use anyhow::{Context, Result};
use reqwest::multipart::{Form, Part};
use serde::Deserialize;
use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use tracing::{debug, error, info};

use crate::whisper::provider::TranscriptionProvider;

#[derive(Debug, Deserialize)]
struct TranscriptionResponse {
    text: String,
}

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    error: ErrorDetail,
}

#[derive(Debug, Deserialize)]
struct ErrorDetail {
    message: String,
    r#type: Option<String>,
    code: Option<String>,
}

pub struct OpenAIProvider {
    client: reqwest::Client,
    api_key: String,
    endpoint: String,
    model: String,
}

impl OpenAIProvider {
    pub fn new(api_key: String, endpoint: Option<String>, model: String) -> Result<Self> {
        let client = reqwest::Client::new();
        let endpoint = endpoint
            .unwrap_or_else(|| "https://api.openai.com/v1/audio/transcriptions".to_string());

        info!("Initialized OpenAI provider with endpoint: {}", endpoint);

        Ok(Self {
            client,
            api_key,
            endpoint,
            model,
        })
    }
}

impl TranscriptionProvider for OpenAIProvider {
    fn name(&self) -> &'static str {
        "OpenAI API"
    }

    fn is_available(&self) -> bool {
        !self.api_key.is_empty()
    }

    fn transcribe<'a>(
        &'a self,
        audio_path: &'a Path,
        language: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<String>> + Send + 'a>> {
        Box::pin(async move {
            info!("Transcribing audio file via OpenAI API: {:?}", audio_path);

            let audio_data = tokio::fs::read(audio_path)
                .await
                .context("Failed to read audio file")?;

            let filename = audio_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("audio.wav");

            let audio_part = Part::bytes(audio_data)
                .file_name(filename.to_string())
                .mime_str("audio/wav")
                .context("Failed to set MIME type")?;

            let mut form = Form::new()
                .part("file", audio_part)
                .text("model", self.model.clone());

            if !language.is_empty() && language != "auto" {
                form = form.text("language", language.to_string());
            }

            form = form.text("response_format", "json");

            debug!(
                "Sending request to OpenAI API with model: {}, language: {}",
                self.model, language
            );

            let response = self
                .client
                .post(&self.endpoint)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .multipart(form)
                .send()
                .await
                .context("Failed to send request to OpenAI API")?;

            let status = response.status();
            let response_text = response
                .text()
                .await
                .context("Failed to read response body")?;

            if !status.is_success() {
                error!(
                    "OpenAI API request failed with status {}: {}",
                    status, response_text
                );

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

            let transcription: TranscriptionResponse = serde_json::from_str(&response_text)
                .context("Failed to parse transcription response")?;

            let text = transcription.text.trim().to_string();
            info!("Transcription complete: {} chars", text.len());
            debug!("Raw transcription: {}", text);

            Ok(text)
        })
    }
}
