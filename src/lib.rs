#[cfg(test)] #[macro_use] extern crate log;
use log::{Log, Level, Record, Metadata, SetLoggerError};

extern crate miniquad;

fn log_record(record: &Record) {
    // pick the console.log() variant for the appropriate logging level
    let console_send = match record.level() {
        Level::Error => miniquad::console::error,
        Level::Warn => miniquad::console::warn,
        Level::Info => miniquad::console::info,
        Level::Debug => miniquad::console::log,
        Level::Trace => miniquad::console::debug,
    };

    console_send(&format!("{}", record.args()));
}

static LOGGER: WasmLogger = WasmLogger {};

struct WasmLogger {}

impl Log for WasmLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        log_record(record);
    }

    fn flush(&self) {}
}

/// Initializes the global logger setting `max_log_level` to the given value.
pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)?;
    log::set_max_level(level.to_level_filter());
    Ok(())
}

/// Initializes the global logger with `max_log_level` set to `Level::Debug` (the only supported level for now).
pub fn init() -> Result<(), SetLoggerError> {
    init_with_level(Level::Debug)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), log::SetLoggerError> {
        super::init()?;
        Ok(debug!("test"))
    }
}
