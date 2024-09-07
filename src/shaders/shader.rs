use std::env;
use std::fs::File;
use std::io::Write;

use hyprland::keyword::Keyword;
use log::info;

pub trait Shader {
    fn should_apply(&self, window_class: String, window_title: String) -> bool;
    fn get(&self) -> Result<String, Box<dyn std::error::Error>>;
    fn hash(&self) -> String;
}

pub fn apply(shader: &impl Shader) -> Result<(), Box<dyn std::error::Error>> {
    let output = shader.get().unwrap();

    let path = env::temp_dir().join("shader.glsl").to_owned();

    let mut shader_file = File::create(path.clone())?;
    shader_file.write_all(output.as_bytes())?;

    remove().unwrap();
    Ok(Keyword::set(
        "decoration:screen_shader",
        path.into_os_string().into_string().unwrap(),
    )?)
}

pub fn apply_if_should(
    shader: &impl Shader,
    window_class: String,
    window_title: String,
    last_applied_shader_hash: String,
) -> Result<bool, Box<dyn std::error::Error>> {
    if last_applied_shader_hash != shader.hash() && shader.should_apply(window_class, window_title)
    {
        info!("Applying shader {}", shader.hash());
        remove().unwrap();
        apply(shader)?;
        return Ok(true);
    }

    Ok(false)
}

pub fn remove() -> Result<(), Box<dyn std::error::Error>> {
    Ok(Keyword::set("decoration:screen_shader", "[[EMPTY]]")?)
}
