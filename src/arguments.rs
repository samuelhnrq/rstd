use clap::{App, Arg, ArgMatches, SubCommand};

pub fn initialize_args<'a>() -> ArgMatches<'a> {
    App::new("RSTS")
        .version("0.0.1")
        .author("Samuel H. <samuelhnrq@pm.me>")
        .about("Rust demo todo CLI app")
        .arg(
            Arg::with_name("config")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("create")
                .about("Creates new task")
                .version("0.1")
                .author("Samuel H. <samosaara@gmail.com>")
                .arg(
                    Arg::with_name("description")
                        .help("Description of the task")
                        .multiple(false)
                        .required(true),
                ),
        )
        .get_matches()

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    // let config = matches.value_of("config").unwrap_or("default.conf");
    // println!("Value for config: {}", config);
}
