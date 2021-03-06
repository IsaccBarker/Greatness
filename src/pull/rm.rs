// use std::path::PathBuf;
use crate::manifest::State;
use crate::utils;
use clap::ArgMatches;
// use snafu::ResultExt;

pub fn repel(matches: &ArgMatches, state: &mut State) -> Result<(), utils::CommonErrors> {
    let to_repel = matches.value_of("name").unwrap();

    if let Some(requires) = &mut state.data.requires {
        requires.retain(|e| {
            e.1.components()
                .last()
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap()
                != to_repel
        });
    }

    state.data.populate_file(&state);

    Ok(())
}
