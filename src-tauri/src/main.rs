mod core;
mod download;
mod hardware;
mod models;
mod recommendation;
mod runtime;
mod storage;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Child;
use std::sync::Mutex;
use tauri::{Manager, State};

#[derive(Default)]
struct AppState {
    running: Mutex<HashMap<String, Child>>,
}

#[derive(Serialize)]
struct RunningModel {
    id: String,
    pid: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[tauri::command]
fn scan_hardware() -> hardware::HardwareProfile {
    hardware::scan()
}

#[tauri::command]
fn list_models() -> Vec<models::ModelDescriptor> {
    models::catalog()
}

#[tauri::command]
fn search_models(query: String, limit: Option<usize>) -> Result<Vec<models::ModelDescriptor>, String> {
    models::search_huggingface(query.trim(), limit.unwrap_or(12))
}

#[tauri::command]
fn recommend_model(model_id: String, variant_id: String) -> Option<recommendation::Recommendation> {
    let hw = hardware::scan();
    let model = models::catalog().into_iter().find(|m| m.id == model_id)?;
    let variant = model.variants.iter().find(|v| v.id == variant_id)?;
    Some(recommendation::recommend(&hw, &model, variant))
}

#[tauri::command]
fn smart_install_plan(model_id: String, variant_id: String) -> Result<core::InstallationPlan, String> {
    let hw = hardware::scan();
    let model = models::catalog().into_iter().find(|m| m.id == model_id)
        .ok_or_else(|| "مدل در کاتالوگ محلی پیدا نشد.".to_string())?;
    let variant = model.variants.iter().find(|v| v.id == variant_id)
        .ok_or_else(|| "Variant انتخاب‌شده پیدا نشد.".to_string())?;
    Ok(core::create_install_plan(&hw, &model, variant))
}

#[tauri::command]
fn download_model(url: String, destination: String, expected_sha256: Option<String>) -> Result<download::DownloadResult, String> {
    download::download(&url, &PathBuf::from(destination), expected_sha256.as_deref())
}

#[tauri::command]
fn runtime_status() -> Vec<runtime::RuntimeStatus> {
    runtime::detect_all()
}

#[tauri::command]
fn runtime_health(port: Option<u16>) -> bool {
    runtime::health_check(port.unwrap_or(1234))
}

#[tauri::command]
fn running_models(state: State<AppState>) -> Vec<RunningModel> {
    let mut running = state.running.lock().expect("running mutex poisoned");
    running.retain(|_, child| child.try_wait().ok().flatten().is_none());
    running.iter().map(|(id, child)| RunningModel { id: id.clone(), pid: child.id() }).collect()
}

#[tauri::command]
fn start_model(model_path: String, config: Option<core::InferenceConfig>, state: State<AppState>) -> Result<RunningModel, String> {
    let canonical = PathBuf::from(&model_path).canonicalize()
        .map_err(|_| "مسیر مدل معتبر نیست یا فایل در دسترس نیست.".to_string())?;
    let id = canonical.display().to_string();
    let mut running = state.running.lock().map_err(|_| "قفل Process Manager خراب است.".to_string())?;
    if let Some(child) = running.get_mut(&id) {
        if child.try_wait().map_err(|e| e.to_string())?.is_none() {
            return Ok(RunningModel { id, pid: child.id() });
        }
    }
    let child = runtime::launch_server(&canonical, &config.unwrap_or_default(), 1234)?;
    let pid = child.id();
    running.insert(id.clone(), child);
    Ok(RunningModel { id, pid })
}

#[tauri::command]
fn stop_model(model_path: String, state: State<AppState>) -> Result<bool, String> {
    let id = PathBuf::from(&model_path).canonicalize().map(|p| p.display().to_string()).unwrap_or(model_path);
    let mut running = state.running.lock().map_err(|_| "قفل Process Manager خراب است.".to_string())?;
    if let Some(mut child) = running.remove(&id) {
        runtime::stop_child(&mut child)?;
        return Ok(true);
    }
    Ok(false)
}

#[tauri::command]
fn chat_completion(messages: Vec<ChatMessage>) -> Result<String, String> {
    if messages.is_empty() { return Err("حداقل یک پیام لازم است.".into()); }
    let client = reqwest::blocking::Client::builder().timeout(std::time::Duration::from_secs(180)).build()
        .map_err(|e| format!("ساخت کلاینت گفتگو ناموفق بود: {e}"))?;
    let response = client.post("http://127.0.0.1:1234/v1/chat/completions")
        .json(&serde_json::json!({"model":"local-model","messages":messages,"stream":false}))
        .send().and_then(|r| r.error_for_status())
        .map_err(|e| format!("Runtime پاسخ نداد: {e}"))?;
    let body: ChatResponse = response.json().map_err(|e| format!("پاسخ Runtime قابل خواندن نیست: {e}"))?;
    body.choices.into_iter().next().map(|c| c.message.content)
        .ok_or_else(|| "Runtime پاسخ متنی برنگرداند.".into())
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .setup(|app| {
            let data_dir = app.path().app_data_dir().map_err(|e| format!("مسیر داده برنامه در دسترس نیست: {e}"))?;
            std::fs::create_dir_all(&data_dir)?;
            storage::initialize(&data_dir.join("localai-studio.db"))?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            scan_hardware, list_models, search_models, recommend_model, smart_install_plan,
            download_model, runtime_status, runtime_health, running_models, start_model,
            stop_model, chat_completion
        ])
        .run(tauri::generate_context!())
        .expect("اجرای LocalAI Studio ناموفق بود");
}
