mod config;
mod shaders;
mod utils;

use hyprland::event_listener::EventListener;
use log::{debug, error, info};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use shaders::night_light::NightLightShader;
use shaders::shader::{self, Shader};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn main() -> hyprland::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let config_path = config::path();
    let config_data = Arc::new(Mutex::new(load_config_and_shaders(&config_path)));

    // Channel for notifying when the config file changes
    let (tx, rx) = mpsc::channel();

    let mut watcher = RecommendedWatcher::new(
        tx,
        notify::Config::default()
            .with_poll_interval(Duration::from_secs(2))
            .with_compare_contents(false),
    )
    .unwrap();

    watcher
        .watch(config_path.as_ref(), RecursiveMode::NonRecursive)
        .unwrap();

    let config_data_clone = Arc::clone(&config_data);

    if config_data_clone.lock().unwrap().hot_reload {
        // Spawn a thread to watch for config changes and reload shaders
        let debounce_delay = Duration::from_millis(2000);
        let mut last_event_time = Instant::now();
        thread::spawn(move || loop {
            match rx.recv() {
                Ok(_) => {
                    let now = Instant::now();
                    if now.duration_since(last_event_time) > debounce_delay {
                        info!("Config file changed. Reloading...");

                        let new_config = load_config_and_shaders(&config_path);

                        let mut config_data = config_data_clone.lock().unwrap();

                        // Only load config if it's not the same and it contains data
                        if new_config != *config_data
                            && new_config.night_light_shader.is_some()
                            && !new_config.vibrance_shaders.is_empty()
                        {
                            *config_data = load_config_and_shaders(&config_path);
                            last_event_time = now;
                        }
                    } else {
                        info!("Ignoring duplicate event within debounce period");
                    }
                }
                Err(error) => error!("Watch error: {:?}", error),
            }
        });
    }

    // Setup the event listener
    let mut event_listener = EventListener::new();

    // Event handler logic
    event_listener.add_active_window_changed_handler(move |data| {
        let data = data.unwrap();
        let applied_shader = shader::get().unwrap_or("null".to_string());
        debug!("Current shader: {}", applied_shader);
        let mut shader_to_apply: Option<Box<dyn shader::Shader>> = None;

        // Access the current config and shaders
        let config_data = config_data.lock().unwrap();

        // Should apply night light shader?
        if config_data.night_light_shader.is_some() {
            let shader = config_data.night_light_shader.clone().unwrap();
            if shader.should_apply(Some(data.class.to_string()), Some(data.title.to_string())) {
                shader_to_apply = Some(Box::new(shader));
            }
        }

        // Should apply vibrance shader?
        for vibrance_shader in &config_data.vibrance_shaders {
            if vibrance_shader
                .should_apply(Some(data.class.to_string()), Some(data.title.to_string()))
            {
                shader_to_apply = Some(Box::new(vibrance_shader.clone()));
                break;
            }
        }

        // Remove current shader if none should apply
        if shader_to_apply.is_none() && applied_shader != *"null" {
            shader::remove().unwrap();
            return;
        } else if shader_to_apply.is_none() {
            return;
        }

        let shader_to_apply = shader_to_apply.unwrap();
        // Apply shader if needed
        if shader_to_apply.hash() != applied_shader {
            shader::apply(shader_to_apply.as_ref()).unwrap();
        }
    });

    event_listener.start_listener()?;

    Ok(())
}

fn load_config_and_shaders(config_path: &str) -> ConfigData {
    let cfg = config::load(config_path.to_string());
    if cfg.is_none() {
        return ConfigData {
            night_light_shader: None,
            vibrance_shaders: [].to_vec(),
            hot_reload: false,
        };
    }

    let cfg = cfg.unwrap();
    info!("Config loaded: {:?}", cfg);

    let night_light_shader: Option<NightLightShader>;
    if cfg.night_light.latitude.is_some() && cfg.night_light.longitude.is_some() {
        night_light_shader = Some(shaders::night_light::new_from_location(
            cfg.night_light.enabled,
            cfg.night_light.latitude.unwrap(),
            cfg.night_light.longitude.unwrap(),
            cfg.night_light.temperature,
            None,
        ));
    } else if cfg.night_light.start_time.is_some() && cfg.night_light.end_time.is_some() {
        night_light_shader = Some(shaders::night_light::new(
            cfg.night_light.enabled,
            cfg.night_light.start_time.unwrap(),
            cfg.night_light.end_time.unwrap(),
            cfg.night_light.temperature,
            None,
        ));
    } else {
        night_light_shader = None;
    }

    let vibrance_shaders: Vec<shaders::vibrance::VibranceShader> = cfg
        .vibrance_configs
        .into_iter()
        .map(|vibrance_cfg| {
            shaders::vibrance::new(
                vibrance_cfg.window_class,
                vibrance_cfg.window_title,
                vibrance_cfg.strength,
            )
        })
        .collect();

    ConfigData {
        night_light_shader,
        vibrance_shaders,
        hot_reload: cfg.hot_reload.unwrap_or(false),
    }
}

#[derive(PartialEq)]
struct ConfigData {
    night_light_shader: Option<shaders::night_light::NightLightShader>,
    vibrance_shaders: Vec<shaders::vibrance::VibranceShader>,
    hot_reload: bool,
}
