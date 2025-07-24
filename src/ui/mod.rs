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
        
        // Try different sound systems in order of preference
        if let Err(_) = self.play_with_paplay(sound_type).await {
            if let Err(_) = self.play_with_aplay(sound_type).await {
                if let Err(_) = self.play_with_beep(sound_type).await {
                    debug!("No sound system available for audio feedback");
                }
            }
        }
    }
    
    async fn play_with_paplay(&self, sound_type: &str) -> Result<()> {
        let frequency = match sound_type {
            "start" => "800",    // Higher pitch for start
            "stop" => "400",     // Lower pitch for stop  
            "complete" => "600", // Medium pitch for completion
            _ => "500",
        };
        
        // Generate a simple tone using PulseAudio
        let result = Command::new("paplay")
            .args(&["--raw", "--format=s16le", "--rate=44100", "--channels=1"])
            .stdin(std::process::Stdio::piped())
            .spawn();
            
        if let Ok(mut child) = result {
            // Generate a simple beep tone (very basic)
            let samples = 4410; // 0.1 seconds at 44100 Hz
            let freq: f32 = frequency.parse().unwrap_or(500.0);
            let mut audio_data = Vec::new();
            
            for i in 0..samples {
                let t = i as f32 / 44100.0;
                let sample = (2.0 * std::f32::consts::PI * freq * t).sin();
                let sample_i16 = (sample * 16384.0) as i16;
                audio_data.extend_from_slice(&sample_i16.to_le_bytes());
            }
            
            if let Some(stdin) = child.stdin.as_mut() {
                use std::io::Write;
                let _ = stdin.write_all(&audio_data);
            }
            
            let _ = child.wait();
            debug!("Played {} sound with paplay", sound_type);
            return Ok(());
        }
        
        Err(anyhow::anyhow!("paplay not available"))
    }
    
    async fn play_with_aplay(&self, sound_type: &str) -> Result<()> {
        // Fallback to system beep sounds if available
        let sound_file = match sound_type {
            "start" => "/usr/share/sounds/alsa/Front_Left.wav",
            "stop" => "/usr/share/sounds/alsa/Front_Right.wav", 
            "complete" => "/usr/share/sounds/alsa/Front_Center.wav",
            _ => "/usr/share/sounds/alsa/Front_Left.wav",
        };
        
        if std::path::Path::new(sound_file).exists() {
            Command::new("aplay")
                .arg(sound_file)
                .output()?;
            debug!("Played {} sound with aplay", sound_type);
            return Ok(());
        }
        
        Err(anyhow::anyhow!("aplay not available or no sound files"))
    }
    
    async fn play_with_beep(&self, sound_type: &str) -> Result<()> {
        let (frequency, duration) = match sound_type {
            "start" => (800, 150),    // High pitch, short
            "stop" => (400, 200),     // Low pitch, slightly longer
            "complete" => (600, 100), // Medium pitch, very short
            _ => (500, 150),
        };
        
        // Use the beep command if available
        Command::new("beep")
            .args(&["-f", &frequency.to_string(), "-l", &duration.to_string()])
            .output()?;
            
        debug!("Played {} sound with beep", sound_type);
        Ok(())
    }
}
