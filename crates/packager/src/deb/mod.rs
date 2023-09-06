use std::path::PathBuf;

use crate::config::Config;

pub fn package(_config: &Config) -> crate::Result<Vec<PathBuf>> {
    log::error!("`deb` format is not implemented yet!");
    std::process::exit(1);
}
