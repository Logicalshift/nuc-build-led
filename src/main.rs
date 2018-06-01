extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate futures;
extern crate tokio;
extern crate tokio_uds;
extern crate tokio_core;

mod led;
mod led_socket;

fn main() {
    println!("Hello, world!");
}
