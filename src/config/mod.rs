use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub audio: AudioConfig,
    pub whisper: WhisperConfig,
    pub ui: UiConfig,
    pub wayland: WaylandConfig,
    pub behavior: BehaviorConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioConfig {
    pub device: String,
    pub sample_rate: u32,
    pub channels: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WhisperConfig {
    pub model: String,
    pub language: String,
    pub command_path: Option<String>,
    pub model_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UiConfig {
    pub indicator_position: String,
    pub indicator_size: u32,
    pub show_notifications: bool,
    pub layer_shell_anchor: String,
    pub layer_shell_margin: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaylandConfig {
    pub input_method: String,
    pub use_hyprland_ipc: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BehaviorConfig {
    pub auto_paste: bool,
    pub preserve_clipboard: bool,
    pub delete_audio_files: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            audio: AudioConfig {
                device: "default".to_string(),
                sample_rate: 16000,
                channels: 1,
            },
            whisper: WhisperConfig {
                model: "base".to_string(),
                language: "en".to_string(),
                command_path: None,
                model_path: None,
            },
            ui: UiConfig {
                indicator_position: "top-right".to_string(),
                indicator_size: 20,
                show_notifications: true,
                layer_shell_anchor: "top | right".to_string(),
                layer_shell_margin: 10,
            },
            wayland: WaylandConfig {
                input_method: "wtype".to_string(),
                use_hyprland_ipc: true,
            },
            behavior: BehaviorConfig {
                auto_paste: true,
                preserve_clipboard: false,
                delete_audio_files: true,
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if !config_path.exists() {
            info!("Config file not found, creating default at {:?}", config_path);
            let config = Self::default();
            config.save()?;
            return Ok(config);
        }
        
        let content = std::fs::read_to_string(&config_path)
            .context("Failed to read config file")?;
        
        let config: Self = toml::from_str(&content)
            .context("Failed to parse config file")?;
        
        info!("Loaded config from {:?}", config_path);
        Ok(config)
    }
    
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)
                .context("Failed to create config directory")?;
        }
        
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;
        
        std::fs::write(&config_path, content)
            .context("Failed to write config file")?;
        
        Ok(())
    }
    
    fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Failed to determine config directory")?;
        
        Ok(config_dir.join("chezwizper").join("config.toml"))
    }
}
