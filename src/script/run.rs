use std::path::PathBuf;
use snafu::ResultExt;
use super::ScriptsState;
use crate::utils;
use log::debug;

impl ScriptsState {
    /// Run a script on a file. Does not check if the file can
    /// have a a script run on it; that check should be done
    /// elsewhere.
    pub fn script_on_file(&self, file: &PathBuf, script: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Running script {} on file {}....", script.display(), file.display());

        let file_data = std::fs::read_to_string(file).context(utils::FileReadError{file})?;
        let asts = self.asts.as_ref().unwrap();
        let script_ast = asts.get(script).unwrap();

        self.engine.eval_ast(script_ast)?;

        Ok(())
    }
}

