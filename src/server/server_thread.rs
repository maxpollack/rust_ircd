use crate::client::Client;
use crate::server::{Server, ServerMessage};
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::thread::{self, JoinHandle};

pub struct ServerThread {
    join_handle: Option<JoinHandle<()>>,
    server: Arc<Server>,
}

impl ServerThread {
    pub fn new(receiver: Receiver<ServerMessage>, server: Server) -> ServerThread {
        let server = Arc::new(server);

        let thread_server_reference = server.clone();
        let mut server = ServerThread {
            join_handle: None,
            server,
        };

        server.join_handle = Some(thread::spawn(move || {
            for command in receiver {
                thread_server_reference.handle_client_command(command);
            }
        }));

        server
    }

    pub fn join_client(&self, client: Client) {
        self.server.add_client(client);
    }
}
