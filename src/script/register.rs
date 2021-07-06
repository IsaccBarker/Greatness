use super::ScriptsState;
use log::{error, info, warn};

impl ScriptsState {
    pub fn register_all(&mut self) {
        self.engine.context(|lua_ctx| {
            let globals = lua_ctx.globals();

            globals.set("info", lua_ctx.create_function(|_, msg: String| {
                info!("\x1b[3m\x1b[1mscript  info: {}", msg);

                Ok(())
            }).unwrap()).unwrap();
            
            globals.set("warn", lua_ctx.create_function(|_, msg: String| {
                warn!("\x1b[3m\x1b[1mscript  warn: {}", msg);

                Ok(())
            }).unwrap()).unwrap();
            
            globals.set("error", lua_ctx.create_function(|_, msg: String| {
                error!("\x1b[3m\x1b[1mscript error: {}", msg);

                Ok(())
            }).unwrap()).unwrap();
        });
    }
}
