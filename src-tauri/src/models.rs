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
    pub downloads: Option<u64>,
    pub likes: Option<u64>,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct HfModel {
    id: String,
    #[serde(default)]
    author: Option<String>,
    #[serde(default)]
    downloads: Option<u64>,
    #[serde(default)]
    likes: Option<u64>,
    #[serde(default)]
    pipeline_tag: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
}

fn task_name(tag: Option<&str>) -> String {
    match tag.unwrap_or_default() {
        "text-generation" => "تولید متن",
        "text2text-generation" => "تولید متن",
        "feature-extraction" => "Embedding",
        "image-text-to-text" => "چندوجهی",
        "automatic-speech-recognition" => "تشخیص گفتار",
        other if !other.is_empty() => other,
        _ => "عمومی",
    }
    .to_string()
}

pub fn search_huggingface(query: &str, limit: usize) -> Result<Vec<ModelDescriptor>, String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("LocalAI-Studio/1.0")
        .build()
        .map_err(|e| format!("ساخت کلاینت Hugging Face ناموفق بود: {e}"))?;
    let response = client
        .get("https://huggingface.co/api/models")
        .query(&[
            ("search", query),
            ("sort", "downloads"),
            ("direction", "-1"),
            ("limit", &limit.clamp(1, 50).to_string()),
        ])
        .send()
        .and_then(|r| r.error_for_status())
        .map_err(|e| format!("دریافت مدل‌ها از Hugging Face ناموفق بود: {e}"))?;

    let items: Vec<HfModel> = response
        .json()
        .map_err(|e| format!("پاسخ Hugging Face قابل خواندن نیست: {e}"))?;

    Ok(items
        .into_iter()
        .map(|m| {
            let name = m.id.rsplit('/').next().unwrap_or(&m.id).to_string();
            let author = m
                .author
                .unwrap_or_else(|| m.id.split('/').next().unwrap_or("نامشخص").to_string());
            ModelDescriptor {
                id: m.id,
                name,
                author,
                architecture: None,
                parameter_count: None,
                context_length: None,
                task: task_name(m.pipeline_tag.as_deref()),
                variants: Vec::new(),
                downloads: m.downloads,
                likes: m.likes,
                tags: m.tags,
            }
        })
        .collect())
}

pub fn catalog() -> Vec<ModelDescriptor> {
    vec![
        ModelDescriptor {
            id: "Qwen/Qwen3-8B-GGUF".into(),
            name: "Qwen3 8B".into(),
            author: "Qwen".into(),
            architecture: Some("Qwen3".into()),
            parameter_count: Some(8_000_000_000),
            context_length: Some(32_768),
            task: "گفتگو و استدلال".into(),
            variants: vec![ModelVariant {
                id: "Q4_K_M".into(),
                format: "GGUF".into(),
                quantization: Some("Q4_K_M".into()),
                file_size: 5_000_000_000,
                estimated_ram: 7_000_000_000,
                estimated_vram: 5_500_000_000,
            }],
            downloads: None,
            likes: None,
            tags: vec!["gguf".into(), "text-generation".into()],
        },
        ModelDescriptor {
            id: "Qwen/Qwen2.5-Coder-7B-Instruct-GGUF".into(),
            name: "Qwen2.5 Coder 7B".into(),
            author: "Qwen".into(),
            architecture: Some("Qwen2.5".into()),
            parameter_count: Some(7_000_000_000),
            context_length: Some(32_768),
            task: "کدنویسی".into(),
            variants: vec![ModelVariant {
                id: "Q4_K_M".into(),
                format: "GGUF".into(),
                quantization: Some("Q4_K_M".into()),
                file_size: 4_700_000_000,
                estimated_ram: 6_500_000_000,
                estimated_vram: 5_100_000_000,
            }],
            downloads: None,
            likes: None,
            tags: vec!["gguf".into(), "coding".into()],
        },
    ]
}
