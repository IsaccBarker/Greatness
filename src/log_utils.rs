use log::Level;

pub fn get_logging_prefix(record: &log::Record) -> String {
    match record.level() {
        Level::Trace => get_logging_prefix_for_level(log::Level::Trace),
        Level::Debug => get_logging_prefix_for_level(log::Level::Debug),
        Level::Info => get_logging_prefix_for_level(log::Level::Info),
        Level::Warn => get_logging_prefix_for_level(log::Level::Warn),
        Level::Error => get_logging_prefix_for_level(log::Level::Error),
    }
}

pub fn get_logging_prefix_for_level(level: log::Level) -> String {
    match level {
        Level::Trace => "\x1b[0;33m",
        Level::Debug => "\x1b[0;36m",
        Level::Info => "",
        Level::Warn => "\x1b[1;33m",
        Level::Error => "\x1b[0;31m",
    }
    .to_owned()
}
