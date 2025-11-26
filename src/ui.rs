use colored::Colorize;

/// Display the application banner
pub fn show_banner() {
    let banner = "
\t ████████╗   ███████╗   ██╗  ██╗   ████████╗   
\t ╚══██╔══╝   ██╔════╝   ╚██╗██╔╝   ╚══██╔══╝   
\t    ██║      █████╗      ╚███╔╝       ██║      
\t    ██║      ██╔══╝      ██╔██╗       ██║      
\t    ██║      ███████╗   ██╔╝ ██╗      ██║      
\t    ╚═╝      ╚══════╝   ╚═╝  ╚═╝      ╚═╝      
\t 
\t ████████╗    ██████╗    
\t ╚══██╔══╝   ██╔═══██╗   
\t    ██║      ██║   ██║   
\t    ██║      ██║   ██║   
\t    ██║      ╚██████╔╝   
\t    ╚═╝       ╚═════╝    
\t 
\t  ██╗   ██╗    ██████╗    ██╗    ██████╗   ███████╗   
\t  ██║   ██║   ██╔═══██╗   ██║   ██╔════╝   ██╔════╝   
\t  ██║   ██║   ██║   ██║   ██║   ██║        █████╗     
\t  ╚██╗ ██╔╝   ██║   ██║   ██║   ██║        ██╔══╝     
\t   ╚████╔╝    ╚██████╔╝   ██║   ╚██████╗   ███████╗   
\t    ╚═══╝      ╚═════╝    ╚═╝    ╚═════╝   ╚══════╝

\t\t T2V - Text to Voice Reader
";

    println!("{}", banner.bright_cyan().bold());
}

/// Print success message
pub fn print_success(message: &str) {
    println!("{}", format!("\t ✅ {}", message).bright_green().bold());
}

/// Print error message
pub fn print_error(message: &str) {
    eprintln!("{}", format!("\t ❌ {}", message).bright_red().bold());
}

/// Print info message
pub fn print_info(message: &str) {
    println!("{}", format!("\t 📖 {}", message).bright_blue().bold());
}

/// Print config message
pub fn print_config(message: &str) {
    println!("{}", format!("\t 🎭 {}", message).bright_purple().bold());
}

/// Print stats message
pub fn print_stats(message: &str) {
    println!("{}", format!("\t 📊 {}", message).bright_yellow().bold());
}

/// Print voice list header
pub fn print_voice_header() {
    println!(
        "{}",
        "🎭 Available voices on this system:".bright_cyan().bold()
    );
    println!();
}

/// Display comprehensive help with usage examples and tips
pub fn show_comprehensive_help() {
    println!(
        "{}",
        "🎭 Text-to-Voice Reader - Comprehensive Usage Guide"
            .bright_cyan()
            .bold()
    );
    println!("{}", "=".repeat(60));
    println!();

    println!("{}", "📋 BASIC USAGE EXAMPLES:".bright_yellow().bold());
    println!("  # Read default file (sample.txt) with system default voice");
    println!("  cargo run");
    println!();
    println!("  # Read a specific file");
    println!("  cargo run -- --file welcome.txt");
    println!("  cargo run -- -f /path/to/your/document.txt");
    println!();

    println!("{}", "🎭 VOICE SELECTION EXAMPLES:".bright_yellow().bold());
    println!("  # Use a specific voice");
    println!("  cargo run -- --voice Victoria");
    println!("  cargo run -- --voice Samantha");
    println!("  cargo run -- --voice \"Good News\"");
    println!();
    println!("  # Popular voice recommendations:");
    println!("  --voice Alex         # Default male voice (clear)");
    println!("  --voice Victoria     # British female voice (elegant)");
    println!("  --voice Samantha     # American female voice (friendly)");
    println!("  --voice Daniel       # British male voice (professional)");
    println!("  --voice \"Bad News\"   # Dramatic/ominous voice (fun)");
    println!("  --voice Zarvox       # Robot voice (entertaining)");
    println!();

    println!("{}", "⚡ SPEECH RATE EXAMPLES:".bright_yellow().bold());
    println!("  # Control speaking speed (words per minute)");
    println!("  cargo run -- --rate 120    # Slow and clear");
    println!("  cargo run -- --rate 200    # Normal speed");
    println!("  cargo run -- --rate 300    # Fast reading");
    println!("  cargo run -- --rate 400    # Very fast");
    println!();

    println!("{}", "🔀 COMBINING OPTIONS:".bright_yellow().bold());
    println!("  # Custom file + voice + speed");
    println!("  cargo run -- --file story.txt --voice Victoria --rate 180");
    println!();
    println!("  # Dramatic reading with slow pace");
    println!("  cargo run -- --file poem.txt --voice \"Bad News\" --rate 140");
    println!();
    println!("  # Quick overview with fast speech");
    println!("  cargo run -- --file summary.txt --voice Alex --rate 350");
    println!();

    println!("{}", "🔍 DISCOVERY COMMANDS:".bright_yellow().bold());
    println!("  # List all available voices");
    println!("  cargo run -- --list-voices");
    println!("  cargo run -- -l");
    println!();
    println!("  # Show this comprehensive help");
    println!("  cargo run -- --bighelp");
    println!();

    println!("{}", "💡 PRO TIPS:".bright_green().bold());
    println!("  • Use quotes around voice names with spaces: --voice \"Good News\"");
    println!("  • Rate 200 is approximately normal human speech");
    println!("  • Rates below 150 are good for learning/accessibility");
    println!("  • Rates above 300 are useful for quick content review");
    println!("  • Try different voices to find your preference");
    println!();

    println!(
        "{}",
        "🚀 Happy text-to-speech reading!".bright_cyan().bold()
    );
}
