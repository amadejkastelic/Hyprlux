mod config;
mod shaders;
mod utils;

use std::sync::{Arc, Mutex};

use config::get_config;
use hyprland::event_listener::EventListener;
use log::{debug, info};
use shaders::shader::{self, Shader};

fn main() -> hyprland::Result<()> {
    colog::init();

    let mut event_listener = EventListener::new();

    // Load config
    let cfg = get_config().unwrap();
    info!("Config loaded: {:?}", cfg);

    let applied_shader = Arc::new(Mutex::new(String::from("")));

    // Create shaders
    let night_light_shader = shaders::night_light::new(
        cfg.night_light.enabled,
        cfg.night_light.start_time,
        cfg.night_light.end_time,
        cfg.night_light.temperature,
        None,
    );

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

    event_listener.add_active_window_change_handler(move |data| {
        let data = data.unwrap();
        let mut applied = false;
        let mut applied_shader = applied_shader.lock().unwrap();
        debug!("Curent shader: {}", applied_shader.to_string());
        for vibrance_shader in &vibrance_shaders {
            if shader::apply_if_should(
                vibrance_shader,
                Some(data.window_class.clone()),
                Some(data.window_title.clone()),
                applied_shader.to_string(),
            )
            .unwrap()
            {
                *applied_shader = vibrance_shader.hash();
                applied = true;
                break;
            }
        }
        if !applied && night_light_shader.should_apply(None, None) {
            if shader::apply_if_should(&night_light_shader, None, None, applied_shader.to_string())
                .unwrap()
            {
                *applied_shader = night_light_shader.hash();
            }
        } else if !applied {
            shader::remove().unwrap();
            *applied_shader = "".to_string();
        }
    });

    event_listener.start_listener()?;

    Ok(())
}
