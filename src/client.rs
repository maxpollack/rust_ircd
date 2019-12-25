use log::*;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;

mod command_parser;

pub use command_parser::ClientCommand;

pub struct Client {
    join_handle: Option<thread::JoinHandle<()>>,
}

pub enum ClientError {
    CouldNotAccept(std::io::Error),
}

impl Client {
    pub fn new(stream: TcpStream, sender: Sender<ClientCommand>) -> Result<Client, ClientError> {
        let address = stream.peer_addr();

        let address = match address {
            Result::Ok(address) => address,
            Result::Err(e) => {
                return Err(ClientError::CouldNotAccept(e));
            }
        };

        debug!("Accepted new connection from {}", address.ip());

        let mut client = Client { join_handle: None };

        client.start(stream, address, sender);

        Ok(client)
    }

    fn start(
        &mut self,
        stream: TcpStream,
        address: std::net::SocketAddr,
        sender: Sender<ClientCommand>,
    ) {
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

                        let command = ClientCommand::parse(&command);

                        sender.send(command).unwrap();
                    }
                }
            }
        });

        self.join_handle = Some(join_handle);
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
