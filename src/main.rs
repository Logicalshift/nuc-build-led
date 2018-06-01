//
//
//

extern crate serde;
extern crate serde_json;
extern crate futures;
extern crate tokio;
extern crate tokio_uds;
extern crate tokio_core;

//
// 
//

mod update_led;
mod json_socket;
mod build_state;
mod led_controller;

//
//
//

use self::update_led::*;
use self::json_socket::*;
use self::build_state::*;
use self::led_controller::*;

use tokio_core::reactor;
use futures::*;
use std::io::Error;
use futures::future;

fn main() {
    // Set up tokio
    let mut tokio   = reactor::Core::new().unwrap();
    let handle      = tokio.handle();

    // Create a socket to receive JSON data
    let socket      = create_json_unix_socket("./test.socket", &handle);

    // Update the LED using the build state
    let led_updates = led_controller(socket, build_state);

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
