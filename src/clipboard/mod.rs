use anyhow::Result;
use arboard::Clipboard;
use tracing::{debug, error, info};

pub struct ClipboardManager {
    clipboard: Clipboard,
    preserve_previous: bool,
}

impl ClipboardManager {
    pub fn new() -> Result<Self> {
        let clipboard = Clipboard::new()?;

        Ok(Self {
            clipboard,
            preserve_previous: false,
        })
    }

    pub fn with_preserve(mut self, preserve: bool) -> Self {
        self.preserve_previous = preserve;
        self
    }

    pub fn copy_text(&mut self, text: &str) -> Result<()> {
        if text.is_empty() {
            return Ok(());
        }

        let previous = if self.preserve_previous {
            self.clipboard.get_text().ok()
        } else {
            None
        };

        info!("Copying {} chars to clipboard", text.len());
        debug!("Text to copy: {}", text);

        self.clipboard.set_text(text)?;

        if let Some(prev) = previous {
            debug!("Previous clipboard content preserved: {} chars", prev.len());
        }

        Ok(())
    }

    pub async fn copy_with_wayland_fallback(&mut self, text: &str) -> Result<()> {
        // Try arboard first
        if let Err(e) = self.copy_text(text) {
            error!("Arboard clipboard failed: {}, trying wl-copy", e);

            // Fallback to wl-copy command
            use std::io::Write;
            use std::process::Command;

            let mut child = Command::new("wl-copy")
                .stdin(std::process::Stdio::piped())
                .spawn()?;

            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(text.as_bytes())?;
            }

            child.wait()?;
            info!("Copied text using wl-copy fallback");
        }

        Ok(())
    }
}
