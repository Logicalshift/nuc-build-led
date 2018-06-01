//
//
//

extern crate serde;
#[macro_use] extern crate serde_json;
extern crate futures;
extern crate tokio;
extern crate tokio_uds;
extern crate tokio_core;

//
// 
//

mod rainbow;
mod or_state;
mod update_led;
mod json_socket;
mod build_state;
mod color_state;
mod led_controller;

//
//
//

use self::rainbow::*;
use self::or_state::*;
use self::update_led::*;
use self::json_socket::*;
use self::build_state::*;
use self::color_state::*;
use self::led_controller::*;

use tokio_core::reactor;
use futures::*;
use futures::future;
use futures::stream;
use serde_json::Value;
use std::io::Error;
use std::path::Path;
use std::fs;
use std::os::unix::fs::*;

const SOCKET_PATH: &str = "/var/run/nuc-led/control";

fn main() {
    // Set up tokio
    let mut tokio   = reactor::Core::new().unwrap();
    let handle      = tokio.handle();

    // Create a socket to receive JSON data
    if Path::new(SOCKET_PATH).exists() {
        fs::remove_file(SOCKET_PATH).unwrap();
    }
    let socket      = create_json_unix_socket(SOCKET_PATH, &handle);

    fs::set_permissions(SOCKET_PATH, fs::Permissions::from_mode(0b111_111_000)).unwrap();
    
    // Show a rainbow startup sequence
    let rainbow     = rainbow();
    let socket      = socket.select(rainbow);

    // Supply a 'null' value initially to reset the LED
    let socket      = stream::iter_ok(vec![Value::Null].into_iter()).chain(socket);

    // Create the LED state function (build state first, color state second)
    let led_state   = or_state(build_state, color_state);

    // Update the LED using the build state
    let led_updates = led_controller(socket, led_state);

    // Send to the LED
    let led_updates = update_led_state(led_updates);

    // Main loop: just print errors and carry on
    let write_data  = led_updates
        .or_else(|err| -> future::Ok<(), Error> {
            // Display errors and pass on the empty value
            println!("Error: {:?}", err);
            future::ok(())
        })
        .for_each(|_| Ok(()));

    // Run our server
    tokio.run(write_data).unwrap();
}
