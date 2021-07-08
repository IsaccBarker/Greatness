use crate::manifest::State;
use crate::utils;
use clap::ArgMatches;
use snafu::ResultExt;
use std::path::PathBuf;

pub fn rm(matches: &ArgMatches, manifest: &mut State) -> Result<(), utils::CommonErrors> {
    Ok(()) 
}

