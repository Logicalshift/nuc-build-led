use futures::*;
use tokio::io;
use tokio::fs::file::File;
use std::io::Error;

/// Proc file used to control the LED
const LED_PROC_FILE: &str = "/proc/acpi/nuc_led";

///
/// Function that takes a stream of strings to set the LED state to and sends
/// them to the NUC LED proc file
/// 
pub fn update_led_state<Input: Stream<Item=String, Error=Error>>(input: Input) -> impl Stream<Item=(), Error=Error> {
    input.and_then(|led_string| {
        // Buffer the LED string for later writing
        let led_string_buf = led_string.into_bytes();

        // Every time we get an input, write it to the file
        File::create(LED_PROC_FILE)
            .and_then(move |led_proc_file| io::write_all(led_proc_file, led_string_buf))
            .and_then(|_| Ok(()))
    })
}
