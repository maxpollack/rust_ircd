use crate::server::ServerMessage;
use log::*;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::net::TcpStream;
use std::sync::mpsc::Sender;
use std::thread;
use uuid::Uuid;

mod command_parser;

pub use command_parser::Message;

pub struct Client {
    join_handle: Option<thread::JoinHandle<()>>,
    pub id: String,
    nick: Option<String>,
    writer: BufWriter<TcpStream>,
}

impl Client {
    pub fn new(stream: TcpStream, sender: Sender<ServerMessage>) -> Result<Client, std::io::Error> {
        let address = stream.peer_addr()?;
        let writer = BufWriter::new(stream.try_clone()?);

        let mut client = Client {
            join_handle: None,
            id: Uuid::new_v4().to_hyphenated().to_string(),
            nick: None,
            writer,
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
        sender: Sender<ServerMessage>,
    ) {
        let client_id = self.id.clone();

        let join_handle = thread::spawn(move || {
            let mut reader = BufReader::new(stream);
            loop {
                match Client::command_from_reader(&mut reader) {
                    Err(_) => {
                        info!(
                            "Socket disconnect detected from {}, killing thread.",
                            address.ip()
                        );

                        let disconnect_command = ServerMessage {
                            command: Message::Disconnect,
                            client_id: client_id.clone(),
                        };

                        sender.send(disconnect_command).unwrap();

                        break;
                    }
                    Ok(command) => {
                        let command = ServerMessage {
                            command: Message::parse(&command),
                            client_id: client_id.clone(),
                        };

                        sender.send(command).unwrap();
                    }
                }
            }
        });

        self.join_handle = Some(join_handle);
    }

    pub fn set_nick(&mut self, name: String) {
        info!("Client with ID {} set nick to {}", self.id, name);

        self.nick = Some(name);
    }

    fn command_from_reader(reader: &mut BufReader<TcpStream>) -> Result<String, ()> {
        let mut command = String::new();
        let read_result = reader.read_line(&mut command);

        match read_result {
            Ok(length) => {
                // 0 length read means a dead socket.
                if length == 0 {
                    return Err(());
                }

                Ok(command)
            }
            Err(e) => {
                error!("Error on read! {}", e);
                Err(())
            }
        }
    }

    pub fn send_numeric_reply(&mut self, reply_number: &str, reply_arguments: &str) {
        debug!("--> {}", reply_arguments);

        let reply = format!(
            ":{} {} {} {}\r\n",
            "rust_ircd.horse",
            reply_number,
            self.nick.as_ref().unwrap(),
            reply_arguments,
        );

        if let Err(error) = self.send_to_client(&reply) {
            error!("Failed to write to client {}: {}", self.id, error);
        }
    }

    pub fn send_to_client(&mut self, message: &str) -> Result<(), std::io::Error> {
        debug!("-> {}", message);

        self.writer.write(format!("{}\r\n", message).as_bytes())?;
        self.writer.flush()?;

        Ok(())
    }
}
