use clap::ArgMatches;
use fern::{
    colors::{Color, ColoredLevelConfig},
    Dispatch,
};
use log::LevelFilter;
use log::{error, trace};

pub fn initialize<'a>(args: &ArgMatches<'a>) {
    let lvl = match args.occurrences_of("v") {
        1 => LevelFilter::Debug,
        2 => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Red)
        .debug(Color::Blue)
        .trace(Color::White)
        .error(Color::BrightRed);
    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{:<5}] {}",
                colors.color(record.level()),
                message
            ))
        })
        .level(lvl)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
    if args.occurrences_of("v") > 2 {
        error!("Unknown verbosity level!");
    }
    trace!("Log system initialized successfully");
}
