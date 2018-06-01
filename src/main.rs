extern crate serde;
#[macro_use] extern crate serde_json;
extern crate futures;
extern crate tokio;
extern crate tokio_uds;
extern crate tokio_core;

mod led;
mod led_socket;

use self::led_socket::*;

use tokio_core::reactor;
use futures::*;
use std::io::Error;
use futures::future;
use serde_json::Value;

fn main() {
    // Set up tokio
    let mut tokio   = reactor::Core::new().unwrap();
    let handle      = tokio.handle();

    // Create a socket to receive JSON data
    let socket      = create_json_unix_socket("./test.socket", &handle);

    // Test: just print some stuff when the socket receives data
    let write_data  = socket
        .or_else(|err| -> future::Ok<Value, Error> {
            // Display errors and pass on the empty value
            println!("Error: {:?}", err);
            future::ok(json![{}])
        })
        .for_each(|json| {
            println!("{:?}", json);

            Ok(())
        });

    // Run our server
    tokio.run(write_data).unwrap();
}
