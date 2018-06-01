extern crate serde;
extern crate serde_json;
extern crate futures;
extern crate tokio;
extern crate tokio_uds;
extern crate tokio_core;

mod led;
mod led_socket;
mod json_socket;

use self::json_socket::*;

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

    // Test: just print some stuff when the socket receives data
    let write_data  = socket
        .map(|json| {
            println!("{:?}", json);
        })
        .or_else(|err| -> future::Ok<(), Error> {
            // Display errors and pass on the empty value
            println!("Error: {:?}", err);
            future::ok(())
        })
        .for_each(|_| Ok(()));

    // Run our server
    tokio.run(write_data).unwrap();
}
