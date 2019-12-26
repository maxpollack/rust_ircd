use crate::client::Client;
use server::Server;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::thread;

pub use server::ServerCommand;

mod server;

pub struct ServerThread {
    join_handle: Option<thread::JoinHandle<()>>,
    server: Arc<Server>,
}

impl ServerThread {
    pub fn new(receiver: Receiver<ServerCommand>) -> ServerThread {
        let server = Arc::new(Server::new());

        let thread_server_context = server.clone();
        let mut server = ServerThread {
            join_handle: None,
            server,
        };

        server.join_handle = Some(thread::spawn(move || {
            for command in receiver {
                thread_server_context.handle_client_command(command);
            }
        }));

        server
    }

    pub fn join_client(&self, client: Client) {
        self.server.add_client(client);
    }
}
