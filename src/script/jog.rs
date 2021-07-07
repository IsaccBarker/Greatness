use crate::manifest::State;
use crate::utils;
use snafu::ResultExt;
use std::fs::File;
use std::io::Write;
use log::debug;

pub fn jog(manifest: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(files) = &manifest.data.files {
        for file in files {
            if let Some(scripts) = &file.scripts {
                for script in scripts {
                    let abs = utils::special_to_absolute(&file.path);
                    let processed = manifest
                        .script_state
                        .script_on_file(&abs, &utils::special_to_absolute(script))?;

                    debug!("Writting processed file:\n\n{}", processed);
                    debug!("\nEnd of writting processed file!");

                    File::create(&abs)
                        .context(utils::FileOpenError { file: &abs })?
                        .write_all(processed.as_bytes())
                        .context(utils::FileWriteError { file: &abs })?;
                }
            }
        }
    }

    Ok(())
}
