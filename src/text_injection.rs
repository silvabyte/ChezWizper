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
    Clipboard,
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
        
        // Smart fallback logic - prioritize based on environment and availability
        
        // First, try ydotool (most reliable on Wayland when properly configured)
        if which("ydotool").is_ok() {
            info!("Using ydotool for text injection (auto-detected)");
            return Ok(Self { method: InjectionMethod::Ydotool });
        }
        
        // Check if we're on Wayland and prefer clipboard method
        if std::env::var("WAYLAND_DISPLAY").is_ok() {
            if which("wl-copy").is_ok() {
                info!("Using clipboard+paste for text injection (Wayland detected)");
                return Ok(Self { method: InjectionMethod::Clipboard });
            }
        }
        
        // Try wtype (limited compatibility but direct when it works)
        if which("wtype").is_ok() {
            info!("Using wtype for text injection (auto-detected, may fall back to clipboard)");
            return Ok(Self { method: InjectionMethod::Wtype });
        }
        
        // Final fallback to clipboard-only mode
        info!("Using clipboard-only for text injection (no direct input tools available)");
        Ok(Self { method: InjectionMethod::Clipboard })
    }
    
    pub async fn inject_text(&self, text: &str) -> Result<()> {
        if text.is_empty() {
            return Ok(());
        }
        
        info!("Injecting text: {} chars", text.len());
        debug!("Text to inject: {}", text);
        
        match self.method {
            InjectionMethod::Wtype => {
                // Try direct injection first, fall back to clipboard on failure
                if let Err(e) = self.inject_with_wtype(text) {
                    warn!("wtype direct injection failed: {}, falling back to clipboard paste", e);
                    self.inject_with_clipboard_paste(text).await
                } else {
                    Ok(())
                }
            }
            InjectionMethod::Ydotool => {
                // Try direct injection first, fall back to clipboard on failure  
                if let Err(e) = self.inject_with_ydotool(text) {
                    warn!("ydotool direct injection failed: {}, falling back to clipboard paste", e);
                    self.inject_with_clipboard_paste(text).await
                } else {
                    Ok(())
                }
            }
            InjectionMethod::Clipboard => self.inject_with_clipboard_paste(text).await,
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
            InjectionMethod::Clipboard => {
                // For clipboard method, paste is handled in inject_with_clipboard_paste
                return Ok(());
            }
        }
        
        Ok(())
    }
    
    async fn inject_with_clipboard_paste(&self, text: &str) -> Result<()> {
        info!("Using clipboard paste method for text injection");
        
        // Step 1: Copy text to clipboard
        self.copy_to_clipboard(text).await?;
        
        // Step 2: Small delay to ensure clipboard is updated
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // Step 3: Simulate paste shortcut
        self.simulate_paste().await
    }
    
    async fn copy_to_clipboard(&self, text: &str) -> Result<()> {
        use std::io::Write;
        
        // Try Wayland clipboard tools first
        if let Ok(mut child) = Command::new("wl-copy")
            .stdin(std::process::Stdio::piped())
            .spawn()
        {
            if let Some(stdin) = child.stdin.as_mut() {
                stdin.write_all(text.as_bytes())?;
            }
            let status = child.wait()?;
            if status.success() {
                debug!("Text copied to clipboard with wl-copy");
                return Ok(());
            }
        }
        
        // Fallback to X11 clipboard tools
        if let Ok(mut child) = Command::new("xclip")
            .args(["-selection", "clipboard"])
            .stdin(std::process::Stdio::piped())
            .spawn()
        {
            if let Some(stdin) = child.stdin.as_mut() {
                stdin.write_all(text.as_bytes())?;
            }
            let status = child.wait()?;
            if status.success() {
                debug!("Text copied to clipboard with xclip");
                return Ok(());
            }
        }
        
        // Try xsel as another X11 fallback
        if let Ok(mut child) = Command::new("xsel")
            .args(["--clipboard", "--input"])
            .stdin(std::process::Stdio::piped())
            .spawn()
        {
            if let Some(stdin) = child.stdin.as_mut() {
                stdin.write_all(text.as_bytes())?;
            }
            let status = child.wait()?;
            if status.success() {
                debug!("Text copied to clipboard with xsel");
                return Ok(());
            }
        }
        
        Err(anyhow::anyhow!("No clipboard tool available (tried wl-copy, xclip, xsel)"))
    }
    
    async fn simulate_paste(&self) -> Result<()> {
        info!("Simulating Ctrl+V paste");
        
        // Try different paste methods based on available tools and detected environment
        
        // Method 1: ydotool (if available and properly configured)
        if which("ydotool").is_ok() {
            if let Ok(output) = Command::new("ydotool")
                .args(["key", "29:1", "47:1", "47:0", "29:0"])  // Ctrl+V key codes
                .output()
            {
                if output.status.success() {
                    debug!("Successfully pasted with ydotool");
                    return Ok(());
                }
            }
        }
        
        // Method 2: wtype (if available)
        if which("wtype").is_ok() {
            if let Ok(output) = Command::new("wtype")
                .args(["-M", "ctrl", "-P", "v", "-m", "ctrl", "-p", "v"])
                .output()
            {
                if output.status.success() {
                    debug!("Successfully pasted with wtype");
                    return Ok(());
                } else {
                    debug!("wtype paste failed, continuing with other methods");
                }
            }
        }
        
        // Method 3: xdotool (X11 fallback)
        if which("xdotool").is_ok() {
            if let Ok(output) = Command::new("xdotool")
                .args(["key", "ctrl+v"])
                .output()
            {
                if output.status.success() {
                    debug!("Successfully pasted with xdotool");
                    return Ok(());
                }
            }
        }
        
        // Method 4: Desktop environment specific methods
        if let Ok(desktop) = std::env::var("XDG_CURRENT_DESKTOP") {
            match desktop.as_str() {
                "KDE" => {
                    if let Ok(output) = Command::new("qdbus")
                        .args([
                            "org.kde.klipper",
                            "/klipper", 
                            "org.kde.klipper.klipper.invokeAction",
                            "paste"
                        ])
                        .output()
                    {
                        if output.status.success() {
                            debug!("Successfully pasted with KDE klipper");
                            return Ok(());
                        }
                    }
                }
                "GNOME" | "ubuntu:GNOME" => {
                    // GNOME doesn't have easy programmatic paste, rely on key simulation
                    debug!("GNOME detected, key simulation methods already tried");
                }
                _ => {
                    debug!("Unknown desktop environment: {}", desktop);
                }
            }
        }
        
        // If all methods fail, inform user but don't error out
        warn!("All paste methods failed - text copied to clipboard, manual paste required");
        info!("Text is available in clipboard. You can paste manually with Ctrl+V");
        Ok(())
    }
}
