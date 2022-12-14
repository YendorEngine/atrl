use std::path::{Path, PathBuf};

use crate::{prelude::*, utilities::*};

pub fn read_str<P: AsRef<Path>>(path: P) -> AtrlResult<String> {
    match std::fs::read_to_string(path) {
        Ok(s) => Ok(s),
        Err(e) => Err(AtrlError::Io(e)),
    }
}

pub fn read_ron<T: for<'a> serde::Deserialize<'a>>(path: impl Into<PathBuf>) -> AtrlResult<T> {
    let path: PathBuf = path.into();
    std::fs::canonicalize::<PathBuf>(path).map_or_else(
        |err| Err(AtrlError::NotAFile(err.to_string())),
        |path| {
            std::fs::File::open(path).map_or_else(
                |err| Err(AtrlError::Io(err)),
                |file| {
                    ron::de::from_reader::<_, T>(file)
                        .map_or_else(|err| Err(AtrlError::RonError(err.code)), |ron| Ok(ron))
                },
            )
        },
    )
}

pub fn read_toml<T: for<'a> serde::Deserialize<'a>>(path: impl AsRef<Path>) -> AtrlResult<T> {
    let path = path.as_ref();
    File::open(path).map_or_else(
        |err| {
            error!("Error opening app_settings.toml: {}", err);
            Err(AtrlError::NotAFile(err.to_string()))
        },
        |file| {
            read_to_string(BufReader::new(file)).map_or_else(
                |err| {
                    error!("Error reading {}: {}", path.display(), err);
                    Err(AtrlError::Io(err))
                },
                |contents| {
                    toml::from_str::<T>(&contents).map_or_else(
                        |err| {
                            error!("Error converting {}: {}", path.display(), err);
                            Err(AtrlError::Io(err.into()))
                        },
                        |toml| Ok(toml),
                    )
                },
            )
        },
    )
}
