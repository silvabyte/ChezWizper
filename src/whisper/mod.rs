use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use tracing::{error, info, warn};
use which::which;

mod api_client;
use api_client::OpenAIClient;

pub struct WhisperTranscriber {
    command_path: Option<PathBuf>,
    pub model: String,
    model_path: Option<String>,
    language: String,
    pub is_openai_whisper: bool,
    pub use_api: bool,
    api_client: Option<OpenAIClient>,
}

impl WhisperTranscriber {
    pub fn new(custom_path: Option<String>, use_api: bool, api_endpoint: Option<String>) -> Result<Self> {
        let (command_path, is_openai_whisper, api_client) = if use_api {
            // API mode - get API key from environment
            let api_key = std::env::var("OPENAI_API_KEY")
                .context("OPENAI_API_KEY environment variable is required when use_api is true")?;
            
            let client = OpenAIClient::new(api_key, api_endpoint)?;
            info!("Using OpenAI API for transcription");
            
            (None, false, Some(client))
        } else {
            // CLI mode
            let command_path = if let Some(path) = custom_path {
                let custom_path = PathBuf::from(path);
                if custom_path.exists() {
                    info!("Using custom whisper path: {:?}", custom_path);
                    custom_path
                } else {
                    return Err(anyhow::anyhow!("Custom whisper path does not exist: {:?}", custom_path));
                }
            } else {
                which("whisper")
                    .context("Whisper CLI not found. Please install whisper-cpp or openai-whisper")?
            };
            
            info!("Found whisper at: {:?}", command_path);
            
            // Detect if this is OpenAI whisper by checking help output
            let help_output = Command::new(&command_path)
                .arg("--help")
                .output();
            
            let is_openai = if let Ok(output) = help_output {
                let help_text = String::from_utf8_lossy(&output.stdout);
                help_text.contains("--output_format") && help_text.contains("--output_dir")
            } else {
                false
            };
            
            if is_openai {
                info!("Detected OpenAI Whisper CLI");
            } else {
                info!("Detected whisper.cpp or other implementation");
            }
            
            (Some(command_path), is_openai, None)
        };
        Ok(Self {
            command_path,
            model: if use_api { "whisper-1".to_string() } else { "base".to_string() },
            model_path: None,
            language: "en".to_string(),
            is_openai_whisper,
            use_api,
            api_client,
        })
    }

    pub fn with_model(mut self, model: String) -> Self {
        self.model = model;
        self
    }

    pub fn with_model_path(mut self, model_path: Option<String>) -> Self {
        self.model_path = model_path;
        self
    }

    pub fn with_language(mut self, language: String) -> Self {
        self.language = language;
        self
    }

    pub async fn transcribe(&self, audio_path: &PathBuf) -> Result<String> {
        info!("Transcribing audio file: {:?}", audio_path);
        
        if self.use_api {
            self.transcribe_api(audio_path).await
        } else if self.is_openai_whisper {
            self.transcribe_openai_whisper(audio_path).await
        } else {
            self.transcribe_whisper_cpp(audio_path).await
        }
    }
    
    async fn transcribe_api(&self, audio_path: &PathBuf) -> Result<String> {
        let client = self.api_client.as_ref()
            .context("API client not initialized")?;
        
        client.transcribe(audio_path, &self.model, &self.language).await
    }
    
    async fn transcribe_openai_whisper(&self, audio_path: &PathBuf) -> Result<String> {
        info!("Using OpenAI Whisper CLI");
        
        let command_path = self.command_path.as_ref()
            .context("Command path not set for CLI mode")?;
        
        let output = Command::new(command_path)
            .arg(audio_path)
            .arg("--model")
            .arg(&self.model)
            .arg("--language")
            .arg(&self.language)
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

        // Read the output file
        let audio_stem = audio_path
            .file_stem()
            .context("Invalid audio path")?
            .to_str()
            .context("Invalid audio filename")?;

        let output_path = PathBuf::from(format!("/tmp/{audio_stem}.txt"));
        let transcription =
            std::fs::read_to_string(&output_path).context("Failed to read transcription output")?;

        // Clean up output file
        let _ = std::fs::remove_file(&output_path);

        let transcription = transcription.trim().to_string();
        info!("Transcription complete: {} chars", transcription.len());

        Ok(transcription)
    }

    async fn transcribe_whisper_cpp(&self, audio_path: &PathBuf) -> Result<String> {
        info!("Using whisper.cpp");
        warn!("whisper.cpp integration is experimental - consider using OpenAI whisper");
        
        let command_path = self.command_path.as_ref()
            .context("Command path not set for CLI mode")?;
        let model_arg = if let Some(model_path) = &self.model_path {
            info!("Using custom model path: {}", model_path);
            model_path.clone()
        } else {
            format!("models/ggml-{}.bin", self.model)
        };

        // For whisper.cpp, we'll capture stdout directly
        let mut cmd = Command::new(command_path);
        cmd.arg("-f")
            .arg(audio_path)
            .arg("-m")
            .arg(&model_arg)
            .arg("-l")
            .arg(&self.language)
            .arg("-nt") // No timestamps
            .arg("-np") // No progress
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::null()); // Explicitly set stdin to null

        let output = cmd
            .output()
            .context("Failed to execute whisper.cpp command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("Whisper.cpp failed: {}", stderr);

            // Fallback: try simpler command
            warn!("Trying fallback whisper.cpp command");
            let mut cmd = Command::new(command_path);
            cmd.arg("-f").arg(audio_path);

            // Add model arg to fallback if we have a custom path
            if let Some(model_path) = &self.model_path {
                cmd.arg("-m").arg(model_path);
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
    }
}
