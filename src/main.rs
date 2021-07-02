mod add;
mod doctor;
mod init;
mod log_utils;
mod manifest;
mod pack;
mod progress;
mod prompt;
mod pull;
mod repel;
mod rm;
mod script;
mod status;
mod tag;
mod utils;

use clap::{App, AppSettings, Arg};
use env_logger::{Builder, Target};
use log::LevelFilter;
use log::{error, info};
use manifest::ManifestData;
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

    builder.init();

    // CLI Interface (RAS Syndrome)
    let mut default_greatness_dir = std::path::PathBuf::new();
    default_greatness_dir.push(home::home_dir().unwrap());
    default_greatness_dir.push(".greatness");

    let matches = App::new("Achieve Greatness!")
        .version("0.1.0")
        .author("Milo Banks (Isacc Barker) <milobanks@zincsoft.dev>")
        .about("Helps you to achieve greatness!")
        .arg(
            Arg::from("<ignore-root-check> --ignore-root-check 'Allow to run as root.'")
                .required(false),
        )
        .subcommand(
            App::new("init")
                .about("Initializes greatness!")
                .version("0.1.0")
                .author("Milo Banks (Isacc Barker) <milobanks@zincsoft.dev>")
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
                .version("0.1.0")
                .author("Milo Banks (Isacc Barker) <milobanks@zincsoft.dev>"),
        )
        .subcommand(
            App::new("status")
                .about("Prints the status of the configuration.")
                .author("Milo Banks (Isacc Barker) <milobanks@zincsoft.dev>")
                .arg(
                    Arg::from("<file> 'A specific great file to get the status of.'")
                        .required(false)
                        .index(1)
                ),
        )
        .subcommand(
            App::new("add")
                .about("Adds (a) file(s) to the manifest.")
                .version("0.1.0")
                .author("Milo Banks (Isacc Barker) <milobanks@zincsoft.dev>")
                .setting(AppSettings::TrailingVarArg)
                .arg(Arg::from("<files>... 'File(s) to add.'").required(true)),
        )
        .subcommand(
            App::new("rm")
                .about("Removes a file from the manifest. Does not remove the file itself.")
                .version("0.1.0")
                .author("Milo Banks (Isacc Barker) <milobanks@zincsoft.dev>")
                .setting(AppSettings::TrailingVarArg)
                .arg(Arg::from("<files>... 'File(s) to remove.'").required(true))
        )
        .subcommand(
            App::new("pull")
                .about("Fetches and merges external manifests.")
                .version("0.1.0")
                .author("Milo Banks (Isacc Barker) <milobanks@zincsoft.dev>")
                .arg(
                    Arg::from("<from> 'Where to fetch the external manifest.'")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::from("<only-with-tag> -t, --only-with-tag 'Only merge files with a specific tag.'")
                        .required(false)
                )
                .arg(
                    Arg::from("<as-main> -m, --as-main 'Install the file, overwriting the main configuration.'")
                        .required(false)
                ),
        )
        .subcommand(
            App::new("repel")
                .about("Unmerges and deleted an external manifest. Takes the name of the manifest, not the url.")
                .version("0.1.0")
                .arg(
                    Arg::from("<name> 'Name of the external manifest to repel.'")
                        .required(true)
                        .index(1)
                )
        )
        .subcommand(
            App::new("tag")
                .about("Tag(s) (a) file(s)")
                .version("0.1.0")
                .author("Milo Banks (Isacc Barker) <milobanks@zincsoft.dev>")
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
            App::new("pack")
                .about("Pack all your dotfiles into a git repository.")
                .version("0.1.0")
                .author("Milo Banks (Isacc Barker) <milobanks@zincosft.dev>")
        )
        .subcommand(
            App::new("prompt")
                .about("Change directory into the git repository, with special environment variables set for git.")
                .version("0.1.0")
                .author("Milo Banks (Isacc Barker) <milobanks@zincsoft.dev>")
                .arg(
                    Arg::from("<no-overwite-ps1> -c, --no-overwrite-ps1 'Dont overwrite the ps1 of your shell.'")
                        .required(false)
                )
        )
        .subcommand(
            App::new("script")
                .setting(AppSettings::SubcommandRequired)
                .about("Deal with Rhai scripts.")
                .version("0.1.0")
                .author("Milo Banks (Isacc Barker) <milobanks@zincsoft.dev>")
                .subcommand(
                    App::new("assign")
                        .about("Assign a Rhai script to a added file.")
                        .version("0.1.0")
                        .author("Milo Banks (Isacc Barker) <milobanks@zincsoft.dev>")
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
                        .version("0.1.0")
                        .author("Milo Banks (Isacc Barker) <milobanks@zincsoft.dev>")
                        .arg(
                            Arg::from("<file> 'The file to operate on'")
                                .index(1)
                                .required(true)
                        )
                        .arg(
                            Arg::from("<script> 'The script to operate on'")
                                .index(2)
                                .required(true)
                        )
                )
                .subcommand(
                    App::new("jog")
                        .about("Run script associated with a file.")
                        .version("0.1.0")
                        .author("Milo Banks (Isacc Barker) <milobanks@zincsoft.dev>")
                )
                .subcommand(
                    App::new("marathon")
                        .about("Run all scripts for the main manifest.")
                        .version("0.1.0")
                        .author("Milo Banks (Isacc Barker) <milobanks@zincsoft.dev>")
                )
                .subcommand(
                    App::new("fold-fitted-sheet")
                        .about("Run all scripts everywhere. It's almost as crazy as trying to fold a fitted sheet!")
                        .version("0.1.0")
                        .author("Milo Banks (Isacc Barker) <milobanks@zincsoft.dev>")
                )
        )
        .get_matches(); // TODO: Push and pull commands?

    if Uid::effective().is_root() {
        eprintln!(
            "You should not be great as root, or it might track files for the
root user. The feeling might also go to your head, and being root
may just tip you over into a state of catatonia.
If you got a permision error previously, please just change the permisions
on the directory."
        );
        std::process::exit(1);
    }

    // Check if we are initialized
    if !default_greatness_dir.as_path().exists() && matches.subcommand_name().unwrap() != "init" {
        error!("It looks you haven't initialized yet! Use `greatness init` to initialize. P.S, we found this out by looking through some pretty great binoculars.");

        std::process::exit(1);
    }

    let mut manifest: manifest::Manifest;
    match manifest::Manifest::new(PathBuf::from(
        default_greatness_dir.as_os_str().to_str().unwrap(),
    )) {
        Ok(m) => manifest = m,
        Err(e) => {
            error!(
                "An error occured whilst getting the greatness manifest: {}",
                e
            );
            std::process::exit(1);
        }
    }

    if matches.subcommand_name().unwrap_or("") != "init" {
        let manifest_data: ManifestData = match ManifestData::populate_from_file(&manifest) {
            Ok(md) => md,
            Err(e) => {
                error!(
                    "An error occured whilst parsing the greatness manifest: {}",
                    e
                );
                std::process::exit(1);
            }
        };

        manifest.data = manifest_data;
    }

    match matches.subcommand() {
        Some(("status", status_matches)) => {
            if status_matches.is_present("file") {
                match status::print_file_status(&manifest, status_matches) {
                    Ok(()) => (),
                    Err(e) => {
                        error!(
                            "An error occured whilst getting the status of the specified file: {}",
                            e
                        );

                        std::process::exit(1);
                    }
                }

                std::process::exit(0);
            }

            status::print_status(&mut manifest);
        }

        Some(("doctor", doctor_matches)) => {
            let verdict = doctor::doctor(&manifest, doctor_matches);
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
                &mut manifest,
            ) {
                Ok(()) => (),
                Err(e) => {
                    error!("An error occured whilst tracking great file(s): {}", e);

                    std::process::exit(1);
                }
            }
        }

        Some(("rm", rm_matches)) => match rm::rm(rm_matches, &mut manifest) {
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

            match init::init(init_matches, &manifest) {
                Ok(()) => (),
                Err(e) => {
                    error!("An error occured whilst initialising your local great greatness environment: {}", e);

                    std::process::exit(1);
                }
            }
        }

        Some(("pull", get_matches)) => {
            match pull::clone_and_install_repo(
                get_matches.value_of("from").unwrap().to_string(),
                get_matches,
                &mut manifest,
                false, // This is the original project
            ) {
                Ok(()) => (),
                Err(e) => {
                    error!(
                        "An error occured whilst cloning/installing the external manifest: {}",
                        e
                    );

                    std::process::exit(1);
                }
            }
        }

        Some(("repel", repel_matches)) => match repel::repel(repel_matches, &mut manifest) {
            Ok(()) => (),
            Err(e) => {
                error!(
                    "An error occured whilst repeling (unpulling) the external manifest: {}",
                    e
                );

                std::process::exit(1);
            }
        },

        Some(("pack", pack_matches)) => match pack::pack(&mut manifest, pack_matches) {
            Ok(()) => (),
            Err(e) => {
                error!(
                    "An error occured whilst packing greatness into a small space: {}",
                    e
                );

                std::process::exit(1);
            }
        },

        Some(("tag", tag_matches)) => match tag::tag(tag_matches, &mut manifest) {
            Ok(()) => (),
            Err(e) => {
                error!("An error occured whilst tagging the file(s): {}", e);

                std::process::exit(1);
            }
        },

        Some(("prompt", prompt_matches)) => match prompt::prompt(prompt_matches, &mut manifest) {
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
                match script::assign::assign(assign_matches, &mut manifest) {
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

            Some(("rm", rm_matches)) => {
                match script::rm::rm(rm_matches, &mut manifest) {
                    Ok(()) => (),
                    Err(e) => {
                        error!(
                            "An error occured whilst un-tethering a script and a file: {}",
                            e
                        );

                        std::process::exit(1);
                    }
                }
            }

            Some(("jog", jog_matches)) => {}

            Some(("marathon", marathon_matches)) => {}

            Some(("fold-fitted-sheet", fold_matches)) => {}

            None => {
                eprintln!("Please use the --help flag to get great knowlage!")
            }
            _ => unreachable!(),
        },

        None => eprintln!("Please use the --help flag to get great knowlage!"),
        _ => unreachable!(),
    }
}
