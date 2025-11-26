use std::fs;
use std::process;

// Import our custom modules
mod config;
mod errors;
mod toml_extract;
mod tts;
mod ui;

use config::AppConfig;
use errors::TtsError;
use tts::create_tts_engine;
use ui::{
    print_config, print_error, print_info, print_stats, print_success, print_voice_header,
    show_banner, show_comprehensive_help,
};

/// Main entry point for the text-to-speech application
fn main() {
    if let Err(e) = run_app() {
        print_error(&e.to_string());
        process::exit(1);
    }
}

/// Main application logic with clean error handling
fn run_app() -> Result<(), TtsError> {
    // Show banner and version info
    show_banner();
    toml_extract::main();

    // Parse configuration from command-line arguments
    let config = AppConfig::from_args()?;

    // Handle special modes first
    if config.list_voices {
        return handle_list_voices();
    }

    if config.show_help {
        show_comprehensive_help();
        return Ok(());
    }

    // Main text-to-speech workflow
    read_and_speak_file(&config)
}

/// Handle voice listing mode
fn handle_list_voices() -> Result<(), TtsError> {
    print_voice_header();
    let tts_engine = create_tts_engine();
    let voices = tts_engine.list_voices()?;
    println!("{}", voices);
    Ok(())
}

/// Read a file and convert it to speech
fn read_and_speak_file(config: &AppConfig) -> Result<(), TtsError> {
    // Read the text file
    print_info(&format!("Reading file: {}", config.file_path));
    let text_content = fs::read_to_string(&config.file_path).map_err(|e| {
        TtsError::FileError(format!("Cannot read file '{}': {}", config.file_path, e))
    })?;

    print_success("File loaded successfully!");
    print_stats(&format!(
        "Content length: {} characters",
        text_content.len()
    ));

    // Show configuration
    print_info("Converting text to speech...");
    if let Some(voice) = &config.speech_config.voice {
        print_config(&format!("Using voice: {}", voice));
    }
    if let Some(rate) = config.speech_config.rate {
        print_config(&format!("Speaking rate: {} words per minute", rate));
    }

    // Convert to speech
    let tts_engine = create_tts_engine();
    tts_engine.speak(&text_content, &config.speech_config)?;

    print_success("Text-to-speech completed successfully!");
    Ok(())
}
