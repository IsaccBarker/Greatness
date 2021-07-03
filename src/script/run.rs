use super::ScriptsState;
use crate::utils;
use log::debug;
use rhai::{Dynamic, Scope};
use snafu::{ResultExt, Snafu};
use std::path::PathBuf;

#[derive(Debug, Snafu)]
pub enum ScriptRunErrors {
    #[snafu(display("Function not found for {}. Is the signature correct? Please check the documentation for the expected signatures of functions called by greatness! Raw error: {}", name, source))]
    FunctionNotFound {
        name: String,
        source: rhai::EvalAltResult,
    },
}

impl ScriptsState {
    /// Run a script on a file. Does not check if the file can
    /// have a a script run on it; that check should be done
    /// elsewhere.
    pub fn script_on_file(
        &self,
        file: &PathBuf,
        script: &PathBuf,
    ) -> Result<String, Box<dyn std::error::Error>> {
        debug!(
            "Running script {} on file {}....",
            script.display(),
            file.display()
        );

        let mut scope = Scope::new();
        let data = std::fs::read_to_string(file).context(utils::FileReadError { file })?;
        let asts = self.asts.as_ref().unwrap();
        let script_ast = asts.get(script).unwrap();

        let result: Result<String, Box<rhai::EvalAltResult>> = self.engine.call_fn(
            &mut scope,
            script_ast,
            "process",
            (data, file.clone().as_os_str().to_str().unwrap().to_string()),
        );
        if result.is_err() {
            Err(*result.err().unwrap()).context(FunctionNotFound { name: "process" })?;

            unreachable!();
        }

        Ok(result.unwrap())
    }
}
