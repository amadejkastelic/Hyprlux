use log::{error, info};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use serde::Deserialize;
use std::{env, fs, path::Path, time::Duration};

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

pub fn load(config_path: String) -> Result<Config, Box<dyn std::error::Error>> {
    info!("Loading config file at {}", &config_path);

    let contents = fs::read_to_string(config_path).unwrap();

    // Return default config if no config file exists
    if contents.is_empty() {
        error!("No config file found. Using default config.");
        return Ok(Config::default());
    }

    Ok(toml::from_str(&contents).unwrap())
}

pub fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(
        tx,
        notify::Config::default()
            .with_poll_interval(Duration::from_secs(2))
            .with_compare_contents(false),
    )?;

    watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

    for res in rx {
        match res {
            Ok(event) => log::info!("Change: {event:?}"),
            Err(error) => log::error!("Error: {error:?}"),
        }
    }

    Ok(())
}
