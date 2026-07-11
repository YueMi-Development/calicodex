//! Persistence for user-defined custom provider models.
//!
//! When a user configures a custom API provider, this module manages a
//! `custom_models.json` file in `$CODEX_HOME` that contains the model list
//! they want to see in the picker.

use serde::Deserialize;
use serde::Serialize;
use std::path::Path;
use std::path::PathBuf;

/// One user-defined model entry for a custom API provider.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub(crate) struct CustomModelEntry {
    pub id: String,
    pub display_name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
struct CustomModelsFile {
    models: Vec<CustomModelEntry>,
}

fn custom_models_path(codex_home: &Path) -> PathBuf {
    codex_home.join("custom_models.json")
}

/// Load the user's custom model list from `$CODEX_HOME/custom_models.json`.
/// Returns an empty `Vec` if the file does not exist or cannot be parsed.
pub(crate) fn load_custom_models(codex_home: &Path) -> Vec<CustomModelEntry> {
    let path = custom_models_path(codex_home);
    match std::fs::read_to_string(&path) {
        Ok(contents) => serde_json::from_str::<CustomModelsFile>(&contents)
            .map(|f| f.models)
            .unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

/// Save the user's custom model list to `$CODEX_HOME/custom_models.json`.
pub(crate) fn save_custom_models(
    codex_home: &Path,
    models: &[CustomModelEntry],
) -> std::io::Result<()> {
    let path = custom_models_path(codex_home);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let contents = serde_json::to_string_pretty(&CustomModelsFile {
        models: models.to_vec(),
    })
    .map_err(std::io::Error::other)?;
    std::fs::write(&path, contents)
}

/// Add a model entry to the custom model list and persist.
pub(crate) fn add_custom_model(
    codex_home: &Path,
    entry: CustomModelEntry,
) -> std::io::Result<Vec<CustomModelEntry>> {
    let mut models = load_custom_models(codex_home);
    models.retain(|m| m.id != entry.id);
    models.push(entry);
    save_custom_models(codex_home, &models)?;
    Ok(models)
}

/// Remove a model entry from the custom model list by id and persist.
pub(crate) fn remove_custom_model(
    codex_home: &Path,
    id: &str,
) -> std::io::Result<Vec<CustomModelEntry>> {
    let mut models = load_custom_models(codex_home);
    models.retain(|m| m.id != id);
    save_custom_models(codex_home, &models)?;
    Ok(models)
}
