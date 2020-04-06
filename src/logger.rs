use clap::ArgMatches;
use fern::{
    colors::{Color, ColoredLevelConfig},
    Dispatch,
};
use log::LevelFilter;
use log::{error, trace};

/**
 * Initializes the logging system using the log crate and fern.
 * 
 * If this looks overkill for a CLI like this it's because it is.
 * Just a demo showcasing possible best pratices.
 */
pub fn initialize<'a>(args: &ArgMatches<'a>) {
    // Clap is great. Period.
    let lvl = match args.occurrences_of("v") {
        1 => LevelFilter::Debug,
        2 => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };
    // This feels very... Out of place like an kludge
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Red)
        .debug(Color::Blue)
        .trace(Color::White)
        .error(Color::BrightRed);
    Dispatch::new()
        // Simple and clever, it's just a callback, do whatever.
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{:<5}] {}",
                colors.color(record.level()),
                message
            ))
        })
        .level(lvl)
        // All the trait magic happening in this method feels a confusing
        .chain(std::io::stdout())
        // Apparently this harmless named method instantiates and
        // replaces the global logger...
        .apply()
        .unwrap();
    if args.occurrences_of("v") > 2 {
        // If this seem out of place beacause it is
        // I need the arguments to initialize the logging system and
        // the logging system to complain that they're wrong...
        error!("Unknown verbosity level!");
    }
    trace!("Log system initialized successfully");
}
