use crate::manifest::Manifest;
use crate::utils;
use clap::ArgMatches;
use snafu::{Snafu, ResultExt};
use std::path::PathBuf;
use log::warn;
use encryptfile as ef;

#[derive(Debug, Snafu)]
pub enum EncryptionErrors {
    #[snafu(display("Passwords do not match!"))]
    PasswordMismatch {
        source: std::io::Error
    },

    #[snafu(display("Password is empty!"))]
    PasswordEmpty {
        source: std::io::Error
    }
}

pub fn add(matches: &ArgMatches, manifest: &mut Manifest) -> Result<(), Box<dyn std::error::Error>> {
    let files = matches.values_of("files").unwrap();

    let p1 = match question::Question::new("Password         : ").clarification("Please enter the password you wish to use for file encryption").ask().unwrap() {
        question::Answer::RESPONSE(r) => r,
        _ => unreachable!(),
    };
    let p2 = match question::Question::new("Re-enter Password: ").clarification("Please enter the password you wish to use for file encryption").ask().unwrap() {
        question::Answer::RESPONSE(r) => r,
        _ => unreachable!(),
    };

    if p1 != p2 {
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData)).context(PasswordMismatch{})?;
    }

    if p1 == "" {
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData)).context(PasswordEmpty{})?;
    }

    for file in files {
        let mut special_file = utils::relative_to_special(&PathBuf::from(file)).context(utils::FileOpenError{file})?;
        if let Some(manifest_files) = &mut manifest.data.files {
            for manifest_file in manifest_files {
                if manifest_file.path == PathBuf::from(&special_file) {
                    special_file = utils::special_to_absolute(&special_file);

                    let output_file = super::get_retname(&special_file);
                    
                    std::fs::File::create(&output_file).context(utils::FileCreationError{file: &output_file})?;

                    warn!("Encrypting files.... This may take a while, because unlike other people, we actually care about your great security.");

                    ef::process(ef::Config::new()
                        .input_stream(ef::InputStream::File(special_file.to_str().unwrap().to_string()))
                        .output_stream(ef::OutputStream::File(output_file.to_str().unwrap().to_string()))
                        .add_output_option(ef::OutputOption::AllowOverwrite)
                        .initialization_vector(ef::InitializationVector::GenerateFromRng)
                        .password(ef::PasswordType::Text(p1.clone(), ef::scrypt_defaults()))
                        .encrypt()).unwrap();

                    manifest_file.encrypted = true;
                }
            }
        } else {
            Err(std::io::Error::from(std::io::ErrorKind::InvalidData)).context(utils::FileNotTracked{file})?;
        }
    }

    manifest.data.populate_file(&manifest);

    Ok(())
}
