mod client;
mod protocol;
mod server;

use clap::{value_t, ArgMatches};
use log::*;
use pretty_env_logger;
use server::ServerMessage;
use std::io;
use std::net::TcpListener;
use std::process;
use std::sync::mpsc::channel;

pub fn run(config: ArgMatches) {
    pretty_env_logger::init();

    info!("Initializing rust_irc...");

    let port = value_t!(config.value_of("port"), u32);
    let port = port.unwrap_or_else(|_| {
        error!("Failed to parse port! Port value must be a valid number!");
        process::exit(1);
    });

    if let Err(e) = serve(port) {
        error!("Failed to bind to address! {}", e);
    }
}

fn serve(port: u32) -> io::Result<()> {
    info!("Binding to port {}", port);

    let bind_address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(bind_address)?;

    info!("Successfully bound to port {}", port);

    let (tx, rx) = channel::<ServerMessage>();

    let server = server::Server::new(rx);

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let client = client::Client::new(stream, tx.clone());

            if let Ok(client) = client {
                server.join_client(client);
            } else {
                error!("Failed to initialize client for incoming connection.");
            }
        } else {
            error!("Error retreiving incoming connection stream!");
        }
    }

    Ok(())
}
