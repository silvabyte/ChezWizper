pub mod openai_api;
pub mod openai_cli;
pub mod whisper_cpp;

pub use openai_api::OpenAIProvider;
pub use openai_cli::OpenAIWhisperCliProvider;
pub use whisper_cpp::WhisperCppProvider;
