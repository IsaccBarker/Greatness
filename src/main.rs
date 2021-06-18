#![feature(command_access)]

mod log_utils;
mod progress;
mod manifest;
mod track;
mod install;

use clap::{App, AppSettings, Arg};
use env_logger::{Builder, Target};
use log::LevelFilter;
use log::{debug, error};
use nix::unistd::Uid;
use std::io::Write;
use manifest::ManifestData;

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
        .author("Milo Banks (Isacc Barker) <plutoisaplanet4324@gmail.com>")
        .about("Helps you to achieve greatness!")
        .arg(
            Arg::from("<ignore-root-check> --ignore-root-check 'allow to run as root'")
                .required(false),
        )
        .arg(
            Arg::from("<root> -r,--root 'the root to merge greatness into'")
                .required(false)
                .default_value(match std::path::MAIN_SEPARATOR {
                    '/' => "/",
                    '\\' => "C:\\",
                    _ => unreachable!(),
                }),
        )
        .arg(
            Arg::from("<greatness-dir> -g,--greatness 'the directory that your great configuration is stored in'")
                .required(false)
                .default_value(default_greatness_dir.as_os_str().to_str().unwrap())
        )
        .subcommand(
            App::new("track")
                .about("Adds (a) file(s) to the manifest")
                .version("0.1.0")
                .author("Milo Banks (Isacc Barker) <milobanks@zincsoft.dev>")
                .setting(AppSettings::TrailingVarArg)
                .arg(Arg::from("<files>... 'File(s) to add'").required(true)),
        )
        .subcommand(
            App::new("install")
                .about("Installs tracked files")
                .version("0.1.0")
                .author("Milo Banks (Iascc Barker) <milobanks@zincsoft.dev>")
        )
        .subcommand(
            App::new("get")
                .about("Fetches and merges external manifests")
                .version("0.1.0")
                .author("Milo Banks (Isacc Barker) <milobanks@zincsoft.dev>")
                .arg(Arg::from("<from> -f,--from 'where to fetch the external manifest'").required(true)),
        )
        .get_matches(); // TODO: Push and pull commands?

    if Uid::effective().is_root() && !matches.is_present("ignore-root-check") {
        eprintln!(
            "You should not be great as root, or it might track files for the
root user. The feeling might also go to your head, and being root
may just tip you over into a state of catatonia. If you really want
to do this, please supply the --ignore-root-check flag.
If you got a permision error previously, please just change the permisions
on the directory."
        );
        std::process::exit(1);
    }

    debug!(
        "Using root of {}, which is undeniably great!",
        matches.value_of("root").unwrap()
    );

    let mut manifest: manifest::Manifest;
    match manifest::Manifest::new() {
        Ok(m) => manifest = m,
        Err(e) => {
            error!(
                "An error occured whilst getting the greatness manifest: {}",
                e
            );
            std::process::exit(1);
        }
    }

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

    match matches.subcommand() {
        Some(("track", track_matches)) => {
            match track::track_files(
                &matches,
                track_matches
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

        Some(("install", _install_matches)) => {
            match install::install_files(&mut manifest) {
                Ok(()) => (),
                Err(e) => {
                    error!("An error occured whilst installing great files: {}", e);

                    std::process::exit(1);
                }
            }
        }

        Some(("get", _get_matches)) => {
            println!("get");
        }

        None => eprintln!("Please use the --help flag to get great knowlage!"),
        _ => unreachable!(),
    }
}
