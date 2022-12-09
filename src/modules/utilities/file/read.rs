use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{error::*, prelude::*};

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

pub fn read_toml<T: for<'a> serde::Deserialize<'a>>(path: impl Into<PathBuf>) -> AtrlResult<T> {
    let path: PathBuf = path.into();
    std::fs::canonicalize::<PathBuf>(path).map_or_else(
        |err| Err(AtrlError::NotAFile(err.to_string())),
        |path| match path.to_str().map_or_else(|| Err(AtrlError::NotAString), read_str) {
            Ok(file) => {
                toml::from_str::<T>(&file).map_or_else(|err| Err(AtrlError::Io(err.into())), |toml| Ok(toml))
            },
            Err(err) => Err(err),
        },
    )
}
