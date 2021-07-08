pub mod assign;
pub mod jog;
pub mod register;
pub mod rm;
pub mod run;

use rlua::Lua;

/// Contains pre compiled script information
pub struct ScriptsState {
    /// The rhai evaluation engine.
    engine: Lua,
}

impl ScriptsState {
    pub fn new() -> Self {
        Self { engine: Lua::new() }
    }
}
