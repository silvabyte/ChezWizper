# Adding New Transcription Providers

This guide shows developers how to add new third-party transcription providers to ChezWizper. The provider architecture is designed to make this process straightforward and consistent.

## Overview

ChezWizper uses a trait-based provider system where each transcription service implements the `TranscriptionProvider` trait. This allows for clean abstraction and easy extensibility.

**Current providers:**
- **OpenAI API** - Cloud-based OpenAI Whisper API
- **OpenAI CLI** - Local OpenAI Whisper CLI tool  
- **whisper.cpp** - Local whisper.cpp implementation

**Potential new providers:**
- Azure OpenAI Service
- Google Speech-to-Text
- Amazon Transcribe
- Anthropic Claude (when audio support arrives)
- AssemblyAI
- Rev.ai
- Custom API endpoints

## Provider Trait Interface

All providers must implement the `TranscriptionProvider` trait:

```rust
use anyhow::Result;
use std::future::Future;
use std::path::Path;
use std::pin::Pin;

pub trait TranscriptionProvider: Send + Sync {
    /// Human-readable name for this provider
    fn name(&self) -> &'static str;
    
    /// Check if this provider is available/configured properly
    fn is_available(&self) -> bool;
    
    /// Transcribe an audio file
    fn transcribe<'a>(
        &'a self,
        audio_path: &'a Path,
        language: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<String>> + Send + 'a>>;
}
```

### Method Details

**`name()`** - Returns a human-readable name for logging and user feedback
- Examples: `"Google Speech-to-Text"`, `"Azure OpenAI"`, `"AssemblyAI"`

**`is_available()`** - Quick check if the provider can be used
- For API providers: Check if API key/credentials are configured
- For CLI providers: Check if binary exists and is accessible
- Should be fast (no network calls)

**`transcribe()`** - Core transcription functionality
- Takes audio file path and language code
- Returns async result with transcribed text
- Should handle errors gracefully with context

## Step-by-Step Implementation

Let's walk through adding a fictional "SuperSpeech API" provider:

### 1. Create the Provider Module

Create `src/whisper/providers/superspeech_api.rs`:

```rust
use anyhow::{Context, Result};
use reqwest::multipart::{Form, Part};
use serde::Deserialize;
use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use tracing::{debug, error, info};

use crate::whisper::provider::TranscriptionProvider;

#[derive(Debug, Deserialize)]
struct SuperSpeechResponse {
    transcription: String,
    confidence: f32,
}

#[derive(Debug, Deserialize)]
struct SuperSpeechError {
    error: String,
    code: i32,
}

pub struct SuperSpeechProvider {
    client: reqwest::Client,
    api_key: String,
    endpoint: String,
    model: String,
}

impl SuperSpeechProvider {
    pub fn new(api_key: String, endpoint: Option<String>, model: String) -> Result<Self> {
        let client = reqwest::Client::new();
        let endpoint = endpoint.unwrap_or_else(|| {
            "https://api.superspeech.ai/v1/transcribe".to_string()
        });

        info!("Initialized SuperSpeech provider with endpoint: {}", endpoint);

        Ok(Self {
            client,
            api_key,
            endpoint,
            model,
        })
    }
}

impl TranscriptionProvider for SuperSpeechProvider {
    fn name(&self) -> &'static str {
        "SuperSpeech API"
    }

    fn is_available(&self) -> bool {
        !self.api_key.is_empty()
    }

    fn transcribe<'a>(
        &'a self,
        audio_path: &'a Path,
        language: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<String>> + Send + 'a>> {
        Box::pin(async move {
            info!("Transcribing audio file via SuperSpeech API: {:?}", audio_path);

            // Read audio file
            let audio_data = tokio::fs::read(audio_path)
                .await
                .context("Failed to read audio file")?;

            let filename = audio_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("audio.wav");

            // Create multipart form
            let audio_part = Part::bytes(audio_data)
                .file_name(filename.to_string())
                .mime_str("audio/wav")
                .context("Failed to set MIME type")?;

            let mut form = Form::new()
                .part("audio", audio_part)
                .text("model", self.model.clone());

            // Add language if not auto
            if !language.is_empty() && language != "auto" {
                form = form.text("language", language.to_string());
            }

            debug!(
                "Sending request to SuperSpeech API with model: {}, language: {}",
                self.model, language
            );

            // Send request
            let response = self
                .client
                .post(&self.endpoint)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("User-Agent", "ChezWizper/1.0")
                .multipart(form)
                .send()
                .await
                .context("Failed to send request to SuperSpeech API")?;

            let status = response.status();
            let response_text = response
                .text()
                .await
                .context("Failed to read response body")?;

            if !status.is_success() {
                error!(
                    "SuperSpeech API request failed with status {}: {}",
                    status, response_text
                );

                // Try to parse error response
                if let Ok(error_response) = serde_json::from_str::<SuperSpeechError>(&response_text) {
                    return Err(anyhow::anyhow!(
                        "SuperSpeech API error: {} (code: {})",
                        error_response.error,
                        error_response.code
                    ));
                }

                return Err(anyhow::anyhow!(
                    "SuperSpeech API request failed with status {}: {}",
                    status,
                    response_text
                ));
            }

            // Parse successful response
            let transcription: SuperSpeechResponse = serde_json::from_str(&response_text)
                .context("Failed to parse SuperSpeech response")?;

            let text = transcription.transcription.trim().to_string();
            info!(
                "SuperSpeech transcription complete: {} chars (confidence: {:.2})",
                text.len(),
                transcription.confidence
            );
            debug!("Raw transcription: {}", text);

            Ok(text)
        })
    }
}
```

