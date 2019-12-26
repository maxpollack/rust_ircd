use crate::client::{Client, ClientCommand};
use log::*;
use std::collections::HashMap;

pub struct ServerCommand {
    pub command: ClientCommand,
    pub client_id: String,
}

pub struct Server {
    clients: HashMap<String, Client>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            clients: HashMap::new(),
        }
    }
    pub fn handle_client_command(&mut self, command: ServerCommand) {
        match command.command {
            ClientCommand::Nick { name } => self.handle_nick_change(&command.client_id, name),
            ClientCommand::Disconnect => self.remove_client(&command.client_id),
            _ => (),
        }
    }

    pub fn remove_client(&mut self, client_id: &str) {
        self.clients.remove_entry(client_id);

        info!(
            "Client {} removed from server. Now {} clients connected.",
            client_id,
            self.clients.len()
        );
    }

    pub fn handle_nick_change(&mut self, client_id: &str, name: String) {
        let client = self.clients.get_mut(client_id).unwrap();

        client.set_name(name);
    }

    pub fn add_client(&mut self, client: Client) {
        self.clients.insert(client.id.clone(), client);

        info!(
            "Client joined to server. Now {} clients connected.",
            self.clients.len()
        );
    }
}
