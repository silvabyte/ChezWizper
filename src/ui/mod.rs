use anyhow::Result;
use std::process::Command;
use tracing::{debug, info, warn};

pub struct Indicator {
    audio_feedback_enabled: bool,
}

impl Indicator {
    pub fn new() -> Self {
        Self {
            audio_feedback_enabled: true,
        }
    }
    
    pub fn with_audio_feedback(mut self, enabled: bool) -> Self {
        self.audio_feedback_enabled = enabled;
        self
    }
    
    pub async fn show_recording(&self) -> Result<()> {
        info!("Showing recording indicator");
        
        if let Err(e) = self.hyprland_notify("🔴 Recording...") {
            debug!("Hyprland notification failed: {}", e);
        }
        
        // Play recording start sound
        self.play_sound("start").await;
        
        Ok(())
    }
    
    pub async fn show_processing(&self) -> Result<()> {
        info!("Showing processing indicator");
        
        if let Err(e) = self.hyprland_notify("⏳Processing...") {
            debug!("Hyprland notification failed: {}", e);
        }
        
        // Play recording stop sound
        self.play_sound("stop").await;
        
        Ok(())
    }
    
    pub async fn show_complete(&self, text: &str) -> Result<()> {
        info!("Showing completion indicator");
        
        let preview = if text.len() > 50 {
            format!("{}...", &text[..50])
        } else {
            text.to_string()
        };
        
        if let Err(e) = self.hyprland_notify(&format!("✅ {}", preview)) {
            debug!("Hyprland notification failed: {}", e);
        }
        
        // Play completion sound
        self.play_sound("complete").await;
        
        Ok(())
    }
    
    pub async fn show_error(&self, error: &str) -> Result<()> {
        warn!("Showing error: {}", error);
        
        if let Err(e) = self.hyprland_notify(&format!("Error: {}", error)) {
            debug!("Hyprland notification failed: {}", e);
        }
        
        Ok(())
    }
    
    fn hyprland_notify(&self, title: &str) -> Result<()> {
        Command::new("hyprctl")
            .args(["notify", "-1", "3000", "rgb(ff1744)", title])
            .output()?;
        
        Ok(())
    }
    
    async fn play_sound(&self, sound_type: &str) {
        if !self.audio_feedback_enabled {
            return;
        }
        
        debug!("Playing {} sound", sound_type);
        
        // Use a simple approach with system commands
        let sound_type = sound_type.to_string();
        tokio::spawn(async move {
            if let Err(e) = Self::play_simple_sound(&sound_type).await {
                debug!("Failed to play sound: {}", e);
            }
        });
    }
    
    async fn play_simple_sound(sound_type: &str) -> Result<()> {
        let (freq, _duration_ms) = match sound_type {
            "start" => (800, 150),
            "stop" => (400, 200),  
            "complete" => (600, 100),
            _ => (500, 150),
        };
        
        // Try different sound files based on sound type
        let sound_files = match sound_type {
            "start" => vec![
                "/usr/share/sounds/alsa/Front_Left.wav",
                "/usr/share/sounds/freedesktop/stereo/bell.oga",
                "/usr/share/sounds/Oxygen-Sys-Log-In.ogg"
            ],
            "stop" => vec![
                "/usr/share/sounds/alsa/Front_Right.wav", 
                "/usr/share/sounds/alsa/Front_Left.wav",
                "/usr/share/sounds/freedesktop/stereo/bell.oga"
            ],
            _ => vec![
                "/usr/share/sounds/alsa/Front_Center.wav",
                "/usr/share/sounds/alsa/Front_Left.wav", 
                "/usr/share/sounds/freedesktop/stereo/bell.oga"
            ],
        };
        
        // Try aplay with system sounds first (we know this works)
        for sound_file in sound_files {
            if std::path::Path::new(sound_file).exists() {
                if let Ok(output) = Command::new("aplay")
                    .arg(sound_file)
                    .output()
                {
                    if output.status.success() {
                        debug!("Played {} with aplay: {}", sound_type, sound_file);
                        return Ok(());
                    }
                }
            }
        }
        
        // Try pactl as fallback (less reliable on this system)
        if let Ok(output) = Command::new("pactl")
            .args(&["play-sample", "bell-window-system"])
            .output()
        {
            if output.status.success() {
                debug!("Played {} with pactl", sound_type);
                return Ok(());
            }
        }
        
        // Final fallback - beep command if available
        if let Ok(output) = Command::new("beep")
            .args(&["-f", &freq.to_string(), "-l", "100"])
            .output()
        {
            if output.status.success() {
                debug!("Played {} with beep", sound_type);
                return Ok(());
            }
        }
        
        debug!("No working sound method found for {}", sound_type);
        Ok(())
    }
    
}
