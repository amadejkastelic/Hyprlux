use log::{error, info};
use serde::Deserialize;
use std::{env, fs};

const DEFAULT_CONFIG_PATH: &str = "/etc/hyprlux/config.toml";

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
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub temperature: i32,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

impl Default for NightLightConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            start_time: None,
            end_time: None,
            temperature: 3500,
            latitude: None,
            longitude: None,
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

pub fn path() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        return args[1].clone().to_string();
    }

    let config_file_path = xdg::BaseDirectories::with_prefix("hypr")
        .unwrap()
        .get_config_file("hyprlux.toml");

    if config_file_path.exists() {
        return config_file_path
            .into_os_string()
            .into_string()
            .unwrap_or(DEFAULT_CONFIG_PATH.to_string());
    }

    DEFAULT_CONFIG_PATH.to_string()
}

pub fn load(config_path: String) -> Option<Config> {
    info!("Loading config file at {}", &config_path);

    let contents = fs::read_to_string(config_path).unwrap_or("".to_string());

    // Return default config if no config file exists
    if contents.is_empty() {
        error!("No config file found. Using default config.");
        return None;
    }

    Some(toml::from_str(&contents).unwrap())
}
