use log::*;

#[derive(Debug)]
pub enum Message {
    Nick { name: String },
    Disconnect,
    None,
}

impl Message {
    pub fn parse(command: &String) -> Message {
        let command_parts: Vec<_> = command.split(' ').collect();

        debug!("{}", command.trim());

        // THERE IS SOME SERIOUS REFACTORING TO DO HERE.
        match command_parts[0] {
            "NICK" => Message::Nick {
                name: String::from(command_parts[1]),
            },
            _ => Message::None,
        }
    }
}
