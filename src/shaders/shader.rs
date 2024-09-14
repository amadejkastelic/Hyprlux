use std::env;
use std::fs::File;
use std::io::Write;

use super::super::utils;
use hyprland::keyword::Keyword;
use log::info;

const SHADER_KEY: &str = "decoration:screen_shader";
const NO_SHADER: &str = "[[EMPTY]]";

pub trait Shader {
    fn should_apply(&self, window_class: Option<String>, window_title: Option<String>) -> bool;
    fn get(&self) -> Result<String, Box<dyn std::error::Error>>;
    fn hash(&self) -> String;
}

pub fn apply(shader: &dyn Shader) -> Result<(), Box<dyn std::error::Error>> {
    info!("Applying shader: {}", shader.hash());

    let output = shader.get().unwrap();

    let path = env::temp_dir().join(shader.hash()).to_owned();

    let mut shader_file = File::create(path.clone())?;
    shader_file.write_all(output.as_bytes())?;

    remove().unwrap();
    Ok(Keyword::set(
        SHADER_KEY,
        path.into_os_string().into_string().unwrap(),
    )?)
}

pub fn remove() -> Result<(), Box<dyn std::error::Error>> {
    info!("Removing active shader");
    Ok(Keyword::set(SHADER_KEY, NO_SHADER)?)
}

pub fn get() -> Option<String> {
    let shader = Keyword::get(SHADER_KEY).unwrap();
    info!("Getting shader {}", shader.value.to_string());

    if shader.value.to_string() == NO_SHADER {
        return None;
    }

    Some(utils::shader_hash_from_path(shader.value.to_string()).unwrap())
}
