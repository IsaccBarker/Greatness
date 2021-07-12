#[macro_use]
extern crate maplit;

#[macro_use]
extern crate lazy_static;

mod add;
mod doctor;
mod git;
mod init;
mod log_utils;
mod manifest;
mod pack;
mod package;
mod progress;
mod prompt;
mod pull;
mod rm;
mod script;
mod status;
mod tag;
mod utils;

use clap::{App, AppSettings, Arg};
use env_logger::{Builder, Target};
use log::LevelFilter;
use log::{error, info};
use manifest::Manifest;
use nix::unistd::Uid;
use std::io::Write;
use std::path::PathBuf;

/// Main
fn main() {
    // Initialize logging
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.format(|buf, record| {
        writeln!(
            buf,
            "{}{}\x1b[0m",
            log_utils::get_logging_prefix(record),
            record.args()
        )
    });
    #[cfg(debug_assertions)]
    builder.filter_level(LevelFilter::Trace);
    #[cfg(not(debug_assertions))]
    builder.filter_level(LevelFilter::Info);

    // CLI Interface (RAS Syndrome)
    let mut default_greatness_dir = std::path::PathBuf::new();
    default_greatness_dir.push(home::home_dir().unwrap());
    default_greatness_dir.push(".greatness");

    let matches = App::new("Achieve Greatness!")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Milo Banks (Isacc Barker) <milobanks@zincsoft.dev>")
        .about("Helps you to achieve greatness!")
        .arg(
            Arg::from("<verbose> -v, --verbose 'Enable verbose mode.'")
                .required(false)
                .takes_value(false)
        )
        .subcommand(
            App::new("init")
                .about("Initializes greatness!")
                .arg(
                    Arg::from("--force 'Force to reinitialize greatness.'")
                        .required(false)
                        .takes_value(false),
                ),
        )
        // TODO: purge (removed all traces of greatness; in short, make your computer not great)
        .subcommand(
            App::new("doctor")
                .about("Finds errors. Not that there are any, this software is great after all!")
        )
        .subcommand(
            App::new("status")
                .about("Prints the status of the configuration.")
        )
        .subcommand(
            App::new("add")
                .about("Adds (a) file(s) to the state.")
                .setting(AppSettings::TrailingVarArg)
                .arg(Arg::from("<files>... 'File(s) to add.'").required(true)),
        )
        .subcommand(
            App::new("rm")
                .about("Removes a file from the state. Does not remove the file itself.")
                .setting(AppSettings::TrailingVarArg)
                .arg(Arg::from("<files>... 'File(s) to remove.'").required(true))
        )
        .subcommand(
            App::new("pull")
                .about("Fetches and merges external states.")
                .subcommand(
                    App::new("add")
                        .about("Fetches and merges an external state.")
                        .arg(
                            Arg::from("<from> 'Where to fetch the external state.'")
                                .required(true)
                                .index(1),
                        )
                        .arg(
                            Arg::from("<only-with-tag> -t, --only-with-tag 'Only merge files with a specific tag.'")
                                .required(false)
                        )
                        .arg(
                            Arg::from("<allow-mods> -d, --allow-mods 'Allow scripts and package installation. Please do not use this argument without trusting the source.'")
                                .required(false)
                                .takes_value(false)
                       )
                        .arg(
                            Arg::from("<as-main> -m, --as-main 'Install the file, overwriting the main configuration.'")
                                .required(false)
                                .takes_value(false)
                        )
                    )
                .subcommand(
                    App::new("rm")
                        .about("Removes an external state.")
                        .arg(
                            Arg::from("<name> 'The name of the external state to remove.'")
                                .required(false)
                                .index(1)
                        )
                )
        )
        .subcommand(
            App::new("tag")
                .about("Tag(s) (a) file(s).")
                .setting(AppSettings::TrailingVarArg)
                .arg(
                    Arg::from("<tag> 'What to tag the file(s) as.'")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::from("<files>... 'File(s) to add.'")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            App::new("git")
                .about("Git utilities. For more indepth commands, use `prompt`.")
                .setting(AppSettings::SubcommandRequired)
                .subcommand(
                    App::new("remote")
                        .about("Set a remote.")
                        .arg(
                            Arg::from("<url> 'The URL of the remote.'")
                                .index(1)
                                .required(true)
                        )
                        .arg(
                            Arg::from("<remote> 'The name of the remote.'")
                                .default_value("origin")
                                .index(2)
                                .required(false)
                        )
                )
                .subcommand(
                    App::new("add")
                        .about("Adds all files to the git repository.")
                )
                .subcommand(
                    App::new("pull")
                        .about("Pull the latest from the git repository.")
                        .arg(
                            Arg::from("<remote> -r, --remote 'Remote to push to.'")
                                .default_value("origin")
                                .required(false)
                        )
                        .arg(
                            Arg::from("<branch> -b, --branch 'Branch to push to.'")
                                .default_value("main")
                                .required(false)
                        )
                        .arg(
                            Arg::from("<allow-mods> -d, --allow-mods 'Allow scripts and package installation. Please do not use this argument without trusting the source.'")
                                .required(false)
                        )

                )
                .subcommand(
                    App::new("push")
                        .about("Push your location configuration to the remote git repository.")
                        .arg(
                            Arg::from("<remote> -r, --remote 'Remote to push to.'")
                                .default_value("origin")
                                .required(false)
                        )
                        .arg(
                            Arg::from("<branch> -b, --branch 'Branch to push to.'")
                                .default_value("main")
                                .required(false)
                        )
                        .arg(
                            Arg::from("<dont-add> -a, --dont-add 'Do not add files before pushing.'")
                                .required(false)
                                .takes_value(false)
                        )

                )
        )
        .subcommand(
            App::new("pack")
                .about("Pack all your dotfiles into a git repository.")
        )
        .subcommand(
            App::new("package")
                .about("Package utilities.")
                .subcommand(
                    App::new("jog")
                        .about("Install all packages.")
                )
                .subcommand(
                    App::new("add")
                        .about("Add a package to install.")
                        .arg(
                            Arg::from("<packages>... 'Packages to mark to install.'")
                                .required(true)
                                .index(1)
                        )
                )
                .subcommand(
                    App::new("rm")
                        .about("Removes a package from what to install.")
                        .arg(
                            Arg::from("<packages>... 'Packages to unmark to install.'")
                                .required(true)
                                .index(1)
                        )
                )
                .subcommand(
                    App::new("overload")
                        .about("Deal with package overloading.")
                        .subcommand(
                            App::new("add")
                                .about("Add an overload to a added package.")
                                .arg(
                                    Arg::from("<manager> 'The manager to overload'")
                                        .index(1)
                                        .required(true)
                                )
                                .arg(
                                    Arg::from("<original> 'The original package to overload'")
                                        .index(2)
                                        .required(true)
                                )
                                .arg(
                                    Arg::from("<overload> 'The overload itself'")
                                        .index(3)
                                        .required(true)
                                )
                        )
                        .subcommand(
                            App::new("rm")
                                .about("Remove a overload from an added package.")
                                .arg(
                                    Arg::from("<overloads>.... 'The overload(s) to remove'")
                                        .index(1)
                                        .required(true)
                                )
                        )
                )
        )
        .subcommand(
            App::new("prompt")
                .about("Change directory into the git repository, with special environment variables set for git.")
                .arg(
                    Arg::from("<no-overwite-ps1> -c, --no-overwrite-ps1 'Dont overwrite the ps1 of your shell.'")
                        .required(false)
                )
        )
        .subcommand(
            App::new("script")
                .setting(AppSettings::SubcommandRequired)
                .about("Deal with Lua scripts.")
                .subcommand(
                    App::new("assign")
                        .about("Assign a Lua script to a added file.")
                        .arg(
                            Arg::from("<file> 'The file to operate on.'")
                        )
                        .arg(
                            Arg::from("<script> 'The script to operate on.'")
                        )
                )
                .subcommand(
                    App::new("rm")
                        .setting(AppSettings::TrailingVarArg)
                        .about("Remove a scripts from a file.")
                        .arg(
                            Arg::from("<file> 'The file to operate on.'")
                                .index(1)
                                .required(true)
                        )
                        .arg(
                            Arg::from("<script> 'The script to operate on.'")
                                .index(2)
                                .required(true)
                        )
                )
                .subcommand(
                    App::new("jog")
                        .about("Run script associated with a file.")
                )
        )
        .get_matches(); // TODO: Push and pull commands?

    if matches.is_present("verbose") {
        builder.filter_level(LevelFilter::Debug);
    }

    builder.init();

    if Uid::effective().is_root() {
        eprintln!(
            "You should not be great as root, or it might track files for the
root user. The feeling might also go to your head, and being root
may just tip you over into a State of catatonia.
If you got a permision error previously, please just change the permisions
on the directory."
        );
        std::process::exit(1);
    }

    // Check if we are initialized
    if !default_greatness_dir.as_path().exists()
        && matches.subcommand_name().unwrap_or("") != "init"
    {
        error!("It looks you haven't initialized yet! Use `greatness init` to initialize. P.S, we found this out by looking through some pretty great binoculars.");

        std::process::exit(1);
    }

    let mut state: manifest::State;
    match manifest::State::new(PathBuf::from(
        default_greatness_dir.as_os_str().to_str().unwrap(),
    )) {
        Ok(m) => state = m,
        Err(e) => {
            error!("An error occured whilst getting the greatness state: {}", e);
            std::process::exit(1);
        }
    }

    if matches.subcommand_name().unwrap_or("") != "init" {
        let state_data: Manifest = match Manifest::populate_from_file(&state) {
            Ok(m) => m,
            Err(e) => {
                error!("An error occured whilst parsing the greatness state: {}", e);
                std::process::exit(1);
            }
        };

        state.data = state_data;
    }

    match matches.subcommand() {
        Some(("status", _status_matches)) => {
            status::print_status(&mut state);
        }

        Some(("doctor", doctor_matches)) => {
            let verdict = doctor::doctor(&state, doctor_matches);
            if verdict.is_none() {
                info!("No errors in your great configuration were found!");

                std::process::exit(0);
            }

            error!("An/some errors in your configuration were found!");
            for err in verdict.unwrap() {
                error!("\t{}", err);
            }

            std::process::exit(1);
        }

        Some(("add", add_matches)) => {
            match add::add_files(
                &matches,
                add_matches
                    .values_of("files")
                    .unwrap()
                    .into_iter()
                    .map(|file| std::path::PathBuf::from(file))
                    .collect(),
                &mut state,
            ) {
                Ok(()) => (),
                Err(e) => {
                    error!("An error occured whilst tracking great file(s): {}", e);

                    std::process::exit(1);
                }
            }
        }

        Some(("rm", rm_matches)) => match rm::rm(rm_matches, &mut state) {
            Ok(()) => (),
            Err(e) => {
                error!(
                    "An error occured whilst removing (untracking) great file(s): {}",
                    e
                );

                std::process::exit(1);
            }
        },

        Some(("init", init_matches)) => {
            if default_greatness_dir.as_path().exists() && !init_matches.is_present("force") {
                error!("It looks like you've already initialized. \x1b[5m\x1b[1m\x1b[3m\x1b[4mReinitializing would overwrite your current configuration.\x1b[0m\x1b[31m\nPlease pass the --force flag to reinitialize.");

                std::process::exit(1);
            }

            match init::init(init_matches, &state) {
                Ok(()) => (),
                Err(e) => {
                    error!("An error occured whilst initialising your local great greatness environment: {}", e);

                    std::process::exit(1);
                }
            }
        }

        Some(("pull", get_matches)) => {
            match get_matches.subcommand() {
                Some(("add", add_matches)) => {
                    match pull::add::clone_and_install_repo(
                        add_matches.value_of("from").unwrap().to_string(),
                        add_matches,
                        &mut state,
                        false, // This is the original project
                    ) {
                        Ok(()) => (),
                        Err(e) => {
                            error!(
                                "An error occured whilst cloning/installing the external state: {}",
                                e
                            );

                            std::process::exit(1);
                        }
                    }
                }

                Some(("rm", rm_matches)) => match pull::rm::repel(rm_matches, &mut state) {
                    Ok(()) => (),
                    Err(e) => {
                        error!("An error occured whilst removing an external state: {}", e);

                        std::process::exit(1);
                    }
                },

                None => {
                    unreachable!();
                }
                _ => {
                    unreachable!();
                }
            }
        }

        Some(("git", git_matches)) => match git_matches.subcommand() {
            Some(("remote", remote_matches)) => {
                match git::remote::set_remote(remote_matches, &mut state) {
                    Ok(()) => (),
                    Err(e) => {
                        error!("An error occured whilst setting the remote: {}", e);

                        std::process::exit(1);
                    }
                }
            }

            Some(("add", add_matches)) => match git::add::add(add_matches, &mut state) {
                Ok(()) => (),
                Err(e) => {
                    error!(
                        "An error occured whilst adding files the the local repository: {}",
                        e
                    );

                    std::process::exit(1);
                }
            },

            Some(("push", push_matches)) => match git::push::push(push_matches, &mut state) {
                Ok(()) => (),
                Err(e) => {
                    error!("An error occured whilst pushing files to the remote: {}", e);

                    std::process::exit(1);
                }
            },

            Some(("pull", pull_matches)) => match git::pull::pull(pull_matches, &mut state) {
                Ok(()) => (),
                Err(e) => {
                    error!("An error occured whilst pulling files to local: {}", e);

                    std::process::exit(1);
                }
            },

            Some(("commit", _)) => {
                error!("Commiting is not yet supported. Please use the prompt subcommand instead!");

                std::process::exit(1);
            }

            _ => {
                unreachable!();
            }
        },

        Some(("pack", pack_matches)) => match pack::pack(&mut state, pack_matches) {
            Ok(()) => (),
            Err(e) => {
                error!(
                    "An error occured whilst packing greatness into a small space: {}",
                    e
                );

                std::process::exit(1);
            }
        },

        Some(("package", package_matches)) => match package_matches.subcommand() {
            Some(("jog", jog_matches)) => match package::jog::jog(jog_matches, &mut state) {
                Ok(()) => (),
                Err(e) => {
                    error!("An error occured whilst installing all packages: {}", e);

                    std::process::exit(1);
                }
            },

            Some(("add", add_matches)) => match package::add::add(add_matches, &mut state) {
                Ok(()) => (),
                Err(e) => {
                    error!("An error occured whilst adding a package to install: {}", e);

                    std::process::exit(1);
                }
            },

            Some(("rm", rm_matches)) => match package::rm::rm(rm_matches, &mut state) {
                Ok(()) => (),
                Err(e) => {
                    error!(
                        "An error occured whilst removing a package to install: {}",
                        e
                    );

                    std::process::exit(1);
                }
            },

            Some(("overload", overload_matches)) => match overload_matches.subcommand() {
                Some(("add", add_matches)) => {
                    match package::overload::add::add(add_matches, &mut state) {
                        Ok(()) => (),
                        Err(e) => {
                            error!("An error occured whilst adding an overload: {}", e);

                            std::process::exit(1);
                        }
                    }
                }

                Some(("rm", rm_matches)) => {
                    match package::overload::rm::rm(rm_matches, &mut state) {
                        Ok(()) => (),
                        Err(e) => {
                            error!("An error occured whilst removing an overload: {}", e);

                            std::process::exit(1);
                        }
                    }
                }

                _ => {
                    unreachable!()
                }
            },

            _ => unreachable!(),
        },

        Some(("tag", tag_matches)) => match tag::tag(tag_matches, &mut state) {
            Ok(()) => (),
            Err(e) => {
                error!("An error occured whilst tagging the file(s): {}", e);

                std::process::exit(1);
            }
        },

        Some(("prompt", prompt_matches)) => match prompt::prompt(prompt_matches, &mut state) {
            Ok(()) => (),
            Err(e) => {
                error!(
                    "An error occured whilst changing directories into the git repository: {}",
                    e
                );

                std::process::exit(1);
            }
        },

        Some(("script", script_matches)) => match script_matches.subcommand() {
            Some(("assign", assign_matches)) => {
                match script::assign::assign(assign_matches, &mut state) {
                    Ok(()) => (),
                    Err(e) => {
                        error!(
                            "An error occured whilst assigning a script to a file: {}",
                            e
                        );

                        std::process::exit(1);
                    }
                }
            }

            Some(("rm", rm_matches)) => match script::rm::rm(rm_matches, &mut state) {
                Ok(()) => (),
                Err(e) => {
                    error!(
                        "An error occured whilst un-tethering a script and a file: {}",
                        e
                    );

                    std::process::exit(1);
                }
            },

            Some(("jog", _jog_matches)) => match script::jog::jog(&mut state) {
                Ok(()) => (),
                Err(e) => {
                    error!("An error occured whilst going jogging: {}", e);

                    std::process::exit(1);
                }
            },

            None => {
                eprintln!("Please use the --help flag to get great knowlage!")
            }
            _ => unreachable!(),
        },

        None => eprintln!("Please use the --help flag to get great knowlage!"),
        _ => unreachable!(),
    }
}
