use crate::ui::print_config;
use colored::Colorize;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    version: String,
    edition: String,
}

#[derive(Debug, Deserialize)]
struct CargoToml {
    package: Package,
}

fn parse_cargo_toml(file_path: &str) {
    // Check if the file exists
    if !std::path::Path::new(file_path).exists() {
        println!(
            "\t {} Cargo.toml file not found; Cannot display version information.\n",
            "⚠️".bright_yellow()
        );
        return;
    }

    // Read and parse the Cargo.toml file
    match fs::read_to_string(file_path) {
        Ok(content) => {
            match toml::from_str::<CargoToml>(&content) {
                Ok(cargo_toml) => {
                    print_config(&format!(
                        "Package: {} v{} ({})",
                        cargo_toml.package.name,
                        cargo_toml.package.version,
                        cargo_toml.package.edition
                    ));
                    println!(); // Add spacing
                }
                Err(e) => {
                    println!(
                        "\t {} Failed to parse Cargo.toml: {}\n",
                        "⚠️".bright_yellow(),
                        e
                    );
                }
            }
        }
        Err(e) => {
            println!(
                "\t {} Failed to read Cargo.toml: {}\n",
                "⚠️".bright_yellow(),
                e
            );
        }
    }
}

pub fn main() {
    parse_cargo_toml("Cargo.toml");
}
