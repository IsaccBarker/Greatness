pub mod add;
pub mod rm;

use std::path::PathBuf;

pub fn get_retname(input: &PathBuf) -> PathBuf {
    let mut ret = input.clone();
    ret.set_extension(ret.extension().unwrap_or(std::ffi::OsStr::new("")).to_str().unwrap().to_owned() + "greatenc");

    ret
}
