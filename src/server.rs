use crate::client::{Client, ClientCommand, ServerCommand};
use log::*;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Server {
    join_handle: Option<thread::JoinHandle<()>>,
    server_context: Arc<Mutex<ServerContext>>,
}

struct ServerContext {
    clients: HashMap<String, Client>,
}

impl Server {
    pub fn new(receiver: Receiver<ServerCommand>) -> Server {
        let server_context = Arc::new(Mutex::new(ServerContext {
            clients: HashMap::new(),
        }));

        let message_thread_context = server_context.clone();
        let mut server = Server {
            join_handle: None,
            server_context,
        };

        server.join_handle = Some(thread::spawn(move || {
            for command in receiver {
                let mut lock = message_thread_context.lock();
                let context = lock.as_mut().unwrap();

                context.handle_client_command(command);
            }
        }));

        server
    }

    pub fn join_client(&self, client: Client) {
        let mut lock = self.server_context.lock();
        let context = lock.as_mut().unwrap();

        context.clients.insert(client.id.clone(), client);

        debug!(
            "Client joined to server. Now {} clients connected.",
            context.clients.len()
        );
    }
}

impl ServerContext {
    pub fn handle_client_command(&mut self, command: ServerCommand) {
        match command.command {
            ClientCommand::Nick { name } => self.handle_nick_change(&command.client_id, name),
            _ => (),
        }
    }

    pub fn handle_nick_change(&mut self, client_id: &str, name: String) {
        let client = self.clients.get_mut(client_id).unwrap();

        client.set_name(name);
    }
}
