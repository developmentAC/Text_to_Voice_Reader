# 🎭 Text-to-Voice Reader

Date: 25 November 2025

Oliver Bonham-Carter

Email: obonhamcarter at allegheny.edu

A powerful, cross-platform Text-to-Voice (T2V) program written in Rust that converts text files into spoken audio with customizable voices and speaking rates. T2V helps the user to listen to text files using OS voice engines, making it ideal for accessibility, learning, and multitasking.

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)  

## 📢 Introduction

Welcome to the enhanced text-to-speech program written in Rust!

This program now supports:

- Custom file selection
- Multiple voice options
- Adjustable speaking rates  
- Cross-platform compatibility

You can use this program on macOS, Windows, and Linux with their respective TTS engines.

The project has been tested on MacOS and Linux but not on Windows. 

### ✨ Features

- 📁 **Flexible File Input**: Read any text file or use the default sample
- 🎭 **Voice Selection**: Choose from 100+ system voices (macOS), including multilingual options
- ⚡ **Adjustable Speed**: Control speaking rate from slow (120 WPM) to very fast (400+ WPM)
- 🌍 **Cross-Platform**: Works on macOS, Windows, and Linux
- 🔍 **Voice Discovery**: List all available voices on your system
- 📋 **Comprehensive Help**: Built-in usage examples and tips
- 🛡️ **Robust Error Handling**: Clear error messages and graceful failure handling

## 🚀 Quick Start

### Prerequisites

Before you begin, ensure you have the following software installed:

- Rust (1.70.0 or later)
- System Text-to-Voice engine:
  - **macOS**: Built-in `say` command (included by default)
  - **Windows**: PowerShell + SAPI (included by default)
  - **Linux**: `espeak` package

I should probably say that this project is a work-in-progress which was created on a MacOS setup. T2V has not been tested on the Windows OS but it has been tested a bit on Linux to ensure that a voice was able to read a text file. Updates will come periodically with improvements and bug fixes.

### Installation

1. **Clone or download this project**
2. **Navigate to the project directory**
   ```bash
   cd sound_to_text2
   ```

3. **Build the project**

   ```bash
   cargo build --release
   ```

4. **Run the program**

   ```bash
   cargo run
   ```

5. **Quick help**

   ```bash
   cargo run -- --help
   ```

6. **Quick command help**

  ```bash
  cargo run -- --bighelp
  ```

### Linux Setup (if needed)

```bash
# Ubuntu/Debian
sudo apt-get install espeak

# Fedora/CentOS/RHEL
sudo dnf install espeak

# Arch Linux
sudo pacman -S espeak
```

## 📖 Usage

### Basic Usage

```bash
# Read the default sample file
cargo run

# Read a specific file
cargo run -- --file welcome.txt

# Use a custom path
cargo run -- --file /path/to/your/document.txt
```

### Voice Selection

```bash
# Use a specific voice
cargo run -- --voice Victoria
cargo run -- --voice Samantha
cargo run -- --voice "Good News"

# List all available voices
cargo run -- --list-voices
```

### Speed Control

```bash
# Slow and clear (good for learning)
cargo run -- --rate 120

# Normal conversational speed
cargo run -- --rate 200

# Fast reading (quick review)
cargo run -- --rate 300

# Very fast (rapid content consumption)
cargo run -- --rate 400
```

### Combining Options

```bash
# Custom file + voice + speed
cargo run -- --file story.txt --voice Victoria --rate 180

# Dramatic reading
cargo run -- --file poem.txt --voice "Bad News" --rate 140

# Quick document review
cargo run -- --file report.txt --voice Alex --rate 320
```

## 🎭 Popular Voice Recommendations

Note: Voice availability varies by platform. Below are some popular voices for macOS. Use `--list-voices` to see what is available on your MacOS system.


### What Voices Are Available? (MacOS)

``` bash
cargo run -- --list-voices
```

Output example:

```text
Albert              en_US    # Hello! My name is Albert.
Alice               it_IT    # Ciao! Mi chiamo Alice.
Alva                sv_SE    # Hej! Jag heter Alva.
Amélie              fr_CA    # Bonjour! Je m’appelle Amélie.
Amira               ms_MY    # Hi my name is Amira
Anna                de_DE    # Hallo! Ich heiße Anna.
...
Zarvox              en_US    # Hello! My name is Zarvox.
Zosia               pl_PL    # Hi my name is Zosia
Zuzana              cs_CZ    # Hi my name is Zuzana
```


### English Voices (MacOS)

- **Alex** - Default male voice (clear and reliable)
- **Victoria** - British female voice (elegant and professional)
- **Samantha** - American female voice (friendly and warm)
- **Daniel** - British male voice (authoritative)

### Fun/Character Voices

- **"Good News"** - Upbeat and cheerful
- **"Bad News"** - Dramatic and ominous
- **Zarvox** - Classic robot voice
- **Whisper** - Soft and mysterious

