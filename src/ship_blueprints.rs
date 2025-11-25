use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::ship_design::{ModuleCategory, ShipDesign};

/// Serializable blueprint representation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShipBlueprint {
    pub name: String,
    pub design: ShipDesign,
}

/// Errors that can occur while loading or saving blueprints.
#[derive(Debug, Error)]
pub enum BlueprintError {
    #[error("io error at {path}: {source}")]
    Io {
        source: std::io::Error,
        path: String,
    },
    #[error("failed to parse {path}: {source}")]
    Parse {
        source: toml::de::Error,
        path: String,
    },
    #[error("failed to serialize blueprint: {0}")]
    Serialize(toml::ser::Error),
}

/// Load all blueprint files from the directory.
pub fn load_blueprints(dir: impl AsRef<Path>) -> Result<Vec<ShipBlueprint>, BlueprintError> {
    let mut blueprints = Vec::new();
    let path = dir.as_ref();
    if !path.exists() {
        return Ok(blueprints);
    }

    for entry in fs::read_dir(path).map_err(|source| BlueprintError::Io {
        source,
        path: path.display().to_string(),
    })? {
        let entry = entry.map_err(|source| BlueprintError::Io {
            source,
            path: path.display().to_string(),
        })?;
        let file_path = entry.path();
        if file_path.extension().and_then(|s| s.to_str()) != Some("toml") {
            continue;
        }
        let content = fs::read_to_string(&file_path).map_err(|source| BlueprintError::Io {
            source,
            path: file_path.display().to_string(),
        })?;
        let blueprint: ShipBlueprint =
            toml::from_str(&content).map_err(|source| BlueprintError::Parse {
                source,
                path: file_path.display().to_string(),
            })?;
        blueprints.push(blueprint);
    }

    Ok(blueprints)
}

/// Save a blueprint to the directory, creating the folder if needed.
pub fn save_blueprint(
    dir: impl AsRef<Path>,
    blueprint: &ShipBlueprint,
) -> Result<PathBuf, BlueprintError> {
    let dir_path = dir.as_ref();
    fs::create_dir_all(dir_path).map_err(|source| BlueprintError::Io {
        source,
        path: dir_path.display().to_string(),
    })?;

    let file_path = dir_path.join(format!("{}.toml", blueprint.name));
    let content = toml::to_string_pretty(blueprint).map_err(BlueprintError::Serialize)?;
    fs::write(&file_path, content).map_err(|source| BlueprintError::Io {
        source,
        path: file_path.display().to_string(),
    })?;

    Ok(file_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ship_design::ShipDesign;
    use std::fs;
    use std::path::PathBuf;

    fn sample_blueprint() -> ShipBlueprint {
        let mut design = ShipDesign::new("small");
        design.add_module(ModuleCategory::Engine, "tonklin_motor");
        design.add_module(ModuleCategory::Weapon, "laser_beam");
        ShipBlueprint {
            name: "test_ship".to_string(),
            design,
        }
    }

    #[test]
    fn saves_and_loads_blueprints() {
        let dir = PathBuf::from("tmp/blueprints_test");
        if dir.exists() {
            fs::remove_dir_all(&dir).unwrap();
        }

        let bp = sample_blueprint();
        let path = save_blueprint(&dir, &bp).expect("save should work");
        assert!(path.exists());

        let loaded = load_blueprints(&dir).expect("load should work");
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0], bp);

        fs::remove_dir_all(&dir).unwrap();
    }
}
