use std::path::Path;

use crate::{prelude::*, utilities::*};

// pub fn write_str<P: AsRef<Path>>(path: P, value: &str) -> AtrlResult<()> {
//     let path = path.as_ref();
//     let path_string = match path.to_str() {
//         Some(s) => s.to_string(),
//         None => return Err(AtrlError::NotAString),
//     };

//     match path.try_exists() {
//         Ok(b) => {
//             if !b {
//                 match path.parent() {
//                     Some(dir) => {
//                         #[cfg(feature = "debug")]
//                         debug!("Creating directory: {:?}", dir);
//                         if let Err(e) = std::fs::create_dir_all(dir) {
//                             return Err(AtrlError::Io(e));
//                         }
//                     },
//                     None => return Err(AtrlError::NotADir(path_string)),
//                 };
//             }
//             #[cfg(feature = "debug")]
//             debug!("Writing to file: {:?}", path);
//             match std::fs::write(path, value) {
//                 Ok(_) => Ok(()),
//                 Err(e) => Err(AtrlError::Io(e)),
//             }
//         },
//         Err(e) => Err(AtrlError::Io(e)),
//     }
// }

pub fn write_toml<T: for<'a> serde::Serialize>(path: impl AsRef<Path>, serialized: &T) -> AtrlResult<()> {
    let path = path.as_ref();
    toml::to_string_pretty(serialized).map_or_else(
        |err| {
            error!("Error converting {} to toml.", path.display());
            Err(AtrlError::TomlSerializationError(err))
        },
        |contents| {
            File::create(path).map_or_else(
                |err| {
                    error!("Error creating app_settings.toml: {}", err);
                    Err(AtrlError::NotAFile(path.display().to_string()))
                },
                |mut file| {
                    file.write(contents.as_bytes()).map_or_else(
                        |err| {
                            error!("Error writing to {}: {}", path.display(), err);
                            Err(AtrlError::Io(err))
                        },
                        |_| Ok(()),
                    )
                },
            )
        },
    )
}
