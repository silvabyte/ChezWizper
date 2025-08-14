use anyhow::{Context, Result};
use std::path::PathBuf;
use tracing::{info, warn};

mod provider;
mod providers;

use provider::TranscriptionProvider;
use providers::{OpenAIProvider, OpenAIWhisperCliProvider, WhisperCppProvider};

pub struct WhisperTranscriber {
    provider: Box<dyn TranscriptionProvider>,
    language: String,
}

impl WhisperTranscriber {
    pub fn auto_detect(config: ProviderConfig) -> Result<Self> {
        let language = config.language.unwrap_or_else(|| "en".to_string());
        let provider = Self::auto_detect_provider(config.command_path)?;

        Ok(Self { provider, language })
    }

    pub fn with_provider(provider_name: &str, config: ProviderConfig) -> Result<Self> {
        let language = config.language.clone().unwrap_or_else(|| "en".to_string());

        let provider: Box<dyn TranscriptionProvider> = match provider_name {
            "openai-api" => {
                let api_key = std::env::var("OPENAI_API_KEY").context(
                    "OPENAI_API_KEY environment variable is required for OpenAI API provider",
                )?;

                let model = config.model.unwrap_or_else(|| "whisper-1".to_string());
                Box::new(OpenAIProvider::new(api_key, config.api_endpoint, model)?)
            }
            "openai-cli" => {
                let model = config.model.unwrap_or_else(|| "base".to_string());
                Box::new(OpenAIWhisperCliProvider::new(config.command_path, model)?)
            }
            "whisper-cpp" => {
                let model = config.model.unwrap_or_else(|| "base".to_string());
                Box::new(WhisperCppProvider::new(
                    config.command_path,
                    model,
                    config.model_path,
                )?)
            }
            _ => {
                warn!("Unknown provider '{}', using auto-detection", provider_name);
                Self::auto_detect_provider(config.command_path)?
            }
        };

        info!("Using {} for transcription", provider.name());

        Ok(Self { provider, language })
    }

    fn auto_detect_provider(custom_path: Option<String>) -> Result<Box<dyn TranscriptionProvider>> {
        info!("Auto-detecting transcription provider...");

        if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
            if !api_key.is_empty() {
                if let Ok(provider) = OpenAIProvider::new(api_key, None, "whisper-1".to_string()) {
                    info!("Auto-detected: OpenAI API (OPENAI_API_KEY found)");
                    return Ok(Box::new(provider));
                }
            }
        }

        if let Ok(provider) = OpenAIWhisperCliProvider::new(custom_path.clone(), "base".to_string())
        {
            if provider.is_available() {
                info!("Auto-detected: OpenAI Whisper CLI");
                return Ok(Box::new(provider));
            }
        }

        if let Ok(provider) = WhisperCppProvider::new(custom_path, "base".to_string(), None) {
            if provider.is_available() {
                info!("Auto-detected: whisper.cpp");
                return Ok(Box::new(provider));
            }
        }

        Err(anyhow::anyhow!(
            "No transcription provider available. Install whisper-cpp, openai-whisper, or set OPENAI_API_KEY"
        ))
    }

    pub async fn transcribe(&self, audio_path: &PathBuf) -> Result<String> {
        info!(
            "Transcribing audio file: {:?} with {}",
            audio_path,
            self.provider.name()
        );
        self.provider
            .transcribe(audio_path.as_path(), &self.language)
            .await
    }

    pub fn is_openai_whisper(&self) -> bool {
        self.provider.name() == "OpenAI Whisper CLI"
    }
}

#[derive(Debug, Clone)]
pub struct ProviderConfig {
    pub model: Option<String>,
    pub model_path: Option<String>,
    pub language: Option<String>,
    pub command_path: Option<String>,
    pub api_endpoint: Option<String>,
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            model: None,
            model_path: None,
            language: Some("en".to_string()),
            command_path: None,
            api_endpoint: None,
        }
    }
}
