use crate::errors::{TtsError, TtsResult};
use std::process::Command;

/// Configuration for text-to-speech synthesis
#[derive(Debug, Clone)]
pub struct SpeechConfig {
    pub voice: Option<String>,
    pub rate: Option<u32>,
}

impl Default for SpeechConfig {
    fn default() -> Self {
        Self {
            voice: None,
            rate: Some(200), // Default to normal speech rate
        }
    }
}

/// Trait for text-to-speech engines
pub trait TextToSpeech {
    fn speak(&self, text: &str, config: &SpeechConfig) -> TtsResult<()>;
    fn list_voices(&self) -> TtsResult<String>;
}

/// macOS text-to-speech implementation using the 'say' command
pub struct MacOsTts;

impl TextToSpeech for MacOsTts {
    fn speak(&self, text: &str, config: &SpeechConfig) -> TtsResult<()> {
        let mut cmd = Command::new("say");

        if let Some(voice) = &config.voice {
            cmd.args(["-v", voice]);
        }

        if let Some(rate) = config.rate {
            cmd.args(["-r", &rate.to_string()]);
        }

        let output = cmd.arg(text).output()?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            // Check if the error is due to an invalid voice
            if error_msg.contains("invalid voice") || error_msg.contains("voice not found") {
                let voice_name = config.voice.as_deref().unwrap_or("unknown");
                return Err(TtsError::VoiceNotFound(format!(
                    "Voice '{}' not found on macOS. Use --list-voices to see available options",
                    voice_name
                )));
            }
            return Err(TtsError::SpeechError(format!(
                "macOS TTS failed: {}",
                error_msg
            )));
        }

        Ok(())
    }

    fn list_voices(&self) -> TtsResult<String> {
        let output = Command::new("say").arg("-v").arg("?").output()?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(TtsError::SystemError(
                "Failed to list macOS voices".to_string(),
            ))
        }
    }
}

/// Windows text-to-speech implementation using PowerShell and SAPI
pub struct WindowsTts;

impl TextToSpeech for WindowsTts {
    fn speak(&self, text: &str, config: &SpeechConfig) -> TtsResult<()> {
        let mut ps_script = String::from("Add-Type -AssemblyName System.Speech; ");
        ps_script.push_str("$synth = New-Object System.Speech.Synthesis.SpeechSynthesizer; ");

        if let Some(voice) = &config.voice {
            ps_script.push_str(&format!("$synth.SelectVoice('{}'); ", voice));
        }

        if let Some(rate) = config.rate {
            // Convert words per minute to SAPI rate scale (-10 to 10)
            let sapi_rate = ((rate as i32 - 200) / 50).clamp(-10, 10);
            ps_script.push_str(&format!("$synth.Rate = {}; ", sapi_rate));
        }

        // Escape single quotes in text
        let escaped_text = text.replace("'", "''");
        ps_script.push_str(&format!("$synth.Speak('{}');", escaped_text));

        let output = Command::new("powershell")
            .args(["-Command", &ps_script])
            .output()?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            // Check if the error is due to an invalid voice
            if error_msg.contains("Cannot find voice") || error_msg.contains("voice not found") {
                let voice_name = config.voice.as_deref().unwrap_or("unknown");
                return Err(TtsError::VoiceNotFound(format!(
                    "Voice '{}' not found on Windows. Use --list-voices to see available options",
                    voice_name
                )));
            }
            return Err(TtsError::SpeechError(format!(
                "Windows TTS failed: {}",
                error_msg
            )));
        }

        Ok(())
    }

    fn list_voices(&self) -> TtsResult<String> {
        let ps_script = "Add-Type -AssemblyName System.Speech; \
                        $synth = New-Object System.Speech.Synthesis.SpeechSynthesizer; \
                        $synth.GetInstalledVoices() | ForEach-Object { \
                            Write-Host $_.VoiceInfo.Name, '-', $_.VoiceInfo.Description \
                        }";

        let output = Command::new("powershell")
            .args(["-Command", ps_script])
            .output()?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(TtsError::SystemError(
                "Failed to list Windows voices".to_string(),
            ))
        }
    }
}

/// Linux text-to-speech implementation using espeak
pub struct LinuxTts;

impl TextToSpeech for LinuxTts {
    fn speak(&self, text: &str, config: &SpeechConfig) -> TtsResult<()> {
        let mut cmd = Command::new("espeak");

        if let Some(voice) = &config.voice {
            cmd.args(["-v", voice]);
        }

        if let Some(rate) = config.rate {
            cmd.args(["-s", &rate.to_string()]);
        }

        let output = cmd.arg(text).output()?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            // Check if the error is due to an invalid voice
            if error_msg.contains("voice not available") || error_msg.contains("unknown voice") {
                let voice_name = config.voice.as_deref().unwrap_or("unknown");
                return Err(TtsError::VoiceNotFound(format!(
                    "Voice '{}' not found on Linux. Use --list-voices to see available options",
                    voice_name
                )));
            }
            return Err(TtsError::SpeechError(format!(
                "Linux TTS failed: {}",
                error_msg
            )));
        }

        Ok(())
    }

    fn list_voices(&self) -> TtsResult<String> {
        let output = Command::new("espeak").arg("--voices").output()?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(TtsError::SystemError(
                "Failed to list Linux voices".to_string(),
            ))
        }
    }
}

/// Factory function to create the appropriate TTS engine for the current platform
pub fn create_tts_engine() -> Box<dyn TextToSpeech> {
    if cfg!(target_os = "macos") {
        Box::new(MacOsTts)
    } else if cfg!(target_os = "windows") {
        Box::new(WindowsTts)
    } else {
        Box::new(LinuxTts)
    }
}
