use clap::{App, Arg};

fn main() {
    let config = App::new("rust_ircd")
        .author("Max Pollack <max@wh1sk3y.com>")
        .about("A likely buggy irc daemon built entirely in rust.")
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("Picks the port to run the service on")
                .default_value("6667")
                .takes_value(true),
        )
        .get_matches();

    rust_ircd::run(config);
}
