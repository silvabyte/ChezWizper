use anyhow::{Context, Result};
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::process::Command;
use tracing::{error, info};
use which::which;

use crate::whisper::provider::TranscriptionProvider;

pub struct OpenAIWhisperCliProvider {
    command_path: PathBuf,
    model: String,
}

impl OpenAIWhisperCliProvider {
    pub fn new(command_path: Option<String>, model: String) -> Result<Self> {
        let command_path = if let Some(path) = command_path {
            let custom_path = PathBuf::from(path);
            if custom_path.exists() {
                info!("Using custom OpenAI whisper path: {:?}", custom_path);
                custom_path
            } else {
                return Err(anyhow::anyhow!(
                    "Custom whisper path does not exist: {:?}",
                    custom_path
                ));
            }
        } else {
            which("whisper")
                .context("OpenAI Whisper CLI not found. Please install openai-whisper")?
        };

        let help_output = Command::new(&command_path).arg("--help").output();

        let is_openai = if let Ok(output) = help_output {
            let help_text = String::from_utf8_lossy(&output.stdout);
            help_text.contains("--output_format") && help_text.contains("--output_dir")
        } else {
            false
        };

        if !is_openai {
            return Err(anyhow::anyhow!(
                "Detected whisper CLI is not OpenAI Whisper"
            ));
        }

        info!("Detected OpenAI Whisper CLI at: {:?}", command_path);

        Ok(Self {
            command_path,
            model,
        })
    }
}

impl TranscriptionProvider for OpenAIWhisperCliProvider {
    fn name(&self) -> &'static str {
        "OpenAI Whisper CLI"
    }

    fn is_available(&self) -> bool {
        self.command_path.exists()
    }

    fn transcribe<'a>(
        &'a self,
        audio_path: &'a Path,
        language: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<String>> + Send + 'a>> {
        let audio_path = audio_path.to_path_buf();
        let language = language.to_string();
        let command_path = self.command_path.clone();
        let model = self.model.clone();

        Box::pin(async move {
            info!("Using OpenAI Whisper CLI to transcribe: {:?}", audio_path);

            let output = Command::new(&command_path)
                .arg(&audio_path)
                .arg("--model")
                .arg(&model)
                .arg("--language")
                .arg(&language)
                .arg("--output_format")
                .arg("txt")
                .arg("--output_dir")
                .arg("/tmp")
                .output()
                .context("Failed to execute whisper command")?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                error!("Whisper failed: {}", stderr);
                return Err(anyhow::anyhow!("Whisper transcription failed: {}", stderr));
            }

            let audio_stem = audio_path
                .file_stem()
                .context("Invalid audio path")?
                .to_str()
                .context("Invalid audio filename")?;

            let output_path = PathBuf::from(format!("/tmp/{audio_stem}.txt"));
            let transcription = std::fs::read_to_string(&output_path)
                .context("Failed to read transcription output")?;

            let _ = std::fs::remove_file(&output_path);

            let transcription = transcription.trim().to_string();
            info!("Transcription complete: {} chars", transcription.len());

            Ok(transcription)
        })
    }
}
