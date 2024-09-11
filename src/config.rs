use log::{error, info};
use serde::Deserialize;
use std::{env, fs};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub night_light: NightLightConfig,
    pub vibrance_configs: Vec<VibranceConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            night_light: NightLightConfig::default(),
            vibrance_configs: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct NightLightConfig {
    pub enabled: bool,
    pub start_time: String,
    pub end_time: String,
    pub temperature: i32,
}

impl Default for NightLightConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            start_time: "00:00".to_string(),
            end_time: "00:00".to_string(),
            temperature: 3500,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct VibranceConfig {
    pub window_class: String,
    pub window_title: String,
    pub strength: i32,
}

impl Default for VibranceConfig {
    fn default() -> Self {
        Self {
            window_class: "".to_string(),
            window_title: "".to_string(),
            strength: 0,
        }
    }
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_file_path: String;

    // If config file path provided as arg
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        config_file_path = args[1].clone();
    } else {
        config_file_path = xdg::BaseDirectories::with_prefix("hypr")
            .unwrap()
            .place_config_file("hyprlux.toml")
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();
    }

    info!("Loading config file at {}", &config_file_path);

    let contents = fs::read_to_string(config_file_path)
        .unwrap_or(fs::read_to_string("/etc/hyprlux/config.toml").unwrap_or("".to_string()));

    // Return default config if no config file exists
    if contents.is_empty() {
        error!("No config file found. Using default config.");
        return Ok(Config::default());
    }

    Ok(toml::from_str(&contents).unwrap())
}

pub fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
    load_config()
}
