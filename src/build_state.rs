use super::led_controller::*;

use serde_json::*;

const BRIGHTNESS: u32           = 80;
const BUILD_COLOR: LedColor     = LedColor::Cyan;
const TEST_COLOR: LedColor      = LedColor::Yellow;
const UPLOADING_COLOR: LedColor = LedColor::White;
const ERROR_COLOR: LedColor     = LedColor::Red;

///
/// Converts a JSON state representing possible building projects (say for Jenkins)
/// into an LED state to display
/// 
pub fn build_state(state: &Value) -> Option<(u32, LedFade, LedColor)> {
    match state {
        // Objects can contain build states
        Value::Object(values) => {
            let is_building     = values.iter().any(|(_key, value)| value.get("building") == Some(&Value::Bool(true)));
            let is_testing      = values.iter().any(|(_key, value)| value.get("testing") == Some(&Value::Bool(true)));
            let is_uploading    = values.iter().any(|(_key, value)| value.get("uploading") == Some(&Value::Bool(true)));
            let is_errored      = values.iter().any(|(_key, value)| value.get("error") == Some(&Value::Bool(true)));

            // If there's an error to be cleared, then 
            let activity_fade   = if is_errored { LedFade::FadeFast } else { LedFade::FadeMedium };

            if is_building {
                // Something building
                Some((BRIGHTNESS, activity_fade, BUILD_COLOR))
            } else if is_testing {
                Some((BRIGHTNESS, activity_fade, TEST_COLOR))
            } else if is_uploading {
                Some((BRIGHTNESS, LedFade::FadeFast, UPLOADING_COLOR))
            } else if is_errored {
                // Nothing building, but a build is failing
                Some((BRIGHTNESS, LedFade::FadeSlow, ERROR_COLOR))
            } else {
                // No build state
                None
            }
        },

        // Unknown state
        _ => None
    }
}