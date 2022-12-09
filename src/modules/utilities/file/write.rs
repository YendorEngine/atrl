use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{error::*, prelude::*};

pub fn write_str<P: AsRef<Path>>(path: P, value: &str) -> AtrlResult<()> {
    let path = path.as_ref();
    let path_string = match path.to_str() {
        Some(s) => s.to_string(),
        None => return Err(AtrlError::NotAString),
    };
    match path.try_exists() {
        Ok(b) => {
            if !b {
                match path.parent() {
                    Some(dir) => {
                        #[cfg(feature = "debug")]
                        debug!("Creating directory: {:?}", dir);
                        if let Err(e) = std::fs::create_dir_all(dir) {
                            return Err(AtrlError::Io(e));
                        }
                    },
                    None => return Err(AtrlError::NotADir(path_string)),
                };
            }
            #[cfg(feature = "debug")]
            debug!("Writing to file: {:?}", path);
            match std::fs::write(path, value) {
                Ok(_) => Ok(()),
                Err(e) => Err(AtrlError::Io(e)),
            }
        },
        Err(e) => Err(AtrlError::Io(e)),
    }
}
