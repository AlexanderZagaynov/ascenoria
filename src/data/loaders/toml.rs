use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::data::errors::DataLoadError;

pub(crate) fn load_toml_file<T>(path: &Path) -> Result<T, DataLoadError>
where
    T: for<'de> Deserialize<'de>,
{
    let content = fs::read_to_string(path).map_err(|source| DataLoadError::Io {
        source,
        path: path.display().to_string(),
    })?;

    toml::from_str::<T>(&content).map_err(|source| DataLoadError::Parse {
        source,
        path: path.display().to_string(),
    })
}

pub(crate) fn load_toml_file_optional<T>(path: &Path) -> Result<Option<T>, DataLoadError>
where
    T: for<'de> Deserialize<'de>,
{
    match fs::read_to_string(path) {
        Ok(content) => {
            toml::from_str::<T>(&content)
                .map(Some)
                .map_err(|source| DataLoadError::Parse {
                    source,
                    path: path.display().to_string(),
                })
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(source) => Err(DataLoadError::Io {
            source,
            path: path.display().to_string(),
        }),
    }
}
