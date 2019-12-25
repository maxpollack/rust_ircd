use log::*;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use std::thread;

pub struct Client {
    join_handle: thread::JoinHandle<()>,
}

pub enum ClientError {
    CouldNotAccept(std::io::Error),
}

impl Client {
    pub fn new(stream: TcpStream) -> Result<Client, ClientError> {
        let address = stream.peer_addr();

        let address = match address {
            Result::Ok(address) => address,
            Result::Err(e) => {
                return Err(ClientError::CouldNotAccept(e));
            }
        };

        debug!("Accepted new connection from {}", address.ip());

        let mut reader = BufReader::new(stream);

        let join_handle = thread::spawn(move || {
            loop {
                let mut command = String::new();
                let read_result = reader.read_line(&mut command);

                match read_result {
                    Ok(length) => {
                        // 0 length read means a dead socket.
                        if length == 0 {
                            break;
                        }

                        debug!("Read command: {}", command.trim());
                    }
                    Err(e) => {
                        error!("Error on read! {}", e);
                    }
                }
            }

            debug!(
                "Socket disconnect detected from {}, killing thread.",
                address.ip()
            );
        });

        Ok(Client { join_handle })
    }
}
