use log::{info, warn, error};
use super::ScriptsState;

pub fn script_info(msg: &str) {
    info!("\x1b[3m\x1b[1mscript  info: {}", msg);
}

pub fn script_warn(msg: &str) {
    warn!("\x1b[3m\x1b[1mscript  warn: {}", msg);
}

pub fn script_error(msg: &str) {
    error!("\x1b[3m\x1b[1mscript error: {}", msg);
}

impl ScriptsState {
    pub fn register_all(&mut self) {
        self.engine.register_fn("info", script_info)
            .register_fn("warn", script_warn)
            .register_fn("error", script_error);
    }
}

