extern crate serde;
extern crate serde_json;
extern crate futures;
extern crate tokio;
extern crate tokio_uds;
extern crate tokio_core;

mod led;
mod led_socket;

use self::led_socket::*;

use tokio_core::reactor;
use futures::*;

fn main() {
    // Set up tokio
    let mut tokio   = reactor::Core::new().unwrap();
    let handle      = tokio.handle();

    // Create a socket to receive JSON data
    let socket      = create_json_unix_socket("./test.socket", &handle);

    // Test: just print some stuff when the socket receives data
    let write_data  = socket
        .for_each(|json| {
            println!("{:?}", json);

            Ok(())
        })
        .map_err(|err| {
            println!("Error: {:?}", err);
        });

    // Run our server
    tokio.run(write_data).unwrap();
}
