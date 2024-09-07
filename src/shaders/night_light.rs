use std::{collections::HashMap, time::SystemTime};

use chrono::{Local, NaiveTime};
use strfmt::Format;

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

pub struct NightLightShader {
    enabled: bool,
    start_time: NaiveTime,
    end_time: NaiveTime,
    temperature: i32,
}

pub fn new(
    enabled: bool,
    start_time: String,
    end_time: String,
    temperature: i32,
) -> NightLightShader {
    NightLightShader {
        enabled,
        start_time: NaiveTime::parse_from_str(&start_time, TIME_FMT).unwrap(),
        end_time: NaiveTime::parse_from_str(&end_time, TIME_FMT).unwrap(),
        temperature,
    }
}

impl Shader for NightLightShader {
    fn should_apply(&self, _: String, _: String) -> bool {
        let now = Local::now();
        self.enabled && (now.time() >= self.start_time && now.time() <= self.end_time)
            || (now.time() < self.start_time && now.time() > self.end_time)
    }

    fn get(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut vars = HashMap::new();
        vars.insert("temperature".to_string(), self.temperature);

        Ok(SHADER.format(&vars).unwrap())
    }

    fn hash(&self) -> String {
        format!("night_{}", self.temperature)
    }
}
