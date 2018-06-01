use futures::*;
use futures::stream;
use serde_json::Value;
use tokio::timer::Delay;

use std::io::{Error, ErrorKind};
use std::time::{Duration, Instant};

/// Time between each colour
const DELAY_MILLIS: u64 = 150;

/// Time to display the final colour
const FINAL_DELAY: u64  = 700;

///
/// Generates a sequence that cycles through the LED colours before turning off
/// (assuming that `color_state` is in use)
/// 
pub fn rainbow() -> impl Stream<Item=Value, Error=Error> {
    // Colors to display 
    let colors          = vec!["red", "pink", "yellow", "amber", "green", "cyan", "blue", "white" ];
    let num_colors      = colors.len();

    // Work out when we're going to display them
    let start_time      = Instant::now();
    let display_times   = (0..num_colors)
        .into_iter()
        .map(|index|                (index as u64) * DELAY_MILLIS)
        .map(move |offset_millis|   start_time + Duration::from_millis(offset_millis));

    // Turn into an iterator generating the JSON
    let colors          = stream::iter_ok(colors)
        .map(|color| json![{ "rainbow": { "color": color } }])
        .zip(stream::iter_ok(display_times));

    // ... with a delay between each
    let colors          = colors.and_then(|(color, when)|
        Delay::new(when)
            .map_err(|err| Error::new(ErrorKind::Other, err))
            .and_then(move |_| Ok(color)));

    // Finally, display the last colour a bit longer before disabling the colours entirely
    let start_time      = Instant::now();
    let final_time      = (num_colors as u64) * DELAY_MILLIS + FINAL_DELAY;
    let final_time      = start_time + Duration::from_millis(final_time);
    let final_delay     = Delay::new(final_time)
        .map_err(|err| Error::new(ErrorKind::Other, err))
        .and_then(|_| Ok(json![{ "rainbow": { } }]));
    
    // Result is the colour sequence followed by the final delay
    colors.chain(final_delay.into_stream())
}
