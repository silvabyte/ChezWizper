use anyhow::{Context, Result};
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

#[derive(Debug)]
struct ClipboardBackend {
    name: &'static str,
    copy_cmd: &'static str,
    copy_args: &'static [&'static str],
    read_cmd: &'static str,
    read_args: &'static [&'static str],
    use_stdin: bool,
}

const CLIPBOARD_BACKENDS: &[ClipboardBackend] = &[
    ClipboardBackend {
        name: "wl-copy",
        copy_cmd: "wl-copy",
        copy_args: &[],
        read_cmd: "wl-paste",
        read_args: &["--no-newline"],
        use_stdin: true,
    },
    ClipboardBackend {
        name: "xclip",
        copy_cmd: "xclip",
        copy_args: &["-selection", "clipboard"],
        read_cmd: "xclip",
        read_args: &["-selection", "clipboard", "-out"],
        use_stdin: true,
    },
    ClipboardBackend {
        name: "xsel",
        copy_cmd: "xsel",
        copy_args: &["--clipboard", "--input"],
        read_cmd: "xsel",
        read_args: &["--clipboard", "--output"],
        use_stdin: true,
    },
];

impl TextInjector {
    pub fn new(preferred: Option<&str>) -> Result<Self> {
        match preferred {
            Some("ydotool") => {
                if which("ydotool").is_ok() {
                    info!("Using ydotool for text injection (per config)");
                    return Ok(Self {
                        method: InjectionMethod::Ydotool,
                    });
                } else {
                    warn!("ydotool requested in config but not found, falling back...");
                }
            }
            Some("wtype") => {
                if which("wtype").is_ok() {
                    info!("Using wtype for text injection (per config)");
                    return Ok(Self {
                        method: InjectionMethod::Wtype,
                    });
                } else {
                    warn!("wtype requested in config but not found, falling back...");
                }
            }
            Some(other) => {
                warn!(
                    "Unknown input_method '{}' in config, falling back to auto-detect",
                    other
                );
            }
            None => {}
        }

        // Smart fallback logic - prioritize based on environment and availability

        // First, try ydotool (most reliable on Wayland when properly configured)
        if which("ydotool").is_ok() {
            info!("Using ydotool for text injection (auto-detected)");
            return Ok(Self {
                method: InjectionMethod::Ydotool,
            });
        }

        // Check if we're on Wayland and prefer clipboard method
        if std::env::var("WAYLAND_DISPLAY").is_ok() && which("wl-copy").is_ok() {
            info!("Using clipboard+paste for text injection (Wayland detected)");
            return Ok(Self {
                method: InjectionMethod::Clipboard,
            });
        }

        // Try wtype (limited compatibility but direct when it works)
        if which("wtype").is_ok() {
            info!("Using wtype for text injection (auto-detected, may fall back to clipboard)");
            return Ok(Self {
                method: InjectionMethod::Wtype,
            });
        }

        // Final fallback to clipboard-only mode
        info!("Using clipboard-only for text injection (no direct input tools available)");
        Ok(Self {
            method: InjectionMethod::Clipboard,
        })
    }

    pub async fn inject_text(&self, text: &str) -> Result<()> {
        if text.is_empty() {
            return Ok(());
        }

        info!("Injecting text: {} chars", text.len());
        debug!("Text to inject: {}", text);

        match self.method {
            InjectionMethod::Wtype => {
                self.try_inject_with_fallback(text, |t| self.inject_with_wtype(t), "wtype")
                    .await
            }
            InjectionMethod::Ydotool => {
                self.try_inject_with_fallback(text, |t| self.inject_with_ydotool(t), "ydotool")
                    .await
            }
            InjectionMethod::Clipboard => self.inject_with_clipboard_paste(text).await,
        }
    }

    async fn try_inject_with_fallback<F>(
        &self,
        text: &str,
        inject_fn: F,
        method_name: &str,
    ) -> Result<()>
    where
        F: FnOnce(&str) -> Result<()>,
    {
        if let Err(e) = inject_fn(text) {
            warn!(
                "{} direct injection failed: {}, falling back to clipboard paste",
                method_name, e
            );
            self.inject_with_clipboard_paste(text).await
        } else {
            Ok(())
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
            return Err(anyhow::anyhow!(
                "ydotool failed: {}. Make sure ydotoold is running",
                stderr
            ));
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

        // Copy text to clipboard with verification and retry
        self.copy_to_clipboard_with_verify(text).await?;

        // Simulate paste shortcut
        self.simulate_paste().await
    }

    async fn copy_to_clipboard_with_verify(&self, text: &str) -> Result<()> {
        let mut delay_ms = 50;
        let max_total_ms = 1000;
        let mut total_ms = 0;

        loop {
            // Try to copy
            self.copy_to_clipboard(text).await?;

            // Small initial delay to let clipboard settle
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

            // Verify it worked
            if let Ok(clipboard_content) = self.read_clipboard().await {
                if clipboard_content.trim() == text.trim() {
                    debug!("Clipboard verified after {}ms", total_ms);
                    return Ok(());
                }
            }

            // Check timeout
            if total_ms >= max_total_ms {
                warn!(
                    "Clipboard verification failed after {}ms, proceeding anyway",
                    total_ms
                );
                return Ok(());
            }

            // Exponential backoff
            tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
            total_ms += delay_ms;
            delay_ms = (delay_ms * 2).min(200); // Cap individual delay at 200ms
        }
    }

    async fn read_clipboard(&self) -> Result<String> {
        for backend in CLIPBOARD_BACKENDS {
            if which(backend.read_cmd).is_err() {
                continue;
            }

            if let Ok(output) = Command::new(backend.read_cmd)
                .args(backend.read_args)
                .output()
            {
                if output.status.success() {
                    return Ok(String::from_utf8_lossy(&output.stdout).to_string());
                }
            }
        }

        Err(anyhow::anyhow!(
            "Failed to read clipboard - no working backend found"
        ))
    }

    async fn copy_to_clipboard(&self, text: &str) -> Result<()> {
        use std::io::Write;

        for backend in CLIPBOARD_BACKENDS {
            if which(backend.copy_cmd).is_err() {
                continue;
            }

            let mut cmd = Command::new(backend.copy_cmd);
            cmd.args(backend.copy_args);

            if backend.use_stdin {
                cmd.stdin(std::process::Stdio::piped());
            }

            if let Ok(mut child) = cmd.spawn() {
                if backend.use_stdin {
                    if let Some(stdin) = child.stdin.as_mut() {
                        if stdin.write_all(text.as_bytes()).is_err() {
                            continue;
                        }
                    }
                }

                if let Ok(status) = child.wait() {
                    if status.success() {
                        debug!("Text copied to clipboard with {}", backend.name);
                        return Ok(());
                    }
                }
            }
        }

        Err(anyhow::anyhow!("No clipboard tool available"))
    }

    async fn simulate_paste(&self) -> Result<()> {
        info!("Simulating Ctrl+V paste");

        // Try different paste methods based on available tools and detected environment

        // Method 1: ydotool (if available and properly configured)
        if which("ydotool").is_ok() {
            if let Ok(output) = Command::new("ydotool")
                .args(["key", "29:1", "47:1", "47:0", "29:0"]) // Ctrl+V key codes
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
            if let Ok(output) = Command::new("xdotool").args(["key", "ctrl+v"]).output() {
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
                            "paste",
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
