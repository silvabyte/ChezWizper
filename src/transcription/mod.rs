use anyhow::Result;
use std::path::PathBuf;
use tracing::{debug, info};

use crate::normalizer::Normalizer;
use crate::whisper::WhisperTranscriber;

/// Service that orchestrates transcription and normalization
pub struct TranscriptionService {
    whisper: WhisperTranscriber,
    normalizer: Normalizer,
}

impl TranscriptionService {
    /// Create a new transcription service with the provided whisper transcriber
    pub fn new(whisper: WhisperTranscriber) -> Result<Self> {
        let normalizer = Normalizer::create(whisper.is_openai_whisper)?;

        Ok(Self {
            whisper,
            normalizer,
        })
    }

    /// Transcribe audio file and return normalized text
    pub async fn transcribe(&self, audio_path: &PathBuf) -> Result<String> {
        info!("Starting transcription pipeline for: {:?}", audio_path);

        // Step 1: Get raw transcription from whisper
        debug!("Getting raw transcription from whisper");
        let raw_transcription = self.whisper.transcribe(audio_path).await?;

        // Step 2: Normalize the transcription
        debug!("Normalizing transcription output");
        let normalized = self.normalizer.run(&raw_transcription);

        info!(
            "Transcription pipeline complete: {} chars -> {} chars",
            raw_transcription.len(),
            normalized.len()
        );

        Ok(normalized)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[tokio::test]
    async fn test_transcription_service_creation() {
        //TODO: implement this
        // NOTE:: This would require mocking WhisperTranscriber
    }
}
