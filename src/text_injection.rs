use anyhow::{Result, Context};
use std::process::Command;
use tracing::{debug, info, warn};
use which::which;

pub struct TextInjector {
    method: InjectionMethod,
}

#[derive(Debug, Clone)]
enum InjectionMethod {
    Wtype,
    Ydotool,
}

impl TextInjector {
    pub fn new(preferred: Option<&str>) -> Result<Self> {
        match preferred {
            Some("ydotool") => {
                if which("ydotool").is_ok() {
                    info!("Using ydotool for text injection (per config)");
                    return Ok(Self { method: InjectionMethod::Ydotool });
                } else {
                    warn!("ydotool requested in config but not found, falling back...");
                }
            }
            Some("wtype") => {
                if which("wtype").is_ok() {
                    info!("Using wtype for text injection (per config)");
                    return Ok(Self { method: InjectionMethod::Wtype });
                } else {
                    warn!("wtype requested in config but not found, falling back...");
                }
            }
            Some(other) => {
                warn!("Unknown input_method '{}' in config, falling back to auto-detect", other);
            }
            None => {}
        }
        // Fallback logic
        if which("wtype").is_ok() {
            info!("Using wtype for text injection (auto-detected)");
            return Ok(Self { method: InjectionMethod::Wtype });
        }
        if which("ydotool").is_ok() {
            info!("Using ydotool for text injection (auto-detected)");
            return Ok(Self { method: InjectionMethod::Ydotool });
        }
        Err(anyhow::anyhow!("No text injection tool found. Please install wtype or ydotool"))
    }
    
    pub async fn inject_text(&self, text: &str) -> Result<()> {
        if text.is_empty() {
            return Ok(());
        }
        
        info!("Injecting text: {} chars", text.len());
        debug!("Text to inject: {}", text);
        
        match self.method {
            InjectionMethod::Wtype => self.inject_with_wtype(text),
            InjectionMethod::Ydotool => self.inject_with_ydotool(text),
        }
    }
    
    fn inject_with_wtype(&self, text: &str) -> Result<()> {
        let output = Command::new("wtype")
            .arg(text)
            .output()
            .context("Failed to execute wtype")?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("wtype failed: {}", stderr));
        }
        
        Ok(())
    }
    
    fn inject_with_ydotool(&self, text: &str) -> Result<()> {
        // ydotool requires the daemon to be running
        let output = Command::new("ydotool")
            .arg("type")
            .arg(text)
            .output()
            .context("Failed to execute ydotool")?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            warn!("ydotool failed: {}", stderr);
            return Err(anyhow::anyhow!("ydotool failed: {}. Make sure ydotoold is running", stderr));
        }
        
        Ok(())
    }
    
    pub async fn paste_from_clipboard(&self) -> Result<()> {
        info!("Simulating paste shortcut");
        
        match self.method {
            InjectionMethod::Wtype => {
                Command::new("wtype")
                    .args(["-M", "ctrl", "-P", "v", "-m", "ctrl", "-p", "v"])
                    .output()
                    .context("Failed to simulate paste with wtype")?;
            }
            InjectionMethod::Ydotool => {
                Command::new("ydotool")
                    .args(["key", "ctrl+v"])
                    .output()
                    .context("Failed to simulate paste with ydotool")?;
            }
        }
        
        Ok(())
    }
}
