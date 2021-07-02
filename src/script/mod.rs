pub mod assign;
pub mod fitted;
pub mod jog;
pub mod marathon;
pub mod rm;

use rhai::{Engine, AST};

/// Contains pre compiled script information
pub struct ScriptsState {
    /// The rhai evaluation engine.
    engine: Engine,
}

impl ScriptsState {
    pub fn new() -> Self {
        Self {
            engine: Engine::new(),
        }
    }
}
