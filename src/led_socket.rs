use futures::*;
use futures::future;
use tokio::io;
use tokio_uds::*;
use serde_json::Value;
use tokio_core::reactor::Handle;

use serde_json::*;
use std::io::{Error, ErrorKind};

/// The filename where we create the UDS socket that we'll use for controlling the LED
// const SOCKET_NAME: &str = "./led-control";

///
/// Creates a unix-domain socket that reads JSON messages
/// 
pub fn create_json_unix_socket(name: &str, handle: &Handle) -> impl Stream<Item=Value, Error=Error> {
    // Bind a listener to this socket
    let socket = UnixListener::bind(name, handle).unwrap();

    // Listen for connections and accept JSON data
    socket.incoming()
        .and_then(|(stream, _socket_addr)| {
            io::read_to_end(stream, vec![])
                .and_then(|(_stream, buf)| 
                    future::result(from_reader(&*buf).map_err(|e| Error::new(ErrorKind::Other, e))))
        })
}
