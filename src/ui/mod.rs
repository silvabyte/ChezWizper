use anyhow::Result;
use std::process::Command;
use tracing::{debug, info, warn};

pub struct Indicator {
}

impl Indicator {
    pub fn new() -> Self {
        Self {
        }
    }
    
    pub async fn show_recording(&self) -> Result<()> {
        info!("Showing recording indicator");
        
        if let Err(e) = self.hyprland_notify("🔴 Recording...") {
            debug!("Hyprland notification failed: {}", e);
        }
        
        Ok(())
    }
    
    pub async fn show_processing(&self) -> Result<()> {
        info!("Showing processing indicator");
        
        if let Err(e) = self.hyprland_notify("⏳Normalizing...") {
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
        
        if let Err(e) = self.hyprland_notify(&format!("✅ {}", preview)) {
            debug!("Hyprland notification failed: {}", e);
        }
        
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
}
