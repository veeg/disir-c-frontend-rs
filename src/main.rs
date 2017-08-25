extern crate disir_remote;
extern crate futures;
extern crate tokio_core;
extern crate tokio_tungstenite;
extern crate tungstenite;

use std::env;
use std::io::{Error, ErrorKind};

use futures::stream::Stream;
use futures::Future;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use tungstenite::protocol::Message;
use tokio_tungstenite::accept_async;

use disir_remote::Client;

fn main() {
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8080".to_string());
    let addr = addr.parse().unwrap();

    // Create the event loop and TCP listener we'll accept connections on.
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let socket = TcpListener::bind(&addr, &handle).unwrap();
    println!("Listening on: {}", addr);

    let srv = socket.incoming().for_each(|(stream, addr)| {
        println!("Accepted new TCP connection");

        // We have to clone both of these values, because the `and_then`
        // function bellow constructs a new future, `and_then` requires
        // `FnOnce`, so we construct a move closure to move the
        // environment inside the future (AndThen future may outlive our
        // `for_each` future).
        let handle_inner = handle.clone();

        // Accept the websocket connection
        // This returns a future which is chained with .and_then
        accept_async(stream).and_then(move |ws_stream| {
            println!("New WebSocket connection: {}", addr);

            // TODO: Allocate a disir remote client
            let remote_client = disir_remote::Client::new();

            // Let's split the WebSocket stream, so we can work with the
            // reading and writing halves separately.
            let (sink, stream) = ws_stream.split();

            // Whenever we receive a message from the client,
            // we process the message and calculate a response
            let ws_reader = stream.for_each(move |message: Message| {
                println!("Received a message from {}: {}", addr, message);

                let response = remote_client.process(message.into_data());
                //sink.write(response);
                // TODO: Process request - send the message as a byte array into
                // the disir remote client library - that should in turn
                // return a byte array as a response.
                Ok(())
            });

            // QUESTION: How the hell does this work.
            // Now that we've got futures representing each half of the socket, we
            // use the `select` combinator to wait for either half to be done to
            // tear down the other. Then we spawn off the result.
            let connection = ws_reader.map(|_| ()).map_err(|_| ());

            handle_inner.spawn(connection.then(move |_| {
                println!("Connection {} closed.", addr);
                Ok(())
            }));

            Ok(())
        }).map_err(|e| {
            println!("Error during the websocket handshake occurred: {}", e);
            Error::new(ErrorKind::Other, e)
        })
    });

    // Execute server.
    core.run(srv).unwrap();
}
