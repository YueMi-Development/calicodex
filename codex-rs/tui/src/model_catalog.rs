use codex_protocol::openai_models::InputModality;
use codex_protocol::openai_models::ModelPreset;
use codex_protocol::openai_models::ReasoningEffort;
use std::convert::Infallible;

use crate::custom_models::load_custom_models;

#[derive(Debug, Clone)]
pub(crate) struct ModelCatalog {
    models: Vec<ModelPreset>,
    /// When set, load models from `custom_models.json` on each list call
    /// instead of using the static `models` vec.
    custom_codex_home: Option<std::path::PathBuf>,
}

impl ModelCatalog {
    #[allow(dead_code)]
    pub(crate) fn new(models: Vec<ModelPreset>) -> Self {
        Self {
            models,
            custom_codex_home: None,
        }
    }

    /// Enable custom model support. When `codex_home` is provided,
    /// `try_list_models` will load models from `custom_models.json`
    /// each time it is called if that file exists and has entries.
    /// Falls back to the default model list when the file is absent
    /// or empty.
    pub(crate) fn with_custom_model_support(
        models: Vec<ModelPreset>,
        codex_home: impl Into<Option<std::path::PathBuf>>,
    ) -> Self {
        Self {
            models,
            custom_codex_home: codex_home.into(),
        }
    }

    pub(crate) fn try_list_models(&self) -> Result<Vec<ModelPreset>, Infallible> {
        if let Some(ref home) = self.custom_codex_home {
            let custom_file = home.join("custom_models.json");
            // Only use custom models if the file exists and has entries
            if custom_file.exists() {
                let custom = load_custom_models(home);
                if !custom.is_empty() {
                    return Ok(custom.into_iter().map(entry_to_preset).collect());
                }
            }
        }
        Ok(self.models.clone())
    }
}

fn entry_to_preset(entry: crate::custom_models::CustomModelEntry) -> ModelPreset {
    ModelPreset {
        id: entry.id.clone(),
        model: entry.id.clone(),
        display_name: entry.display_name,
        description: entry.description,
        default_reasoning_effort: ReasoningEffort::Medium,
        supported_reasoning_efforts: Vec::new(),
        supports_personality: false,
        additional_speed_tiers: Vec::new(),
        service_tiers: Vec::new(),
        default_service_tier: None,
        is_default: false,
        upgrade: None,
        show_in_picker: true,
        availability_nux: None,
        supported_in_api: true,
        input_modalities: vec![InputModality::Text],
    }
}
