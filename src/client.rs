use log::*;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use std::sync::mpsc::Sender;
use std::thread;
use uuid::Uuid;

mod command_parser;

pub use command_parser::ClientCommand;

pub struct ServerCommand {
    pub command: ClientCommand,
    pub client_id: String,
}

pub struct Client {
    join_handle: Option<thread::JoinHandle<()>>,
    pub id: String,
    name: Option<String>,
}

pub enum ClientError {
    CouldNotAccept(std::io::Error),
}

impl Client {
    pub fn new(stream: TcpStream, sender: Sender<ServerCommand>) -> Result<Client, ClientError> {
        let address = stream.peer_addr();

        let address = match address {
            Result::Ok(address) => address,
            Result::Err(e) => {
                return Err(ClientError::CouldNotAccept(e));
            }
        };

        let mut client = Client {
            join_handle: None,
            id: Uuid::new_v4().to_hyphenated().to_string(),
            name: None,
        };

        debug!(
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
                match command_from_reader(&mut reader) {
                    CommandResult::Disconnected => {
                        debug!(
                            "Socket disconnect detected from {}, killing thread.",
                            address.ip()
                        );
                        break;
                    }
                    CommandResult::Received(command) => {
                        debug!("{} sent text {}", address, command.trim());

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
        debug!("Client with ID {} set nick to {}", self.id, name);

        self.name = Some(name);
    }
}

enum CommandResult {
    Received(String),
    Disconnected,
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
