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
    KdeConnect,
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
        // Fallback logic - try in order of reliability for different environments
        if which("ydotool").is_ok() {
            info!("Using ydotool for text injection (auto-detected)");
            return Ok(Self { method: InjectionMethod::Ydotool });
        }
        
        // Check if we're in KDE and have qdbus for KDE Connect
        if std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default() == "KDE" && which("qdbus").is_ok() {
            info!("Using clipboard+paste for text injection (KDE detected)");
            return Ok(Self { method: InjectionMethod::Clipboard });
        }
        
        if which("wtype").is_ok() {
            info!("Using wtype for text injection (auto-detected, may fall back to clipboard)");
            return Ok(Self { method: InjectionMethod::Wtype });
        }
        
        // Ultimate fallback - just use clipboard
        info!("Using clipboard-only for text injection (fallback)");
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
                // On KDE Wayland, wtype typing often fails but paste works
                // So let's always use clipboard paste instead of typing
                info!("Using clipboard paste method for KDE Wayland");
                self.inject_with_clipboard_paste(text).await
            }
            InjectionMethod::Ydotool => {
                if let Err(e) = self.inject_with_ydotool(text) {
                    warn!("ydotool typing failed: {}, falling back to clipboard paste", e);
                    self.inject_with_clipboard_paste(text).await
                } else {
                    info!("Successfully injected text with ydotool");
                    Ok(())
                }
            }
            InjectionMethod::KdeConnect => self.inject_with_kde_connect(text).await,
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
            InjectionMethod::KdeConnect => {
                self.simulate_paste_kde().await?;
            }
            InjectionMethod::Clipboard => {
                self.simulate_paste_kde().await?;
            }
        }
        
        Ok(())
    }
    
    async fn inject_with_kde_connect(&self, text: &str) -> Result<()> {
        // For KDE, we'll use clipboard + paste simulation
        self.inject_with_clipboard_paste(text).await
    }
    
    async fn inject_with_clipboard_paste(&self, text: &str) -> Result<()> {
        use std::io::Write;
        
        info!("Using wl-copy + KDE scripting method for KDE Wayland");
        
        // Step 1: Copy text to clipboard using wl-copy
        let mut child = Command::new("wl-copy")
            .arg("--trim-newline")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .context("Failed to spawn wl-copy")?;
            
        if let Some(stdin) = child.stdin.as_mut() {
            stdin.write_all(text.as_bytes())
                .context("Failed to write to wl-copy stdin")?;
        }
        
        let status = child.wait().context("Failed to wait for wl-copy")?;
        if !status.success() {
            return Err(anyhow::anyhow!("wl-copy failed"));
        }
        
        info!("Text copied to clipboard successfully");
        
        // Step 2: Small delay to ensure clipboard is updated
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // Step 3: Use ydotool with proper systemd service
        self.ydotool_paste().await
    }
    
    async fn ydotool_paste(&self) -> Result<()> {
        info!("Using ydotool with systemd service for paste");
        
        // Use the proper socket path for systemd service
        let socket_path = "/run/user/1000/.ydotool_socket";
        
        // Send Ctrl+V using ydotool (key codes: 29=Ctrl, 47=V)
        let output = Command::new("ydotool")
            .args(["key", "29:1", "47:1", "47:0", "29:0"])
            .env("YDOTOOL_SOCKET", socket_path)
            .output()
            .context("Failed to execute ydotool")?;
            
        if output.status.success() {
            info!("Successfully pasted with ydotool Ctrl+V");
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("ydotool paste failed: {}", stderr))
        }
    }
    
    async fn kde_script_paste(&self) -> Result<()> {
        info!("Attempting KDE script-based paste");
        
        // Create a temporary KWin script to send Ctrl+V
        let script_content = r#"
        workspace.sendKeyEvent("ctrl+v");
        "#;
        
        // Try to execute via KWin scripting
        let temp_script = "/tmp/chezwizper_paste.js";
        std::fs::write(temp_script, script_content)?;
        
        // Load and run the script
        let output = Command::new("qdbus")
            .args([
                "org.kde.KWin",
                "/Scripting",
                "org.kde.kwin.Scripting.loadScript",
                temp_script,
                "chezwizper-paste"
            ])
            .output()
            .context("Failed to load KWin script")?;
            
        if output.status.success() {
            // Run the script
            let script_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
            
            let run_output = Command::new("qdbus")
                .args([
                    "org.kde.KWin",
                    &format!("/{}", script_id),
                    "org.kde.kwin.Script.run"
                ])
                .output()
                .context("Failed to run KWin script")?;
                
            // Clean up
            let _ = std::fs::remove_file(temp_script);
            
            if run_output.status.success() {
                info!("Successfully pasted using KDE scripting");
                return Ok(());
            }
        }
        
        // Fallback: Try simple qdbus paste commands
        info!("KWin script failed, trying qdbus shortcuts");
        
        // Try different KDE paste methods
        let methods = [
            vec!["org.kde.kglobalaccel", "/kglobalaccel", "org.kde.kglobalaccel.invokeShortcut", "kwin", "Paste"],
            vec!["org.kde.klipper", "/klipper", "org.kde.klipper.klipper.invokeAction", "paste"],
        ];
        
        for method in &methods {
            let output = Command::new("qdbus")
                .args(method)
                .output();
            
            if let Ok(output) = output {
                if output.status.success() {
                    info!("Successfully pasted using qdbus method");
                    return Ok(());
                }
            }
        }
        
        warn!("All KDE paste methods failed - text is in clipboard, manual paste required");
        Ok(())
    }
    
    async fn ensure_ydotool_daemon_running(&self) -> Result<()> {
        // Check if ydotoold is already running
        let output = Command::new("pgrep")
            .args(["-x", "ydotoold"])
            .output()
            .context("Failed to check for ydotoold process")?;
            
        if output.status.success() {
            debug!("ydotoold daemon is already running");
            return Ok(());
        }
        
        info!("Starting ydotoold daemon with proper permissions");
        
        // Try to start ydotoold daemon with sudo if needed
        let mut cmd = Command::new("sudo");
        cmd.args(["ydotoold", "--socket-path", "/tmp/.ydotool_socket", "--socket-perm", "0666"]);
        
        // Start as background process
        let child = cmd.spawn().context("Failed to start ydotoold daemon with sudo")?;
        
        // Give the daemon more time to create the virtual device and set up permissions
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        
        // Verify the socket was created
        if !std::path::Path::new("/tmp/.ydotool_socket").exists() {
            return Err(anyhow::anyhow!("ydotool socket was not created"));
        }
        
        debug!("ydotoold daemon started successfully");
        Ok(())
    }
    
    async fn simulate_paste_kde(&self) -> Result<()> {
        info!("Trying KDE-specific paste methods");
        
        // Method 1: Try to trigger paste via KDE's kglobalaccel
        if let Ok(output) = Command::new("qdbus")
            .args([
                "org.kde.kglobalaccel",
                "/component/kwin",
                "org.kde.kglobalaccel.Component.invokeShortcut",
                "Edit"
            ])
            .output()
        {
            if output.status.success() {
                debug!("Used KDE global accelerator for paste");
                return Ok(());
            }
        }
        
        // Method 2: Try sending keypress via kwin
        if let Ok(output) = Command::new("qdbus")
            .args([
                "org.kde.KWin",
                "/Scripting",
                "org.kde.kwin.Scripting.loadScript",
                "sendkey.js"
            ])
            .output()
        {
            debug!("Attempted KWin script approach");
        }
        
        // Method 3: Use KDE's input method if available
        if let Ok(output) = Command::new("qdbus")
            .args([
                "org.kde.keyboard",
                "/Layouts",
                "org.kde.KeyboardLayouts.setLayout",
                "0"
            ])
            .output()
        {
            debug!("Attempted keyboard layout approach");
        }
        
        warn!("All KDE paste methods failed - text is in clipboard but couldn't paste automatically");
        info!("Text copied to clipboard - you can paste manually with Ctrl+V");
        Ok(())
    }
}
