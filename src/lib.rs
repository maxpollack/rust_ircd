use clap::ArgMatches;
use log::*;
use pretty_env_logger;

pub fn run(config: ArgMatches) {
    pretty_env_logger::init();

    let port = config.value_of("port").unwrap();

    debug!("Loading rust_irc...");
    debug!("Binding to port {}", port);
}
