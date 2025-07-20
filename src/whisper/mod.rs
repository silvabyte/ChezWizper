use anyhow::{Result, Context};
use std::path::PathBuf;
use std::process::Command;
use tracing::{debug, info, error, warn};
use which::which;

pub struct WhisperTranscriber {
    command_path: PathBuf,
    model: String,
    language: String,
    is_openai_whisper: bool,
}

impl WhisperTranscriber {
    pub fn new() -> Result<Self> {
        let command_path = which("whisper")
            .context("Whisper CLI not found. Please install whisper-cpp or openai-whisper")?;
        
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
            info!("Detected OpenAI Whisper");
        } else {
            info!("Detected whisper.cpp or other implementation");
        }
        
        Ok(Self {
            command_path,
            model: "base".to_string(),
            language: "en".to_string(),
            is_openai_whisper: is_openai,
        })
    }
    
    pub fn with_model(mut self, model: String) -> Self {
        self.model = model;
        self
    }
    
    pub fn with_language(mut self, language: String) -> Self {
        self.language = language;
        self
    }
    
    pub async fn transcribe(&self, audio_path: &PathBuf) -> Result<String> {
        info!("Transcribing audio file: {:?}", audio_path);
        
        if self.is_openai_whisper {
            self.transcribe_openai_whisper(audio_path).await
        } else {
            self.transcribe_whisper_cpp(audio_path).await
        }
    }
    
    async fn transcribe_openai_whisper(&self, audio_path: &PathBuf) -> Result<String> {
        info!("Using OpenAI Whisper");
        
        let output = Command::new(&self.command_path)
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
        let audio_stem = audio_path.file_stem()
            .context("Invalid audio path")?
            .to_str()
            .context("Invalid audio filename")?;
        
        let output_path = PathBuf::from(format!("/tmp/{}.txt", audio_stem));
        let transcription = std::fs::read_to_string(&output_path)
            .context("Failed to read transcription output")?;
        
        // Clean up output file
        let _ = std::fs::remove_file(&output_path);
        
        let transcription = transcription.trim().to_string();
        info!("Transcription complete: {} chars", transcription.len());
        debug!("Transcription: {}", transcription);
        
        Ok(transcription)
    }
    
    async fn transcribe_whisper_cpp(&self, audio_path: &PathBuf) -> Result<String> {
        info!("Using whisper.cpp");
        warn!("whisper.cpp integration is experimental - consider using OpenAI whisper");
        
        // For whisper.cpp, we'll capture stdout directly
        let output = Command::new(&self.command_path)
            .arg("-f")
            .arg(audio_path)
            .arg("-m")
            .arg(&format!("models/ggml-{}.bin", self.model))
            .arg("-l")
            .arg(&self.language)
            .arg("--output-txt")
            .output()
            .context("Failed to execute whisper.cpp command")?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("Whisper.cpp failed: {}", stderr);
            
            // Fallback: try simpler command
            warn!("Trying fallback whisper.cpp command");
            let output = Command::new(&self.command_path)
                .arg("-f")
                .arg(audio_path)
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
        debug!("Transcription: {}", transcription);
        
        Ok(transcription)
    }
}