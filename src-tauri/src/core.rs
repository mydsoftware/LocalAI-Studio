use crate::hardware::HardwareProfile;
use crate::models::{ModelDescriptor, ModelVariant};
use crate::recommendation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceConfig {
    pub context_length: u32,
    pub gpu_layers: i32,
    pub threads: u32,
    pub batch_size: u32,
    pub temperature: f32,
    pub top_p: f32,
    pub top_k: u32,
    pub repeat_penalty: f32,
    pub flash_attention: bool,
    pub cpu_offload: bool,
}

impl Default for InferenceConfig {
    fn default() -> Self {
        Self {
            context_length: 4096,
            gpu_layers: -1,
            threads: 0,
            batch_size: 512,
            temperature: 0.7,
            top_p: 0.9,
            top_k: 40,
            repeat_penalty: 1.1,
            flash_attention: true,
            cpu_offload: false,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct InstallationPlan {
    pub model: String,
    pub variant: String,
    pub runtime: String,
    pub backend: String,
    pub config: InferenceConfig,
    pub estimated_download_size: u64,
    pub estimated_memory: u64,
    pub compatibility_score: u8,
    pub warnings: Vec<String>,
}

pub fn create_install_plan(
    hw: &HardwareProfile,
    model: &ModelDescriptor,
    variant: &ModelVariant,
) -> InstallationPlan {
    let rec = recommendation::recommend(hw, model, variant);
    let gpu_layers = if rec.backend == "CPU" { 0 } else { -1 };
    let max_context = model.context_length.unwrap_or(4096).min(8192) as u32;
    let threads = (hw.logical_cores.max(1) as u32).min(32);
    let mut config = InferenceConfig {
        context_length: max_context.max(2048),
        gpu_layers,
        threads,
        ..InferenceConfig::default()
    };

    if rec.compatibility == "Limited" {
        config.context_length = config.context_length.min(4096);
        config.batch_size = 256;
    }
    if rec.compatibility == "Unsupported" {
        config.context_length = 2048;
        config.batch_size = 128;
        config.gpu_layers = 0;
    }

    InstallationPlan {
        model: model.id.clone(),
        variant: variant.id.clone(),
        runtime: rec.runtime,
        backend: rec.backend,
        config,
        estimated_download_size: variant.file_size,
        estimated_memory: variant.estimated_ram,
        compatibility_score: rec.score,
        warnings: rec.warnings,
    }
}
