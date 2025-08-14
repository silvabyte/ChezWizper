use anyhow::{Context, Result};
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::process::{Command, Stdio};
use tracing::{error, info, warn};
use which::which;

use crate::whisper::provider::TranscriptionProvider;

pub struct WhisperCppProvider {
    command_path: PathBuf,
    model_path: Option<String>,
    model: String,
}

impl WhisperCppProvider {
    pub fn new(
        command_path: Option<String>,
        model: String,
        model_path: Option<String>,
    ) -> Result<Self> {
        let command_path = if let Some(path) = command_path {
            let custom_path = PathBuf::from(path);
            if custom_path.exists() {
                info!("Using custom whisper.cpp path: {:?}", custom_path);
                custom_path
            } else {
                return Err(anyhow::anyhow!(
                    "Custom whisper path does not exist: {:?}",
                    custom_path
                ));
            }
        } else {
            which("whisper").context("Whisper CLI not found. Please install whisper.cpp")?
        };

        info!("Found whisper.cpp at: {:?}", command_path);

        Ok(Self {
            command_path,
            model_path,
            model,
        })
    }
}

impl TranscriptionProvider for WhisperCppProvider {
    fn name(&self) -> &'static str {
        "whisper.cpp"
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
        let model_path = self.model_path.clone();

        Box::pin(async move {
            info!("Using whisper.cpp to transcribe: {:?}", audio_path);
            warn!("whisper.cpp integration is experimental - consider using OpenAI whisper");

            let model_arg = if let Some(mp) = &model_path {
                info!("Using custom model path: {}", mp);
                mp.clone()
            } else {
                format!("models/ggml-{model}.bin")
            };

            let mut cmd = Command::new(&command_path);
            cmd.arg("-f")
                .arg(&audio_path)
                .arg("-m")
                .arg(&model_arg)
                .arg("-l")
                .arg(&language)
                .arg("-nt")
                .arg("-np")
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .stdin(Stdio::null());

            let output = cmd
                .output()
                .context("Failed to execute whisper.cpp command")?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                error!("Whisper.cpp failed: {}", stderr);

                warn!("Trying fallback whisper.cpp command");
                let mut cmd = Command::new(&command_path);
                cmd.arg("-f").arg(&audio_path);

                if let Some(mp) = &model_path {
                    cmd.arg("-m").arg(mp);
                }

                let output = cmd
                    .output()
                    .context("Failed to execute fallback whisper.cpp command")?;

                if !output.status.success() {
                    return Err(anyhow::anyhow!("Whisper.cpp transcription failed"));
                }

                let transcription = String::from_utf8_lossy(&output.stdout);
                return Ok(transcription.trim().to_string());
            }

            let transcription = String::from_utf8_lossy(&output.stdout);
            let transcription = transcription.trim().to_string();

            info!("Transcription complete: {} chars", transcription.len());

            Ok(transcription)
        })
    }
}
