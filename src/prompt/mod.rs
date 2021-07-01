use clap::ArgMatches;
use crate::manifest::Manifest;
use log::info;
use crate::utils;

// const PRELUDE_PS1: &str = r#"\e[1;30mgreatness ($(whoami) at $(pwd)) >> \e[0m"#;
const PRELUDE_PS1: &str = "greatness (git prompt) > ";

/// Changes directory into the git directory
pub fn prompt(_matches: &ArgMatches, manifest: &Manifest) -> Result<(), utils::CommonErrors> {
    let shell_to_use = std::env::var("SHELL").unwrap();
    info!("You are now in a great child shell. Type `exit` to return to your great previous shell!");

    subprocess::Exec::cmd(shell_to_use)
        // .stdout(subprocess::Redirection::Pipe)
        .arg("-f") // Do not use a RC
        //.arg(r#"-c """#)
        .cwd(&manifest.greatness_git_pack_dir)
        .env("PS1", PRELUDE_PS1)
        .popen()
        .unwrap();

    Ok(())
} 
 
