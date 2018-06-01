use futures::*;

use std::fs::File;
use std::io::{Error, Write};

/// Proc file used to control the LED
const LED_PROC_FILE: &str = "/proc/acpi/nuc_led";

///
/// Function that takes a stream of strings to set the LED state to and sends
/// them to the NUC LED proc file
/// 
pub fn update_led_state<Input: Stream<Item=String, Error=Error>>(input: Input) -> impl Stream<Item=(), Error=Error> {
    input.and_then(|led_string| {
        // Buffer the LED string for later writing
        let led_string_buf      = led_string.into_bytes();

        // Write straight to the control file in /proc
        let mut led_proc_file   = File::create(LED_PROC_FILE)?;
        led_proc_file.write_all(&led_string_buf)?;

        Ok(())
    })
}