### 2. Update the Providers Module

Add your provider to `src/whisper/providers/mod.rs`:

```rust
pub mod openai_api;
pub mod openai_cli;
pub mod whisper_cpp;
pub mod superspeech_api;  // Add this line

pub use openai_api::OpenAIProvider;
pub use openai_cli::OpenAIWhisperCliProvider;
pub use whisper_cpp::WhisperCppProvider;
pub use superspeech_api::SuperSpeechProvider;  // Add this line
```

### 3. Register the Provider

Update `src/whisper/mod.rs` to include your provider:

```rust
// Add to imports
use providers::{OpenAIProvider, OpenAIWhisperCliProvider, WhisperCppProvider, SuperSpeechProvider};

// Add to the match statement in with_provider()
pub fn with_provider(provider_name: &str, config: ProviderConfig) -> Result<Self> {
    let language = config.language.clone().unwrap_or_else(|| "en".to_string());

    let provider: Box<dyn TranscriptionProvider> = match provider_name {
        "openai-api" => {
            // existing OpenAI code...
        }
        "openai-cli" => {
            // existing OpenAI CLI code...
        }
        "whisper-cpp" => {
            // existing whisper.cpp code...
        }
        "superspeech-api" => {  // Add this case
            let api_key = config.api_key
                .context("api_key is required for SuperSpeech API provider")?;

            let model = config.model.unwrap_or_else(|| "base".to_string());
            Box::new(SuperSpeechProvider::new(
                api_key,
                config.api_endpoint,
                model,
            )?)
        }
        _ => {
            // existing auto-detection code...
        }
    };

    // rest of function...
}
```

### 4. Update Configuration Documentation

Add your provider to `docs/configuration.md`:

```markdown
**SuperSpeech API** (`provider = "superspeech-api"`)
- **Best for:** High accuracy speech recognition with confidence scores
- **Requirements:** SuperSpeech API key in config, internet connection  
- **Models:** `"base"`, `"premium"`, `"realtime"`
- **Cost:** ~$0.004 per minute of audio
```

And add a configuration example:

```markdown
### For SuperSpeech API Users
```toml
[whisper]
provider = "superspeech-api"
api_key = "ss-your-api-key-here"
model = "premium"
language = "en"  # or "auto" for automatic detection
```

### 5. Add to Example Config

Update `example_config.toml` with a commented example:

```toml
# provider = "superspeech-api"  # SuperSpeech API provider
# api_key = "ss-your-key"      # Your SuperSpeech API key
# model = "premium"            # SuperSpeech model
```

### 6. Optional: Add to Auto-Detection

If your provider should be part of auto-detection (for non-API providers), add it to the `auto_detect_provider()` function:

```rust
fn auto_detect_provider(custom_path: Option<String>) -> Result<Box<dyn TranscriptionProvider>> {
    info!("Auto-detecting transcription provider...");

    // Note: API providers require explicit configuration
    
    if let Ok(provider) = OpenAIWhisperCliProvider::new(custom_path.clone(), "base".to_string()) {
        if provider.is_available() {
            info!("Auto-detected: OpenAI Whisper CLI");
            return Ok(Box::new(provider));
        }
    }

    if let Ok(provider) = WhisperCppProvider::new(custom_path, "base".to_string(), None) {
        if provider.is_available() {
            info!("Auto-detected: whisper.cpp");
            return Ok(Box::new(provider));
        }
    }

    // Add here for CLI-based providers that should be auto-detected
    
    Err(anyhow::anyhow!(
        "No transcription provider available. Install whisper-cpp, openai-whisper, or configure API provider"
    ))
}
```

## Testing Your Provider

### 1. Unit Tests

Create tests in `src/whisper/providers/superspeech_api.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_creation() {
        let provider = SuperSpeechProvider::new(
            "test-key".to_string(),
            None,
            "base".to_string(),
        ).unwrap();

        assert_eq!(provider.name(), "SuperSpeech API");
        assert!(provider.is_available());
    }

    #[test]
    fn test_provider_unavailable_without_key() {
        let provider = SuperSpeechProvider::new(
            "".to_string(),
            None,
            "base".to_string(),
        ).unwrap();

        assert!(!provider.is_available());
    }
}
```

### 2. Integration Testing

Test with a real API key:

```rust
// In src/bin/ create test_superspeech.rs
use anyhow::Result;
use chezwizper::whisper::{WhisperTranscriber, ProviderConfig};

