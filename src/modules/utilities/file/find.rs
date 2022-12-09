use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{error::*, prelude::*};

pub fn find_files_with_extension<P: AsRef<Path>>(path: P, extension: &str) -> AtrlResult<Vec<String>> {
    find_files_with_extensions(path, vec![extension])
}

pub fn find_files_with_extensions<P: AsRef<Path>>(path: P, extensions: Vec<&str>) -> AtrlResult<Vec<String>> {
    let mut ret = Vec::new();
    let path = path.as_ref();
    let paths = std::fs::read_dir(path)?;
    #[cfg(feature = "debug")]
    {
        let mut extension_names = String::new();
        let mut first = true;
        for extension in &extensions {
            if first {
                extension_names = extension.to_string();
                first = false;
            } else {
                extension_names = format!("{extension_names}, {extension}");
            }
        }
        info!("Looking for {{{}}} files in {:?}", extension_names, path);
    }
    for dir_entry in paths.flatten() {
        let path = dir_entry.path();
        if let Some(ext) = path.extension() {
            let mut found = false;
            for extension in &extensions {
                if ext == *extension {
                    found = true;
                    break;
                }
            }
            if found {
                if let Some(file_name) = path.file_name() {
                    if let Some(path_name) = file_name.to_str() {
                        ret.push(path_name.to_string());
                    }
                }
            }
        }
    }
    #[cfg(feature = "debug")]
    {
        let mut found_names = String::new();
        let mut first = true;
        for file in &ret {
            if first {
                found_names = format!("\t{file}");
                first = false;
            } else {
                found_names = format!("{found_names}\n\t{file}");
            }
        }
        info!("Found {{\n{}\n}}", found_names);
    }
    Ok(ret)
}

pub fn strip_extension<P: AsRef<Path>>(path: P) -> Option<String> {
    let path = path.as_ref();
    path.with_extension("").to_str().map(|p| p.to_string())
}