### International Voices

- **Thomas** - French male voice
- **Mónica** - Spanish female voice
- **Anna** - German female voice
- **Kyoko** - Japanese female voice

## 📋 Command Reference

| Option | Short | Description | Example |
|--------|-------|-------------|---------|
| `--file` | `-f` | Text file to read | `--file document.txt` |
| `--voice` | `-v` | Voice to use | `--voice Victoria` |
| `--rate` | `-r` | Speaking rate (WPM) | `--rate 250` |
| `--list-voices` | `-l` | List available voices | `--list-voices` |
| `--bighelp` | | Comprehensive usage guide | `--bighelp` |
| `--help` | `-h` | Basic help information | `--help` |
| `--version` | `-V` | Show version | `--version` |

## 🎯 Common Use Cases

Note: these commands assume the MacOS platform for voice names. Adjust voice names as needed for Windows/Linux.

### 📚 Study Aid

Perfect for reviewing notes or textbooks:

```bash
cargo run -- --file notes.txt --voice Victoria --rate 160
```

### 🎵 Audiobook Experience

Comfortable listening for longer content:

```bash
cargo run -- --file chapter1.txt --voice Daniel --rate 180
```

### 🚀 Quick Content Review

Quick Study: Have a voice read quickly to you!

```bash
cargo run -- --file summary.txt --voice Alex --rate 320
```

### 🎭 Entertainment

Fun character voices for stories!

```bash
cargo run -- --file story.txt --voice Zarvox --rate 200
```

### 🌍 Language Learning

Train your ear when learning a new language!

```bash
cargo run -- --file french-text.txt --voice Thomas --rate 140
```

## 💡 Pro Tips

- **Voice Names with Spaces**: Use quotes around voice names containing spaces

  ```bash
  cargo run -- --voice "Good News"
  ```

- **Speed Guidelines**:
  - **120-150 WPM**: Learning, accessibility, complex material
  - **180-220 WPM**: Normal, comfortable listening
  - **250-300 WPM**: Review, familiar content
  - **300+ WPM**: Rapid scanning, summaries

- **Voice Discovery**: Use `--list-voices` to explore all available options on your system

- **File Formats**: Works with any plain text file (`.txt`, `.md`, `.rst`, etc.)

## 🌐 Cross-Platform Notes

### macOS

- Uses built-in `say` command
- 100+ high-quality voices available
- Best voice variety and quality
- Supports all features natively

### Windows

- Uses PowerShell with SAPI (Speech API)
- Voice selection may vary by Windows version
- Some voices may have different names

### Linux

- Uses `espeak` Text-to-Voice engine
- Install with your package manager if not present
- Voice selection varies by distribution
- May have different voice names/quality

## ❓ Troubleshooting

### File Issues

- **File not found**: Ensure the file path is correct and the file exists
- **Permission denied**: Check file read permissions

### Audio Issues

- **No sound**: Check system volume and audio output device
- **Voice not found**: Use `--list-voices` to see available options
- **Distorted audio**: Try a different voice or lower speaking rate

### Performance Issues

- **Slow startup**: Large files take longer to process
- **Memory usage**: Very large files may consume significant memory
- **Rate limits**: Some systems may have limits on very fast speech rates

### Platform-Specific Issues

- **Linux**: Ensure `espeak` is installed
- **Windows**: Some voices may require additional language packs
- **macOS**: Voice downloads may be needed for some international voices

## 🔧 Development

### Building from Source

Uh, I think some of these commands were mentioned earlier, but just in case... Hey, even rocket scientists need reminders! 🚀

```bash
# Debug build (faster compilation)
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Check code formatting
cargo fmt --check

# Run linter
cargo clippy
```

## 📄 License

This project is open source. Feel free to use, modify, and distribute as needed.

## 🤝 Contributing

Contributions are welcome! Areas for improvement:

- Additional platform support
- More T2V engine options
- GUI interface
- Batch file processing
- Audio output to files
- SSML support for advanced speech control

## 🎉 Acknowledgments

- Built with the excellent [clap](https://crates.io/crates/clap) library for CLI parsing
- Utilizes system T2V engines for cross-platform compatibility
- Inspired by the need for accessible, customizable Text-to-Voice solutions


## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Contributing

Contributions are welcome! If you have ideas for improvements or want to add more features, feel free to open an issue or submit a pull request.

### A Work In Progress

Check back often to see the evolution of the project! This project is a work-in-progress. Updates will come periodically.

If you would like to contribute to this project, please do! For instance, if you see some low-hanging fruit or tasks that could add value to the project, I would love to have your insight.

Otherwise, please create an issue for bugs or errors. Since I am a teaching faculty member at Allegheny College, I may not have all the time necessary to quickly fix bugs. I welcome the Open Source Community to further the development of this project. Much thanks in advance.

If you appreciate this project, please consider clicking the project's Star button. :-)
