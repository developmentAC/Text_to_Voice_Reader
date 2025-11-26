use std::fmt;

/// Custom error types for the text-to-speech application
#[derive(Debug)]
pub enum TtsError {
    /// File not found or cannot be read
    FileError(String),
    /// TTS command execution failed
    SpeechError(String),
    /// Voice not found on system
    VoiceNotFound(String),
    /// Invalid configuration parameter
    ConfigError(String),
    /// System command failed
    SystemError(String),
}

impl fmt::Display for TtsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TtsError::FileError(msg) => write!(f, "File error: {}", msg),
            TtsError::SpeechError(msg) => write!(f, "Speech synthesis error: {}", msg),
            TtsError::VoiceNotFound(msg) => write!(f, "Voice not found: {}", msg),
            TtsError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            TtsError::SystemError(msg) => write!(f, "System error: {}", msg),
        }
    }
}

impl std::error::Error for TtsError {}

/// Convert from std::io::Error to TtsError
impl From<std::io::Error> for TtsError {
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            std::io::ErrorKind::NotFound => TtsError::FileError(error.to_string()),
            _ => TtsError::SystemError(error.to_string()),
        }
    }
}

/// Convenient type alias for Results using TtsError
pub type TtsResult<T> = Result<T, TtsError>;
