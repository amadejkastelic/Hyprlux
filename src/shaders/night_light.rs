use std::collections::HashMap;

use chrono::NaiveTime;
use strfmt::Format;

use super::super::utils::Time;
use super::shader::Shader;

const SHADER: &str = "
precision highp float;
varying vec2 v_texcoord;
uniform sampler2D tex;

const float temperature = {temperature}.0;
const float temperatureStrength = 1.0;

#define WithQuickAndDirtyLuminancePreservation
const float LuminancePreservationFactor = 1.0;

// function from https://www.shadertoy.com/view/4sc3D7
// valid from 1000 to 40000 K (and additionally 0 for pure full white)
vec3 colorTemperatureToRGB(const in float temperature) {{
    // values from: http://blenderartists.org/forum/showthread.php?270332-OSL-Goodness&p=2268693&viewfull=1#post2268693
    mat3 m = (temperature <= 6500.0) ? mat3(vec3(0.0, -2902.1955373783176, -8257.7997278925690),
                                            vec3(0.0, 1669.5803561666639, 2575.2827530017594),
                                            vec3(1.0, 1.3302673723350029, 1.8993753891711275))
                                     : mat3(vec3(1745.0425298314172, 1216.6168361476490, -8257.7997278925690),
                                            vec3(-2666.3474220535695, -2173.1012343082230, 2575.2827530017594),
                                            vec3(0.55995389139931482, 0.70381203140554553, 1.8993753891711275));
    return mix(clamp(vec3(m[0] / (vec3(clamp(temperature, 1000.0, 40000.0)) + m[1]) + m[2]), vec3(0.0), vec3(1.0)),
               vec3(1.0), smoothstep(1000.0, 0.0, temperature));
}}
void main() {{
    vec4 pixColor = texture2D(tex, v_texcoord);
    // RGB
    vec3 color = vec3(pixColor[0], pixColor[1], pixColor[2]);
#ifdef WithQuickAndDirtyLuminancePreservation
    color *= mix(1.0, dot(color, vec3(0.2126, 0.7152, 0.0722)) / max(dot(color, vec3(0.2126, 0.7152, 0.0722)), 1e-5),
                 LuminancePreservationFactor);
#endif
    color = mix(color, color * colorTemperatureToRGB(temperature), temperatureStrength);
    vec4 outCol = vec4(color, pixColor[3]);
    gl_FragColor = outCol;
}}
";
const TIME_FMT: &str = "%H:%M";

#[derive(Clone, PartialEq)]
pub struct NightLightShader {
    enabled: bool,
    start_time: NaiveTime,
    end_time: NaiveTime,
    shader_vars: HashMap<String, String>,
    time_impl: Time,
}

pub fn new(
    enabled: bool,
    start_time: String,
    end_time: String,
    temperature: i32,
    mock_time: Option<String>,
) -> NightLightShader {
    let time: Time;
    match mock_time {
        Some(p) => time = Time::new(Some(NaiveTime::parse_from_str(&p, TIME_FMT).unwrap())),
        None => time = Time::new(None),
    }

    let shader_vars = HashMap::from([("temperature".to_string(), temperature.to_string())]);

    NightLightShader {
        enabled,
        start_time: NaiveTime::parse_from_str(&start_time, TIME_FMT).unwrap(),
        end_time: NaiveTime::parse_from_str(&end_time, TIME_FMT).unwrap(),
        shader_vars,
        time_impl: time,
    }
}

impl Shader for NightLightShader {
    fn should_apply(&self, _: Option<String>, _: Option<String>) -> bool {
        let now = self.time_impl.now();

        if !self.enabled {
            return false;
        }

        if self.start_time < self.end_time {
            return self.start_time <= now && now <= self.end_time;
        }

        now >= self.start_time || now <= self.end_time
    }

    fn get(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(SHADER.format(&self.shader_vars).unwrap())
    }

    fn hash(&self) -> String {
        format!("night_{}", self.shader_vars["temperature"])
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_should_apply() {
        let shaders = [
            (
                new(false, "12:00".to_string(), "14:00".to_string(), 3500, None),
                false,
            ),
            (
                new(
                    true,
                    "13:00".to_string(),
                    "15:00".to_string(),
                    3500,
                    Some("16:00".to_string()),
                ),
                false,
            ),
            (
                new(
                    true,
                    "13:00".to_string(),
                    "15:00".to_string(),
                    3500,
                    Some("14:00".to_string()),
                ),
                true,
            ),
            (
                new(
                    true,
                    "22:00".to_string(),
                    "03:00".to_string(),
                    3500,
                    Some("04:00".to_string()),
                ),
                false,
            ),
            (
                new(
                    true,
                    "22:00".to_string(),
                    "03:00".to_string(),
                    3500,
                    Some("23:00".to_string()),
                ),
                true,
            ),
            (
                new(
                    true,
                    "22:00".to_string(),
                    "03:00".to_string(),
                    3500,
                    Some("02:00".to_string()),
                ),
                true,
            ),
            (
                new(
                    true,
                    "22:00".to_string(),
                    "03:00".to_string(),
                    3500,
                    Some("03:00".to_string()),
                ),
                true,
            ),
            (
                new(
                    true,
                    "22:00".to_string(),
                    "03:00".to_string(),
                    3500,
                    Some("22:00".to_string()),
                ),
                true,
            ),
        ];
        for (shader, expected) in shaders {
            assert_eq!(shader.should_apply(None, None), expected)
        }
    }

    #[test]
    fn test_get() {
        let time = "00:00".to_string();
        let shaders = [
            (new(true, time.clone(), time.clone(), 3500, None), "3500.0"),
            (new(true, time.clone(), time.clone(), 5000, None), "5000.0"),
            (new(true, time.clone(), time.clone(), 1, None), "1.0"),
        ];
        for (shader, expected) in shaders {
            assert!(shader
                .get()
                .unwrap()
                .contains(&format!("const float temperature = {};", expected)))
        }
    }

    #[test]
    fn test_hash() {
        let time = "00:00".to_string();
        let shaders = [
            (
                new(true, time.clone(), time.clone(), 3500, None),
                "night_3500",
            ),
            (
                new(true, time.clone(), time.clone(), 5000, None),
                "night_5000",
            ),
            (new(true, time.clone(), time.clone(), 1, None), "night_1"),
        ];
        for (shader, expected) in shaders {
            assert_eq!(shader.hash(), expected)
        }
    }
}