#[tokio::main]
async fn main() -> Result<()> {
    let config = ProviderConfig {
        api_key: Some("your-test-key".to_string()),
        model: Some("base".to_string()),
        language: Some("en".to_string()),
        ..Default::default()
    };

    let transcriber = WhisperTranscriber::with_provider("superspeech-api", config)?;
    println!("✓ SuperSpeech provider initialized successfully");

    Ok(())
}
```

### 3. Manual Testing

Test with the main binary:

```toml
# In your config.toml
[whisper]
provider = "superspeech-api"
api_key = "your-real-api-key"
model = "base"
language = "en"
```

```bash
# Run ChezWizper and test recording
cargo run
```

## Best Practices

### 1. Error Handling

- **Use `anyhow::Context`** for descriptive error messages
- **Log errors** with appropriate levels (error, warn, info, debug)
- **Parse API errors** when possible to provide helpful feedback
- **Handle network issues** gracefully (timeouts, connectivity)

### 2. Configuration

- **Validate inputs** in the constructor (API keys, endpoints, models)
- **Provide sensible defaults** where possible
- **Document all configuration options** thoroughly
- **Support optional parameters** (custom endpoints, timeouts)

### 3. Logging

- **Info level**: Provider initialization, transcription start/complete
- **Debug level**: Request details, raw responses
- **Error level**: API failures, configuration problems
- **Warn level**: Fallbacks, deprecated options

### 4. Security

- **Never log API keys** or sensitive data
- **Use HTTPS** for all API communications
- **Validate SSL certificates** (enabled by default in reqwest)
- **Set appropriate timeouts** to prevent hanging requests

### 5. Performance

- **Reuse HTTP clients** (create once in constructor)
- **Handle large files** efficiently (streaming where possible)
- **Implement retries** for transient failures
- **Add timeouts** for network operations

## Common Patterns

### API-Based Providers

Most cloud providers follow similar patterns:

```rust
// HTTP client with authentication
pub struct CloudProvider {
    client: reqwest::Client,
    api_key: String,
    endpoint: String,
    model: String,
}

// Multipart form upload
let form = Form::new()
    .part("audio", audio_part)
    .text("model", self.model.clone())
    .text("language", language);

// JSON response parsing
#[derive(Deserialize)]
struct Response {
    text: String,
    // ... other fields
}
```

### CLI-Based Providers

For command-line tools:

```rust
// Binary detection and validation
let command_path = which("speech-tool")
    .context("Speech tool not found")?;

// Command execution
let output = Command::new(&command_path)
    .arg("--input").arg(audio_path)
    .arg("--output").arg("text")
    .output()
    .context("Failed to execute speech tool")?;

// Output parsing
let transcription = String::from_utf8_lossy(&output.stdout)
    .trim().to_string();
```

### File-Based Providers

For providers that work with temporary files:

```rust
// Create temp output file
let output_path = audio_path.with_extension("txt");

// Process file
let output = Command::new(&tool_path)
    .arg(audio_path)
    .arg("--output").arg(&output_path)
    .output()?;

// Read result and cleanup
let transcription = std::fs::read_to_string(&output_path)?;
let _ = std::fs::remove_file(&output_path); // Cleanup
```

## Real-World Examples

Check out the existing providers for reference:

- **`openai_api.rs`** - API-based provider with HTTP multipart uploads
- **`openai_cli.rs`** - CLI-based provider with binary detection and file I/O  
- **`whisper_cpp.rs`** - CLI-based provider with fallback strategies

## Submitting Your Provider

When contributing a new provider:

1. **Create a feature branch**: `git checkout -b add-superspeech-provider`
2. **Add comprehensive tests** for your provider
3. **Update documentation** including configuration guide
4. **Test thoroughly** with real API keys/services
5. **Follow existing code style** and linting rules
6. **Create a pull request** with:
   - Clear description of the provider
   - Configuration examples
   - Any special setup requirements
   - Test results

## Getting Help

- **Study existing providers** in `src/whisper/providers/`
- **Check the trait definition** in `src/whisper/provider.rs`
- **Review configuration handling** in `src/whisper/mod.rs`
- **Ask questions** in GitHub issues or discussions

Happy coding! 🚀