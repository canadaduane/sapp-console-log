#[cfg(test)] #[macro_use] extern crate log;
use log::{Log, Level, Record, Metadata, SetLoggerError};

#[cfg(target_arch = "wasm32")]
extern crate sapp_wasm as sapp;

#[cfg(not(target_arch = "wasm32"))]
pub mod console {
    pub fn debug(_msg: &str) {
        panic!("console::debug only implemented for wasm32 target");
    }
    
    pub fn log(_msg: &str) {
        panic!("console::log only implemented for wasm32 target");
    }

    pub fn info(_msg: &str) {
        panic!("console::info only implemented for wasm32 target");
    }

    pub fn warn(_msg: &str) {
        panic!("console::warn only implemented for wasm32 target");
    }
    
    pub fn error(_msg: &str) {
        panic!("console::error only implemented for wasm32 target");
    }
}

#[cfg(target_arch = "wasm32")]
pub mod console {
    pub fn debug(msg: &str) {
        use std::ffi::CString;
        let string = CString::new(msg).unwrap();
        unsafe { sapp::console_debug(string.as_ptr()); }
    }
    
    pub fn log(msg: &str) {
        use std::ffi::CString;
        let string = CString::new(msg).unwrap();
        unsafe { sapp::console_log(string.as_ptr()); }
    }

    pub fn info(msg: &str) {
        use std::ffi::CString;
        let string = CString::new(msg).unwrap();
        unsafe { sapp::console_info(string.as_ptr()); }
    }

    pub fn warn(msg: &str) {
        use std::ffi::CString;
        let string = CString::new(msg).unwrap();
        unsafe { sapp::console_warn(string.as_ptr()); }
    }
    
    pub fn error(msg: &str) {
        use std::ffi::CString;
        let string = CString::new(msg).unwrap();
        unsafe { sapp::console_error(string.as_ptr()); }
    }
}

fn log_record(record: &Record) {
    // pick the console.log() variant for the appropriate logging level
    let console_send = match record.level() {
        Level::Error => console::error,
        Level::Warn => console::warn,
        Level::Info => console::info,
        Level::Debug => console::log,
        Level::Trace => console::debug,
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
