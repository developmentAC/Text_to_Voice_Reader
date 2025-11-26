use crate::errors::{TtsError, TtsResult};
use crate::tts::SpeechConfig;
use clap::{Arg, ArgMatches, Command as ClapCommand};

/// Application configuration
#[derive(Debug)]
pub struct AppConfig {
    pub file_path: String,
    pub speech_config: SpeechConfig,
    pub list_voices: bool,
    pub show_help: bool,
}

impl AppConfig {
    /// Create configuration from command-line arguments
    pub fn from_args() -> TtsResult<Self> {
        let matches = Self::build_cli().get_matches();
        Self::parse_matches(&matches)
    }

    /// Build the command-line interface
    fn build_cli() -> ClapCommand {
        ClapCommand::new("Text-to-Speech Reader")
            .version(env!("CARGO_PKG_VERSION")) // Use version from Cargo.toml
            .author("Text-to-Voice Reader")
            .about("Reads text files aloud with customizable voice and speed")
            .arg(
                Arg::new("file")
                    .short('f')
                    .long("file")
                    .value_name("FILE")
                    .help("Text file to read aloud")
                    .default_value("sample.txt"),
            )
            .arg(
                Arg::new("voice")
                    .short('v')
                    .long("voice")
                    .value_name("VOICE")
                    .help("Voice to use (e.g., Victoria, Alex, Samantha)"),
            )
            .arg(
                Arg::new("rate")
                    .short('r')
                    .long("rate")
                    .value_name("RATE")
                    .help("Speaking rate (words per minute, e.g., 200)")
                    .value_parser(clap::value_parser!(u32)),
            )
            .arg(
                Arg::new("list-voices")
                    .short('l')
                    .long("list-voices")
                    .help("List available voices on this system")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("bighelp")
                    .long("bighelp")
                    .help("Show comprehensive usage examples and tips")
                    .action(clap::ArgAction::SetTrue),
            )
    }

    /// Parse command-line arguments into configuration
    fn parse_matches(matches: &ArgMatches) -> TtsResult<Self> {
        let file_path = matches.get_one::<String>("file").unwrap().clone();

        let voice = matches.get_one::<String>("voice").cloned();
        let rate = matches.get_one::<u32>("rate").copied();

        // Validate rate if provided
        if let Some(rate) = rate {
            Self::validate_rate(rate)?;
        }

        let speech_config = SpeechConfig { voice, rate };

        Ok(AppConfig {
            file_path,
            speech_config,
            list_voices: matches.get_flag("list-voices"),
            show_help: matches.get_flag("bighelp"),
        })
    }

    /// Validate that the speech rate is within reasonable bounds
    fn validate_rate(rate: u32) -> TtsResult<()> {

        // valid range is 50-1000 wpm
 
        //clippy likes this line better, which does not work properly with all compilers.
        // if !(500..=1000).contains(&rate) { 

        if rate < 50 || rate > 1000 { 
            return Err(TtsError::ConfigError(format!(
                "Speech rate {} is outside valid range (50-1000 words per minute)",
                rate
            )));
        }
        Ok(())
    }
}
