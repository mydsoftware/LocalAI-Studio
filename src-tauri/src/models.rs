use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelVariant {
    pub id: String,
    pub format: String,
    pub quantization: Option<String>,
    pub file_size: u64,
    pub estimated_ram: u64,
    pub estimated_vram: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDescriptor {
    pub id: String,
    pub name: String,
    pub author: String,
    pub architecture: Option<String>,
    pub parameter_count: Option<u64>,
    pub context_length: Option<u64>,
    pub task: String,
    pub variants: Vec<ModelVariant>,
}

pub fn catalog() -> Vec<ModelDescriptor> {
    vec![
        ModelDescriptor { id: "Qwen/Qwen3-8B-GGUF".into(), name: "Qwen3 8B".into(), author: "Qwen".into(), architecture: Some("Qwen3".into()), parameter_count: Some(8_000_000_000), context_length: Some(32_768), task: "گفتگو و استدلال".into(), variants: vec![ModelVariant{id:"Q4_K_M".into(),format:"GGUF".into(),quantization:Some("Q4_K_M".into()),file_size:5_000_000_000,estimated_ram:7_000_000_000,estimated_vram:5_500_000_000}] },
        ModelDescriptor { id: "Qwen/Qwen2.5-Coder-7B-Instruct-GGUF".into(), name: "Qwen2.5 Coder 7B".into(), author: "Qwen".into(), architecture: Some("Qwen2.5".into()), parameter_count: Some(7_000_000_000), context_length: Some(32_768), task: "کدنویسی".into(), variants: vec![ModelVariant{id:"Q4_K_M".into(),format:"GGUF".into(),quantization:Some("Q4_K_M".into()),file_size:4_700_000_000,estimated_ram:6_500_000_000,estimated_vram:5_100_000_000}] },
    ]
}
