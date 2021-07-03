use std::path::PathBuf;
use std::collections::HashMap;
use snafu::ResultExt;
use rhai::AST;
use super::ScriptsState;
use crate::utils;
use crate::manifest::ManifestData;

impl ScriptsState {
    /// Parse a script before we use it, so we don't need to reparse
    /// everytime we want to run it.
    pub fn parse_script(&mut self, script: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(asts) = &mut self.asts {
            asts.insert(Box::from(script.clone()), self.engine.compile(&std::fs::read_to_string(script).context(utils::FileReadError{file: script})?)?);
        } else {
            let mut tmp: HashMap<Box<PathBuf>, AST> = HashMap::new();
            tmp.insert(Box::from(script.clone()), self.engine.compile(&std::fs::read_to_string(script).context(utils::FileReadError{file: script})?)?);
            self.asts.replace(tmp);
        }

        Ok(())
    }

    /// Parse every script.
    pub fn parse_all_scripts(&mut self, manifest_data: &mut ManifestData) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(files) = &mut manifest_data.files {
            for file in files {
                if let Some(scripts) = &mut file.scripts {
                    for script in scripts {
                        self.parse_script(&utils::special_to_absolute(script))?;
                    }
                }
            }
        }

        Ok(())
    }
}
