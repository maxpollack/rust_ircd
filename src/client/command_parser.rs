use log::*;

#[derive(Debug)]
pub enum Message {
    Nick { new_nick: String },
    User { username: String, real_name: String },
    Disconnect,
    Ping,
    None,
}

impl Message {
    pub fn parse(command: &String) -> Message {
        let command_parts: Vec<_> = command.trim().split(' ').collect();

        debug!("{}", command.trim());

        // THERE IS SOME SERIOUS REFACTORING TO DO HERE.
        match command_parts[0] {
            "NICK" => Message::Nick {
                new_nick: String::from(command_parts[1]),
            },
            "PING" => Message::Ping,
            _ => Message::None,
        }
    }
}
