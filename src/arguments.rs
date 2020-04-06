use clap::{App, Arg, ArgMatches, SubCommand};

/**
 * Didn't take much time considering the design of this. Some of the most
 * greener parts of the project so far.
 *
 * An obvious oversight. This is vital to the usage, but needs a lot more
 * work I still need to sit and brainstorm a way to go about this before
 * implementing. I mean, clap is amazing, truly one of the best libraries
 * of it's kind I've ever used and I mean considering every other language
 * I know.
 */
pub fn initialize_args<'a>() -> ArgMatches<'a> {
    // I don't really believe the code needs much comments
    // builder pattern is pretty self-documenting.
    App::new("RSTS")
        .version("0.0.1")
        .author("Samuel H. <samuelhnrq@pm.me>")
        .about("Rust demo todo CLI app")
        .arg(
            // Love the OOP here, just another object with definitions.
            // Could move this to fn arg_config() if this ever gets
            // too big.
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
        // I really like how easy this makes to implement git-like cli
        .subcommand(
            // this will need to be moved to it's own fn ASAP
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
}
