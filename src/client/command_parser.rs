use log::*;

#[derive(Debug)]
pub enum ClientCommand {
    Nick { name: String },
    None,
}

impl ClientCommand {
    pub fn parse(command: &String) -> ClientCommand {
        let command_parts: Vec<_> = command.split(' ').collect();

        debug!("{}", command);

        // THERE IS SOME SERIOUS REFACTORING TO DO HERE.
        match command_parts[0] {
            "NICK" => ClientCommand::Nick {
                name: String::from(command_parts[1]),
            },
            _ => ClientCommand::None,
        }
    }
}
