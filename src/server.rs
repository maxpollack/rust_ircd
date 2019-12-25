use crate::client::{Client, ClientCommand, ServerCommand};
use log::*;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Server {
    join_handle: thread::JoinHandle<()>,
    server_context: Arc<Mutex<ServerContext>>,
}

struct ServerContext {
    clients: Vec<Client>,
}

impl Server {
    pub fn new(receiver: Receiver<ServerCommand>) -> Server {
        let server_context = Arc::new(Mutex::new(ServerContext {
            clients: Vec::new(),
        }));

        let message_thread_context = server_context.clone();

        let join_handle = thread::spawn(move || {
            for message in receiver {
                Server::handle_client_command(message.command);
            }
        });

        Server {
            join_handle,
            server_context,
        }
    }

    pub fn handle_client_command(command: ClientCommand) {
        match command {
            ClientCommand::Nick { name } => println!("Test!"),
        }
    }

    pub fn join_client(&self, client: Client) {
        let mut lock = self.server_context.lock();
        let context = lock.as_mut().unwrap();

        context.clients.push(client);

        debug!(
            "Client joined to server. Now {} clients connected.",
            context.clients.len()
        );
    }
}
