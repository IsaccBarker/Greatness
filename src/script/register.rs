use super::ScriptsState;
use log::{error, info, warn};

impl ScriptsState {
    pub fn register_all(&mut self) {
        self.engine
            .register_fn("info", |msg: &str| {
                info!("\x1b[3m\x1b[1mscript  info: {}", msg);
            })
            .register_fn("warn", |msg: &str| {
                warn!("\x1b[3m\x1b[1mscript  warn: {}", msg);
            })
            .register_fn("error", |msg: &str| {
                error!("\x1b[3m\x1b[1mscript error: {}", msg);
            });
    }
}
