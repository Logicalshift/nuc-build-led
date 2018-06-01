use futures::*;
use futures::future;
use tokio::io;
use tokio_uds::*;
use tokio_core::reactor::Handle;

use serde_json;
use std::io::{Error, ErrorKind};

///
/// Creates a unix-domain socket that reads JSON messages
/// 
pub fn create_json_unix_socket(name: &str, handle: &Handle) -> impl Stream<Item=serde_json::Value, Error=Error> {
    // Bind a listener to this socket
    let socket = UnixListener::bind(name, handle).unwrap();

    // Listen for connections and accept JSON data
    socket.incoming()
        .and_then(|(stream, _socket_addr)| {
            io::read_to_end(stream, vec![])
                .and_then(|(_stream, buf)| 
                    future::result(serde_json::from_reader(&*buf)
                        .map_err(|e| Error::new(ErrorKind::Other, e))))
        })
}
