#[derive(Debug)]
pub enum ClientCommand {
    Nick { name: String },
}

impl ClientCommand {
    pub fn parse(command: &String) -> ClientCommand {
        ClientCommand::Nick {
            name: String::from("Nick"),
        }
    }
}
