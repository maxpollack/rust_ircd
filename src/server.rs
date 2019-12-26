use crate::client::Client;
use server::Server;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, RwLock};
use std::thread;

pub use server::ServerCommand;

mod server;

pub struct ServerThread {
    join_handle: Option<thread::JoinHandle<()>>,
    server: Arc<RwLock<Server>>,
}

impl ServerThread {
    pub fn new(receiver: Receiver<ServerCommand>) -> ServerThread {
        let server = Arc::new(RwLock::new(Server::new()));

        let message_thread_context = server.clone();
        let mut server = ServerThread {
            join_handle: None,
            server,
        };

        server.join_handle = Some(thread::spawn(move || {
            for command in receiver {
                let mut lock = message_thread_context.read();
                let context = lock.as_mut().unwrap();

                context.handle_client_command(command);
            }
        }));

        server
    }

    pub fn join_client(&self, client: Client) {
        let mut lock = self.server.read();
        let context = lock.as_mut().unwrap();

        context.add_client(client);
    }
}
