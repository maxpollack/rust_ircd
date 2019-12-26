mod server_thread;

use crate::client::{Client, Message};
use crate::protocol::numeric_responses;
use log::*;
use server_thread::ServerThread;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::RwLock;

pub struct ServerMessage {
    pub command: Message,
    pub client_id: String,
}

type ClientList = HashMap<String, RwLock<Client>>;

pub struct Server {
    clients: RwLock<ClientList>,
}

impl Server {
    pub fn new(receiver: Receiver<ServerMessage>) -> ServerThread {
        let server = Server {
            clients: RwLock::new(ClientList::new()),
        };
        ServerThread::new(receiver, server)
    }
    pub fn handle_client_command(&self, command: ServerMessage) {
        match command.command {
            Message::Nick { new_nick } => self.nick(&command.client_id, new_nick),
            Message::Disconnect => self.remove_client(&command.client_id),
            _ => (),
        }
    }

    pub fn remove_client(&self, client_id: &str) {
        self.with_write_clients(|clients| {
            clients.remove_entry(client_id);

            info!(
                "Client {} removed from server. Now {} clients connected.",
                client_id,
                clients.len()
            );
        });
    }

    fn nick(&self, client_id: &str, nick: String) {
        self.with_write_client(client_id, |client| {
            let welcome_message = format!("Welcome to rust_ircd, {}", &nick);
            client.set_nick(nick);
            client.send_numeric_reply(&numeric_responses::RPL_WELCOME, &welcome_message);
        });
    }

    pub fn add_client(&self, client: Client) {
        self.with_write_clients(|clients| {
            let client_id = &client.id.clone();

            clients.insert(client.id.clone(), RwLock::new(client));

            info!(
                "Client {} joined to server. Now {} clients connected.",
                client_id,
                clients.len()
            );
        });
    }

    fn with_write_clients<F>(&self, f: F)
    where
        F: FnOnce(&mut ClientList),
    {
        let mut lock = self.clients.write();
        let mut clients = lock.as_mut().unwrap();

        f(&mut clients)
    }

    fn with_write_client<F>(&self, client_id: &str, f: F)
    where
        F: FnOnce(&mut Client),
    {
        let clients = self.clients.read().unwrap();
        let mut client_lock = clients.get(client_id).unwrap().write();
        let mut client = client_lock.as_mut().unwrap();

        f(&mut client)
    }
}
