use crate::core::InferenceConfig;
use reqwest::blocking::Client;
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::Duration;

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeStatus {
    pub id: String,
    pub installed: bool,
    pub healthy: bool,
    pub binary: Option<String>,
    pub version: Option<String>,
    pub capabilities: Vec<String>,
}

fn version_of(binary: &Path) -> Option<String> {
    let output = Command::new(binary).arg("--version").output().ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    let text = if stdout.is_empty() { stderr } else { stdout };
    (!text.is_empty()).then_some(text.lines().next().unwrap_or(&text).to_string())
}

pub fn detect_llama_server() -> Option<PathBuf> {
    let candidates = if cfg!(target_os = "windows") {
        ["llama-server.exe", "server.exe"]
    } else {
        ["llama-server", "server"]
    };
    candidates
        .into_iter()
        .find_map(|name| which::which(name).ok())
}

pub fn detect_llama_cli() -> Option<PathBuf> {
    let candidates = if cfg!(target_os = "windows") {
        ["llama-cli.exe", "main.exe"]
    } else {
        ["llama-cli", "main"]
    };
    candidates
        .into_iter()
        .find_map(|name| which::which(name).ok())
}

pub fn health_check(port: u16) -> bool {
    Client::builder()
        .timeout(Duration::from_secs(3))
        .build()
        .ok()
        .and_then(|client| {
            client
                .get(format!("http://127.0.0.1:{port}/health"))
                .send()
                .ok()
        })
        .map(|response| response.status().is_success())
        .unwrap_or(false)
}

pub fn detect_all() -> Vec<RuntimeStatus> {
    let binary = detect_llama_server().or_else(detect_llama_cli);
    let installed = binary.is_some();
    let version = binary.as_deref().and_then(version_of);
    let healthy = installed && health_check(1234);
    vec![RuntimeStatus {
        id: "llama.cpp".into(),
        installed,
        healthy,
        binary: binary.as_ref().map(|p| p.display().to_string()),
        version,
        capabilities: vec![
            "GGUF".into(),
            "CPU".into(),
            "GPU offload".into(),
            "OpenAI API".into(),
        ],
    }]
}

pub fn launch_server(model: &Path, config: &InferenceConfig, port: u16) -> Result<Child, String> {
    if !model.is_file() {
        return Err("فایل مدل پیدا نشد یا مسیر معتبر نیست.".into());
    }
    if model
        .extension()
        .and_then(|x| x.to_str())
        .map(|x| x.eq_ignore_ascii_case("gguf"))
        != Some(true)
    {
        return Err("در نسخه v1.0 اجرای مستقیم فقط برای فایل GGUF فعال است.".into());
    }
    if health_check(port) {
        return Err(format!("پورت {port} در حال استفاده است."));
    }
    let binary = detect_llama_server().ok_or_else(|| {
        "llama-server پیدا نشد. ابتدا llama.cpp را نصب و مسیر Binary را در PATH قرار دهید."
            .to_string()
    })?;
    let threads = if config.threads == 0 {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
    } else {
        config.threads as usize
    };
    let mut command = Command::new(binary);
    command
        .arg("-m")
        .arg(model)
        .arg("-c")
        .arg(config.context_length.to_string())
        .arg("-ngl")
        .arg(config.gpu_layers.to_string())
        .arg("-t")
        .arg(threads.to_string())
        .arg("-b")
        .arg(config.batch_size.to_string())
        .arg("--temp")
        .arg(config.temperature.to_string())
        .arg("--top-p")
        .arg(config.top_p.to_string())
        .arg("--top-k")
        .arg(config.top_k.to_string())
        .arg("--repeat-penalty")
        .arg(config.repeat_penalty.to_string())
        .arg("--host")
        .arg("127.0.0.1")
        .arg("--port")
        .arg(port.to_string())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    if config.flash_attention {
        command.arg("--flash-attn");
    }
    command
        .spawn()
        .map_err(|e| format!("اجرای llama.cpp ناموفق بود: {e}"))
}

pub fn stop_child(child: &mut Child) -> Result<(), String> {
    child
        .kill()
        .map_err(|e| format!("توقف Runtime ناموفق بود: {e}"))?;
    let _ = child.wait();
    Ok(())
}
