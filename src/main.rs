mod add;
mod log_utils;
mod manifest;
mod progress;
mod pull;
mod pack;
mod utils;
mod init;

use clap::{App, AppSettings, Arg};
use env_logger::{Builder, Target};
use log::LevelFilter;
use log::error;
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
            Arg::from("<ignore-root-check> --ignore-root-check 'allow to run as root'")
                .required(false),
        )
        .subcommand(
            App::new("add")
                .about("Adds (a) file(s) to the manifest")
                .version("0.1.0")
                .author("Milo Banks (Isacc Barker) <milobanks@zincsoft.dev>")
                .setting(AppSettings::TrailingVarArg)
                .arg(Arg::from("<files>... 'File(s) to add'").required(true)),
        )
        .subcommand(
            App::new("init")
                .about("Initializes greatness!")
                .version("0.1.0")
                .author("Milo Banks (Isacc Barker) <milobanks@zincsoft.dev>")
                .arg(Arg::from("--force 'Force to reinitialize'").required(false).takes_value(false)),
        )
        .subcommand(
            App::new("pull")
                .about("Fetches and merges external manifests")
                .version("0.1.0")
                .author("Milo Banks (Isacc Barker) <milobanks@zincsoft.dev>")
                .arg(Arg::from("<from> 'where to fetch the external manifest'").required(true).index(1)),
        )
        .subcommand(
            App::new("pack")
                .about("Pack all your dotfiles and configurations into multiple formats")
                .version("0.1.0")
                .author("Milo Banks (Isacc Barker) <milobanks@zincosft.dev>")
                .arg(Arg::from("<type> 'what to pack into. values: git'").required(true).index(1))
                .arg(Arg::from("<where> 'where to pack into'").required(true).index(2))
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
    if ! default_greatness_dir.as_path().exists() && matches.subcommand_name().unwrap() != "init" {
        error!("It looks you haven't initialized yet! Use `greatness init` to initialize. P.S, we found this out by looking through some pretty great binoculars.");

        std::process::exit(1);
    }

    let mut manifest: manifest::Manifest;
    match manifest::Manifest::new(PathBuf::from(default_greatness_dir.as_os_str().to_str().unwrap())) {
        Ok(m) => manifest = m,
        Err(e) => {
            error!(
                "An error occured whilst getting the greatness manifest: {}",
                e
            );
            std::process::exit(1);
        }
    }

    if matches.subcommand_name().unwrap() != "init" {
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
                    error!("An error occured whilst tracking great files: {}", e);

                    std::process::exit(1);
                }
            }
        }

        Some(("init", init_matches)) => {
            if default_greatness_dir.as_path().exists() && ! init_matches.is_present("force") {
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
                    error!("An error occured whilst cloning/installing the repo: {}", e);

                    std::process::exit(1);
                }
            }
        }

        Some(("pack", pack_matches)) => {
            match pack::pack(
                pack_matches.value_of("type").unwrap().to_string(),
                &mut manifest,
                pack_matches,
            ) {
                Ok(()) => (),
                Err(e) => {
                    error!("An error occured whilst packing greatness into a small space: {}", e);

                    std::process::exit(1);
                }
            }
        }

        None => eprintln!("Please use the --help flag to get great knowlage!"),
        _ => unreachable!(),
    }
}
