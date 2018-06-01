use super::led_controller::*;

use serde_json::Value;

///
/// Takes two state functions, and uses the 'right' state function in the event that
/// the 'left' state function indicates no value
/// 
pub fn or_state<LeftStateFn, RightStateFn>(first: LeftStateFn, second: RightStateFn) -> impl FnMut(&Value) -> Option<(u32, LedFade, LedColor)> 
where   LeftStateFn:    FnMut(&Value) -> Option<(u32, LedFade, LedColor)>,
        RightStateFn:   FnMut(&Value) -> Option<(u32, LedFade, LedColor)> {
    let mut first   = first;
    let mut second  = second;

    move |val| {
        if let Some(result) = first(val) {
            Some(result)
        } else {
            second(val)
        }
    }
}