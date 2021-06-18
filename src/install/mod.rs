use crate::manifest;

use snafu::{ResultExt, Snafu};
use std::path::PathBuf;
use crate::progress;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum InstallError {
    #[snafu(display("Failed to make great symlink {} -> {}: {}", src.display(), dest.display(), source))]
    InstallSymlink {
        src: PathBuf,
        dest: PathBuf,
        source: std::io::Error,
    },
}

pub fn install_files(manifest: &mut manifest::Manifest) -> Result<(), InstallError> {
    let files = manifest.data.files.take().unwrap_or(vec![]);
    let pb = progress::new_progress_bar(files.len() as u64);

    for file in &files {
        pb.set_message(format!(
            "Symlinking {} -> {}....",
            &file.0.display(),
            &file.1.display()
        ));

        install_file(&file.0, &file.1)?;
        pb.inc(1);
    }

    pb.reset();

    Ok(())
}

pub fn install_file(src: &PathBuf, dest: &PathBuf) -> Result<(), InstallError>{
    symlink::symlink_file(src, dest).context(InstallSymlink{src, dest})?;
    
    Ok(())
}

