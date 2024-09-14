mod config;
mod shaders;
mod utils;

use hyprland::event_listener::EventListener;
use log::info;
use shaders::shader::{self, Shader};

fn main() -> hyprland::Result<()> {
    colog::init();

    let mut event_listener = EventListener::new();

    let config_path = config::path();

    // Load config
    let cfg = config::load(config_path).unwrap();
    info!("Config loaded: {:?}", cfg);

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
        let applied_shader = shader::get().unwrap_or("null".to_string());
        info!("Curent shader: {}", applied_shader);
        let mut shader_to_apply: Option<Box<dyn shader::Shader>> = None;

        // Should apply night light shader?
        if night_light_shader.should_apply(
            Some(data.window_class.to_string()),
            Some(data.window_title.to_string()),
        ) {
            shader_to_apply = Some(Box::new(night_light_shader.clone()))
        }

        // Should apply vibrance shader?
        for vibrance_shader in &vibrance_shaders {
            if vibrance_shader.should_apply(
                Some(data.window_class.to_string()),
                Some(data.window_title.to_string()),
            ) {
                shader_to_apply = Some(Box::new(vibrance_shader.clone()));
                break;
            }
        }

        // Remove current shader
        if shader_to_apply.is_none() && applied_shader != "null".to_string() {
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
