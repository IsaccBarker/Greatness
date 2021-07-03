pub mod assign;
pub mod fitted;
pub mod jog;
pub mod marathon;
pub mod parse;
pub mod register;
pub mod rm;
pub mod run;

use rhai::{Engine, AST};
use std::collections::HashMap;
use std::path::PathBuf;

/// Contains pre compiled script information
pub struct ScriptsState {
    /// The rhai evaluation engine.
    engine: Engine,

    /// All scripts, and their pre-parsed ASTs
    asts: Option<HashMap<Box<PathBuf>, AST>>,
}

impl ScriptsState {
    pub fn new() -> Self {
        Self {
            engine: Engine::new(),
            asts: None,
        }
    }
}
