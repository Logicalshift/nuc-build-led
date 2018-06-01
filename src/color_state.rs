use super::led_controller::*;

use serde_json::Value;

/// Brightness for 'color' states
const BRIGHTNESS: u32 = 50;

///
/// Finds 'color' keys in the state and sets the LED color accordingly
/// 
pub fn color_state(state: &Value) -> Option<(u32, LedFade, LedColor)> {
    if let Value::Object(state) = state {
        // Search the states for 'color' values
        let mut colors = state.iter()
            .filter_map(|(_key, value)| value.get("color"))
            .filter_map(|color| color.as_str())
            .filter_map(|color| match color {
                "white"     => Some(LedColor::White),
                "red"       => Some(LedColor::Red),
                "green"     => Some(LedColor::Green),
                "blue"      => Some(LedColor::Blue),
                "cyan"      => Some(LedColor::Cyan),
                "pink"      => Some(LedColor::Pink),
                "yellow"    => Some(LedColor::Yellow),
                "amber"     => Some(LedColor::Amber),

                _           => None
            });
        
        if let Some(color) = colors.nth(0) {
            // Use the first colour we find
            Some((BRIGHTNESS, LedFade::None, color))
        } else {
            // No colour
            None
        }
    } else {
        // State is not an object
        None
    }
}

