// Configuration file loading and management.
// Handles reading TOML config from ./speedo.toml, ./.speedo.toml, or ~/.speedo.toml.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    #[serde(default = "default_user_agent")]
    pub user_agent: String,
    #[serde(default)]
    pub custom_servers: Vec<CustomServer>,
    #[serde(default)]
    pub interactive: bool,
    #[serde(default = "default_speed_unit")]
    pub speed_unit: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpeedUnit {
    BitsMetric,      // Mbps, Gbps (megabits, gigabits per second - 1000-based)
    BitsBinary,      // Mibps, Gibps (mebibits, gibibits per second - 1024-based)
    BytesMetric,     // MB/s, GB/s (megabytes, gigabytes per second - 1000-based)
    BytesBinary,     // MiB/s, GiB/s (mebibytes, gibibytes per second - 1024-based)
}

impl SpeedUnit {
    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "bits" | "bits-metric" | "mbps" | "gbps" => SpeedUnit::BitsMetric,
            "bits-binary" | "mibps" | "gibps" => SpeedUnit::BitsBinary,
            "bytes" | "bytes-metric" | "mb/s" | "gb/s" => SpeedUnit::BytesMetric,
            "bytes-binary" | "mib/s" | "gib/s" => SpeedUnit::BytesBinary,
            _ => SpeedUnit::BytesMetric, // Default to MB/s
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CustomServer {
    pub name: String,
    pub url: String,
}

fn default_user_agent() -> String {
    "Mozilla/5.0".to_string()
}

fn default_speed_unit() -> String {
    "bytes-metric".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            user_agent: default_user_agent(),
            custom_servers: Vec::new(),
            interactive: false,
            speed_unit: default_speed_unit(),
        }
    }
}

fn get_config_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    
    if let Ok(cwd) = std::env::current_dir() {
        paths.push(cwd.join("speedo.toml"));
        paths.push(cwd.join(".speedo.toml"));
    }
    
    if let Some(home_dir) = dirs::home_dir() {
        paths.push(home_dir.join(".speedo.toml"));
    }
    
    paths
}

pub fn load_config() -> Config {
    use colored::*;
    
    for path in get_config_paths() {
        if path.exists() {
            if let Ok(contents) = std::fs::read_to_string(&path) {
                if let Ok(config) = toml::from_str::<Config>(&contents) {
                    println!("{}", format!("Loaded config from: {}", path.display()).bright_black());
                    return config;
                }
            }
        }
    }
    Config::default()
}
