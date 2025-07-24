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
        
        // Try paplay first (most reliable)
        if let Ok(_) = Command::new("pactl")
            .args(&["play-sample", "bell-window-system"])
            .output()
        {
            return Ok(());
        }
        
        // Try aplay with system sound
        if let Ok(_) = Command::new("aplay")
            .args(&["/usr/share/sounds/alsa/Front_Left.wav"])
            .output()
        {
            return Ok(());
        }
        
        // Fallback to speaker-test
        Command::new("timeout")
            .args(&["0.2", "speaker-test", "-t", "sine", "-f", &freq.to_string(), "-c", "1"])
            .output()?;
            
        Ok(())
    }
    
}
