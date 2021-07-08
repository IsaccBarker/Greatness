use super::ScriptsState;
use crate::utils;
use log::debug;
use rlua::Function;
use snafu::{ResultExt, Snafu};
use std::path::PathBuf;

#[derive(Debug, Snafu)]
pub enum ScriptRunErrors {
    #[snafu(display("Failed to load lua code in script {}. Rlua Error:\n{}", file.display(), source))]
    LuaLoadError { file: PathBuf, source: rlua::Error },

    #[snafu(display("Function not found for {} in file {}. Is the signature correct? Please check the documentation for the expected signatures of functions called by greatness! Rlua error:\n{}", name, file.display(), source))]
    FunctionNotFound {
        name: String,
        file: PathBuf,
        source: rlua::Error,
    },

    #[snafu(display("An error occured while executing file {}! Rlua error:\n{}", file.display(), source))]
    RuntimeError { file: PathBuf, source: rlua::Error },
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

        let data = std::fs::read_to_string(file).context(utils::FileReadError { file })?;
        let script_src =
            std::fs::read_to_string(script).context(utils::FileReadError { file: script })?;

        let output_res: Result<String, ScriptRunErrors> =
            self.engine.context(|lua_ctx: rlua::prelude::LuaContext| {
                let globals = lua_ctx.globals();
                let process;

                lua_ctx
                    .load(&script_src)
                    .exec()
                    .context(LuaLoadError { file: script })?;

                process = globals
                    .get::<_, Function>("process")
                    .context(FunctionNotFound {
                        name: "process".to_owned(),
                        file: script,
                    })?;
                Ok(process
                    .call::<(String, String), String>((
                        data,
                        file.clone().as_os_str().to_str().unwrap().to_string(),
                    ))
                    .context(RuntimeError { file: script })?)
            });
        let output = output_res?;

        Ok(output)
    }
}
