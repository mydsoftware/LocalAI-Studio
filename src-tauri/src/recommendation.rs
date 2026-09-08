use crate::hardware::HardwareProfile;
use crate::models::{ModelDescriptor, ModelVariant};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Recommendation {
    pub model_id: String,
    pub variant_id: String,
    pub runtime: String,
    pub backend: String,
    pub score: u8,
    pub confidence: String,
    pub compatibility: String,
    pub reasons: Vec<String>,
    pub warnings: Vec<String>,
}

fn clamp_score(v: f32) -> f32 {
    v.clamp(0.0, 100.0)
}

pub fn recommend(
    hw: &HardwareProfile,
    model: &ModelDescriptor,
    variant: &ModelVariant,
) -> Recommendation {
    let safe_ram = hw.available_ram_bytes as f32 * 0.85;
    let max_vram = hw.gpus.iter().map(|g| g.vram_bytes).max().unwrap_or(0) as f32;
    let gpu_possible = max_vram > 0.0;

    let ram_ratio = if variant.estimated_ram == 0 {
        1.0
    } else {
        safe_ram / variant.estimated_ram as f32
    };
    let vram_ratio = if variant.estimated_vram == 0 {
        1.0
    } else if gpu_possible {
        max_vram / variant.estimated_vram as f32
    } else {
        0.0
    };

    let hardware = clamp_score((ram_ratio.min(1.0) * 70.0) + (vram_ratio.min(1.0) * 30.0));
    let runtime = if variant.format.eq_ignore_ascii_case("GGUF") { 100.0 } else { 35.0 };
    let performance = if gpu_possible {
        clamp_score(55.0 + vram_ratio.min(1.0) * 45.0)
    } else {
        clamp_score(45.0 + ram_ratio.min(1.0) * 35.0)
    };
    let quality = match variant.quantization.as_deref().unwrap_or("") {
        "Q8_0" | "Q6_K" => 92.0,
        "Q5_K_M" => 88.0,
        "Q4_K_M" => 82.0,
        q if q.starts_with("Q3") => 70.0,
        q if q.starts_with("Q2") => 58.0,
        _ => 75.0,
    };
    let task = if model.task.is_empty() || model.task == "عمومی" { 75.0 } else { 90.0 };

    let total = hardware * 0.30 + performance * 0.25 + quality * 0.20 + task * 0.15 + runtime * 0.10;
    let score = total.round().clamp(0.0, 100.0) as u8;

    let compatibility = if ram_ratio < 0.75 {
        "Unsupported"
    } else if ram_ratio < 1.0 || (gpu_possible && vram_ratio < 0.65) {
        "Limited"
    } else if score >= 85 {
        "Excellent"
    } else {
        "Good"
    }
    .to_string();

    let backend = if gpu_possible && vram_ratio >= 0.65 {
        if hw.accelerators.iter().any(|a| a == "CUDA") {
            "CUDA"
        } else if hw.accelerators.iter().any(|a| a == "Metal") {
            "Metal"
        } else {
            "GPU"
        }
    } else {
        "CPU"
    }
    .to_string();

    let mut reasons = vec![format!("امتیاز سازگاری سخت‌افزار: {:.0}/100", hardware)];
    if variant.format.eq_ignore_ascii_case("GGUF") {
        reasons.push("فرمت GGUF با llama.cpp سازگاری مستقیم دارد.".into());
    }
    reasons.push(format!("Backend پیشنهادی: {backend}"));

    let mut warnings = Vec::new();
    if ram_ratio < 1.0 {
        warnings.push("RAM آزاد فعلی برای حاشیه امن این Variant کافی نیست؛ Quantization سبک‌تر پیشنهاد می‌شود.".into());
    }
    if gpu_possible && vram_ratio < 1.0 {
        warnings.push("کل مدل در VRAM جا نمی‌شود؛ اجرای Hybrid CPU/GPU محتمل است.".into());
    }
    if !variant.format.eq_ignore_ascii_case("GGUF") {
        warnings.push("این Variant برای Runtime اصلی v1.0 یعنی llama.cpp مناسب نیست.".into());
    }

    Recommendation {
        model_id: model.id.clone(),
        variant_id: variant.id.clone(),
        runtime: "llama.cpp".into(),
        backend,
        score,
        confidence: if variant.file_size > 0 && variant.estimated_ram > 0 {
            "medium"
        } else {
            "low"
        }
        .into(),
        compatibility,
        reasons,
        warnings,
    }
}
