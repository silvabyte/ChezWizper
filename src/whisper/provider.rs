use anyhow::Result;
use std::future::Future;
use std::path::Path;
use std::pin::Pin;

pub trait TranscriptionProvider: Send + Sync {
    fn name(&self) -> &'static str;

    fn is_available(&self) -> bool;

    fn transcribe<'a>(
        &'a self,
        audio_path: &'a Path,
        language: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<String>> + Send + 'a>>;
}
