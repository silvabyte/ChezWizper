use anyhow::Result;
use notify_rust::Notification;
use std::process::Command;
use tracing::{debug, info, warn};

pub struct Indicator {
    use_notifications: bool,
}

impl Indicator {
    pub fn new() -> Self {
        Self {
            use_notifications: true,
        }
    }
    
    pub async fn show_recording(&self) -> Result<()> {
        info!("Showing recording indicator");
        
        if self.use_notifications {
            Notification::new()
                .summary("ChezWizper")
                .body("🔴 Recording audio...")
                .icon("audio-input-microphone")
                .timeout(3000) // Persistent
                .show()?;
        }
        
        // Try to use Hyprland notification
        if let Err(e) = self.hyprland_notify("Recording", "🔴 Recording audio...") {
            debug!("Hyprland notification failed: {}", e);
        }
        
        Ok(())
    }
    
    pub async fn show_processing(&self) -> Result<()> {
        info!("Showing processing indicator");
        
        if self.use_notifications {
            Notification::new()
                .summary("ChezWizper")
                .body("⏳ Transcribing audio...")
                .icon("audio-x-generic")
                .timeout(5000)
                .show()?;
        }
        
        if let Err(e) = self.hyprland_notify("Processing", "⏳ Transcribing audio...") {
            debug!("Hyprland notification failed: {}", e);
        }
        
        Ok(())
    }
    
    pub async fn show_complete(&self, text: &str) -> Result<()> {
        info!("Showing completion indicator");
        
        let preview = if text.len() > 50 {
            format!("{}...", &text[..50])
        } else {
            text.to_string()
        };
        
        if self.use_notifications {
            Notification::new()
                .summary("ChezWizper")
                .body(&format!("✅ Transcribed: {}", preview))
                .icon("dialog-information")
                .timeout(3000)
                .show()?;
        }
        
        if let Err(e) = self.hyprland_notify("Complete", &format!("✅ {}", preview)) {
            debug!("Hyprland notification failed: {}", e);
        }
        
        Ok(())
    }
    
    pub async fn show_error(&self, error: &str) -> Result<()> {
        warn!("Showing error: {}", error);
        
        if self.use_notifications {
            Notification::new()
                .summary("ChezWizper Error")
                .body(error)
                .icon("dialog-error")
                .timeout(5000)
                .show()?;
        }
        
        if let Err(e) = self.hyprland_notify("Error", error) {
            debug!("Hyprland notification failed: {}", e);
        }
        
        Ok(())
    }
    
    fn hyprland_notify(&self, title: &str, _message: &str) -> Result<()> {
        Command::new("hyprctl")
            .args(["notify", "-1", "5000", "rgb(ff1744)", title])
            .output()?;
        
        Ok(())
    }
}
