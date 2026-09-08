use crate::hardware::HardwareProfile;
use crate::models::{ModelDescriptor, ModelVariant};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Recommendation { pub model_id:String, pub variant_id:String, pub runtime:String, pub backend:String, pub score:u8, pub reasons:Vec<String>, pub warnings:Vec<String> }

pub fn recommend(hw:&HardwareProfile, model:&ModelDescriptor, variant:&ModelVariant) -> Recommendation {
    let ram_ok = variant.estimated_ram <= (hw.ram_bytes as f64 * 0.82) as u64;
    let score = if ram_ok { 85 } else { 35 };
    let backend = "CPU".to_string();
    let mut warnings = Vec::new();
    if !ram_ok { warnings.push("RAM موجود برای این مدل در محدوده امن نیست.".into()); }
    Recommendation { model_id:model.id.clone(), variant_id:variant.id.clone(), runtime:"llama.cpp".into(), backend, score, reasons:vec!["فرمت GGUF برای llama.cpp مناسب است.".into(), if ram_ok {"حافظه سیستم در محدوده قابل قبول است.".into()} else {"پیکربندی سبک‌تر پیشنهاد می‌شود.".into()}], warnings }
}
