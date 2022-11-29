use fern::colors::Color;
use log::SetLoggerError;

pub(crate) fn setup_logging(loglevel: log::LevelFilter) -> Result<(), SetLoggerError> {
    let color_text = fern::colors::ColoredLevelConfig::new()
        .debug(Color::BrightBlack)
        .error(Color::White)
        .trace(Color::BrightBlue);

    let color_symbol = fern::colors::ColoredLevelConfig::new()
        .debug(Color::BrightBlack)
        .error(Color::Red)
        .error(Color::Red)
        .info(Color::Green)
        .trace(Color::BrightBlue)
        .warn(Color::Yellow);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{color_symbol}{level}\x1B[0m] {color_line}{message}\x1B[0m",
                color_symbol = format_args!(
                    "\x1B[{}m",
                    color_symbol.get_color(&record.level()).to_fg_str()
                ),
                color_line = format_args!(
                    "\x1B[{}m",
                    color_text.get_color(&record.level()).to_fg_str()
                ),
                message = message,
                level = record.level(),
            ));
        })
        .level(loglevel)
        .chain(std::io::stdout())
        .apply()
}
