use clap::{App, Arg, ArgMatches};

pub fn initialize_args<'a>() -> ArgMatches<'a> {
    App::new("RSTS")
        .version("0.0.1")
        .author("Samuel H. <samuelhnrq@pm.me>")
        .about("Rust demo todo CLI app")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        // .arg(
        //     Arg::with_name("INPUT")
        //         .help("Sets the input file to use")
        //         .required(true)
        //         .index(1),
        // )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        // .subcommand(
        //     SubCommand::with_name("test")
        //         .about("controls testing features")
        //         .version("1.3")
        //         .author("Someone E. <someone_else@other.com>")
        //         .arg(
        //             Arg::with_name("debug")
        //                 .short("d")
        //                 .help("print debug information verbosely"),
        //         ),
        // )
        .get_matches()

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    // let config = matches.value_of("config").unwrap_or("default.conf");
    // println!("Value for config: {}", config);
}
