use crate::server::ServerCommand;
use log::*;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use std::sync::mpsc::Sender;
use std::thread;
use uuid::Uuid;

mod command_parser;

pub use command_parser::ClientCommand;

enum CommandResult {
    Received(String),
    Disconnected,
}

pub struct Client {
    join_handle: Option<thread::JoinHandle<()>>,
    pub id: String,
    name: Option<String>,
}

impl Client {
    pub fn new(stream: TcpStream, sender: Sender<ServerCommand>) -> Result<Client, std::io::Error> {
        let address = stream.peer_addr()?;

        let mut client = Client {
            join_handle: None,
            id: Uuid::new_v4().to_hyphenated().to_string(),
            name: None,
        };

        info!(
            "Accepted new connection from {}, client id {}",
            address.ip(),
            client.id
        );

        client.start(stream, address, sender);

        Ok(client)
    }

    fn start(
        &mut self,
        stream: TcpStream,
        address: std::net::SocketAddr,
        sender: Sender<ServerCommand>,
    ) {
        let client_id = self.id.clone();

        let join_handle = thread::spawn(move || {
            let mut reader = BufReader::new(stream);
            loop {
                match Client::command_from_reader(&mut reader) {
                    CommandResult::Disconnected => {
                        info!(
                            "Socket disconnect detected from {}, killing thread.",
                            address.ip()
                        );

                        let disconnect_command = ServerCommand {
                            command: ClientCommand::Disconnect,
                            client_id: client_id.clone(),
                        };

                        sender.send(disconnect_command).unwrap();

                        break;
                    }
                    CommandResult::Received(command) => {
                        let command = ServerCommand {
                            command: ClientCommand::parse(&command),
                            client_id: client_id.clone(),
                        };

                        sender.send(command).unwrap();
                    }
                }
            }
        });

        self.join_handle = Some(join_handle);
    }

    pub fn set_name(&mut self, name: String) {
        info!("Client with ID {} set nick to {}", self.id, name);

        self.name = Some(name);
    }

    fn command_from_reader(reader: &mut BufReader<TcpStream>) -> CommandResult {
        let mut command = String::new();
        let read_result = reader.read_line(&mut command);

        match read_result {
            Ok(length) => {
                // 0 length read means a dead socket.
                if length == 0 {
                    return CommandResult::Disconnected;
                }

                CommandResult::Received(command)
            }
            Err(e) => {
                error!("Error on read! {}", e);
                CommandResult::Disconnected
            }
        }
    }
}
