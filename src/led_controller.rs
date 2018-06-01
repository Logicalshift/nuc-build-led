use futures::*;
use serde_json::{Value, Map};

use std::fmt::Debug;

///
/// Available colours for the LED
///
#[allow(dead_code)]
pub enum LedColor {
    Amber,
    Cyan,
    Blue,
    Green,
    Off,
    Pink,
    Red,
    White,
    Yellow
}

///
/// Possible fade/blink states for the LED
/// 
#[allow(dead_code)]
pub enum LedFade {
    BlinkFast,
    BlinkMedium,
    BlinkSlow,
    FadeFast,
    FadeMedium,
    FadeSlow,
    None
}

///
/// Type of LED to set
/// 
#[allow(dead_code)]
pub enum LedType {
    Ring,
    Power
}

///
/// Converts a LED state to a string
/// 
pub fn led_state_string(led: LedType, brightness: u32, fade: LedFade, color: LedColor) -> String {
    let led = match led {
        LedType::Ring   => "ring",
        LedType::Power  => "power"
    };

    let fade = match fade {
        LedFade::BlinkFast      => "blink_fast",
        LedFade::BlinkMedium    => "blink_medium",
        LedFade::BlinkSlow      => "blink_slow",
        LedFade::FadeFast       => "fade_fast",
        LedFade::FadeMedium     => "fade_medium",
        LedFade::FadeSlow       => "fade_slow",
        LedFade::None           => "none"
    };

    let color = match color {
        LedColor::Amber     => "amber",
        LedColor::Blue      => "blue",
        LedColor::Cyan      => "cyan",
        LedColor::Green     => "green",
        LedColor::Off       => "off",
        LedColor::Pink      => "pink",
        LedColor::Red       => "red",
        LedColor::White     => "white",
        LedColor::Yellow    => "yellow"
    };

    format!("{},{},{},{}", led, brightness, fade, color)
}

///
/// Converts from JSON LED updates to a stream of updates for the NUC LED driver
/// 
pub fn led_controller<JsonError, JsonStream, StateFn>(input: JsonStream, get_led_state: StateFn) -> impl Stream<Item=String, Error=JsonError> 
where   JsonError: Debug, 
        JsonStream: Stream<Item=Value, Error=JsonError>,
        StateFn: FnMut(&Value) -> Option<(u32, LedFade, LedColor)> {
    // Map will represent the current state of the controller
    let mut state           = Value::Object(Map::new());
    let mut get_led_state   = get_led_state;

    input.map(move |next_value| {
        // Merge the next value with the current state
        match next_value {
            // Merge object values into the existing state
            Value::Object(new_values) => {
                state = if let Value::Object(mut existing_map) = state.take() {
                    // Merge in the values in new_values
                    for (key, value) in new_values {
                        existing_map.insert(key.clone(), value.clone());
                    }

                    // Re-use the existing map to make the new state
                    Value::Object(existing_map)
                } else {
                    // Existing state is not an object somehow
                    Value::Object(new_values)
                };
            },

            // Do nothing for other JSON types
            _ => ()
        };

        // Call the state function on the current state
        let (brightness, fade, color) = get_led_state(&state).unwrap_or((0, LedFade::None, LedColor::Off));

        // Convert to a string for the result
        led_state_string(LedType::Ring, brightness, fade, color)
    })
}
