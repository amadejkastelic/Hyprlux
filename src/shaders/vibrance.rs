use std::collections::HashMap;

use regex::Regex;
use strfmt::Format;

use crate::utils;

use super::shader::Shader;

const SHADER: &str = "
precision highp float;
varying vec2 v_texcoord;
uniform sampler2D tex;

const vec3 VIB_RGB_BALANCE = vec3(1.0, 1.0, 1.0);
const float VIB_VIBRANCE = {strength:.2};

const vec3 VIB_coeffVibrance = VIB_RGB_BALANCE * -VIB_VIBRANCE;

void main() {{
    vec4 pixColor = texture2D(tex, v_texcoord);
    vec3 color = vec3(pixColor[0], pixColor[1], pixColor[2]);

    vec3 VIB_coefLuma = vec3(0.212656, 0.715158, 0.072186); // try both and see which one looks nicer.

    float luma = dot(VIB_coefLuma, color);

    float max_color = max(color[0], max(color[1], color[2]));
    float min_color = min(color[0], min(color[1], color[2]));

    float color_saturation = max_color - min_color;

    vec3 p_col = vec3(vec3(vec3(vec3(sign(VIB_coeffVibrance) * color_saturation) - 1.0) * VIB_coeffVibrance) + 1.0);

    pixColor[0] = mix(luma, color[0], p_col[0]);
    pixColor[1] = mix(luma, color[1], p_col[1]);
    pixColor[2] = mix(luma, color[2], p_col[2]);

    gl_FragColor = pixColor;
}}
";

#[derive(Clone, PartialEq)]
pub struct VibranceShader {
    window_class: String,
    window_title: String,
    strength: i32,
}

pub fn new(window_class: String, window_title: String, strength: i32) -> VibranceShader {
    VibranceShader {
        window_class,
        window_title,
        strength: utils::int_in_range(strength, 1, 1000),
    }
}

impl Shader for VibranceShader {
    fn should_apply(&self, window_class: Option<String>, window_title: Option<String>) -> bool {
        let window_class = window_class.unwrap_or("".to_string());
        let window_title = window_title.unwrap_or("".to_string());

        let mut class_match = false;
        let mut title_match = false;

        if !window_class.is_empty() {
            class_match = Regex::new(&self.window_class)
                .unwrap()
                .is_match(&window_class);
        }
        if !window_title.is_empty() {
            title_match = Regex::new(&self.window_title)
                .unwrap()
                .is_match(&window_title);
        }
        if !window_title.is_empty() && !window_class.is_empty() {
            return class_match && title_match;
        }

        class_match || title_match
    }

    fn get(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut vars = HashMap::new();
        vars.insert("strength".to_string(), (self.strength as f64) / 100.0);

        Ok(SHADER.format(&vars).unwrap())
    }

    fn hash(&self) -> String {
        format!("vibrance_{}", self.strength)
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_should_apply() {
        let shaders = [
            (
                "No match",
                new("firefox".to_string(), "firefox".to_string(), 100),
                (Some("class".to_string()), Some("title".to_string())),
                false,
            ),
            (
                "Class matches",
                new("firefox".to_string(), "firefox".to_string(), 100),
                (Some("firefox".to_string()), None),
                true,
            ),
            (
                "Title matches",
                new("firefox".to_string(), "firefox".to_string(), 100),
                (None, Some("firefox".to_string())),
                true,
            ),
            (
                "Regex class matches",
                new("^(steam_app_)(.*)$".to_string(), "".to_string(), 100),
                (
                    Some("steam_app_123".to_string()),
                    Some("Some Epic Game".to_string()),
                ),
                true,
            ),
            (
                "Regex class doesn't match",
                new("^(steam_app_)(.*)$".to_string(), "".to_string(), 100),
                (Some("firefox".to_string()), Some("firefox".to_string())),
                false,
            ),
            (
                "Regex title matches",
                new("".to_string(), "^(Some Epic)(.*)$".to_string(), 100),
                (
                    Some("steam_app_123".to_string()),
                    Some("Some Epic Game".to_string()),
                ),
                true,
            ),
            (
                "Regex class doesn't match",
                new("".to_string(), "^(Some Epic)(.*)$".to_string(), 100),
                (None, Some("Other Game".to_string())),
                false,
            ),
        ];
        for (name, shader, (class, title), expected) in shaders {
            let res = shader.should_apply(class.clone(), title.clone());
            assert!(
                res == expected,
                "{} - {} - {} - {}",
                name,
                class.unwrap_or("".to_string()),
                title.unwrap_or("".to_string()),
                expected,
            )
        }
    }

    #[test]
    fn test_get() {
        let string = "".to_string();
        let shaders = [
            (new(string.clone(), string.clone(), 100), "1.00".to_string()),
            (new(string.clone(), string.clone(), 90), "0.90".to_string()),
            (
                new(string.clone(), string.clone(), 10000),
                "10.00".to_string(),
            ),
            (new(string.clone(), string.clone(), 0), "0.01".to_string()),
            (new(string.clone(), string.clone(), -10), "0.01".to_string()),
            (new(string.clone(), string.clone(), 55), "0.55".to_string()),
        ];
        for (shader, expected) in shaders {
            assert!(
                shader
                    .get()
                    .unwrap()
                    .contains(&format!("const float VIB_VIBRANCE = {};", expected)),
                "{}",
                expected,
            )
        }
    }

    #[test]
    fn test_hash() {
        let shaders = [
            (
                new("class".to_string(), "title".to_string(), 100),
                "vibrance_100".to_string(),
            ),
            (
                new("firefox".to_string(), "".to_string(), 10),
                "vibrance_10".to_string(),
            ),
            (
                new("firefox".to_string(), "firefox".to_string(), 15),
                "vibrance_15".to_string(),
            ),
        ];
        for (shader, expected) in shaders {
            assert_eq!(shader.hash(), expected)
        }
    }
}
