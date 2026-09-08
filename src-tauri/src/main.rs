mod core;
mod hardware;
mod models;
mod recommendation;
mod runtime;
mod storage;

use serde::Serialize;
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
fn recommend_model(
    model_id: String,
    variant_id: String,
) -> Option<recommendation::Recommendation> {
    let hw = hardware::scan();
    let model = models::catalog().into_iter().find(|m| m.id == model_id)?;
    let variant = model.variants.iter().find(|v| v.id == variant_id)?;
    Some(recommendation::recommend(&hw, &model, variant))
}

#[tauri::command]
fn smart_install_plan(
    model_id: String,
    variant_id: String,
) -> Result<core::InstallationPlan, String> {
    let hw = hardware::scan();
    let model = models::catalog()
        .into_iter()
        .find(|m| m.id == model_id)
        .ok_or_else(|| "مدل در کاتالوگ محلی پیدا نشد.".to_string())?;
    let variant = model
        .variants
        .iter()
        .find(|v| v.id == variant_id)
        .ok_or_else(|| "Variant انتخاب‌شده پیدا نشد.".to_string())?;
    Ok(core::create_install_plan(&hw, &model, variant))
}

#[tauri::command]
fn runtime_status() -> Vec<runtime::RuntimeStatus> {
    runtime::detect_all()
}

#[tauri::command]
fn running_models(state: State<AppState>) -> Vec<RunningModel> {
    let mut running = state.running.lock().expect("running mutex poisoned");
    running.retain(|_, child| child.try_wait().ok().flatten().is_none());
    running
        .iter()
        .map(|(id, child)| RunningModel {
            id: id.clone(),
            pid: child.id(),
        })
        .collect()
}

#[tauri::command]
fn start_model(
    model_path: String,
    config: Option<core::InferenceConfig>,
    state: State<AppState>,
) -> Result<RunningModel, String> {
    let path = PathBuf::from(&model_path);
    let canonical = path
        .canonicalize()
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
    let id = PathBuf::from(&model_path)
        .canonicalize()
        .map(|p| p.display().to_string())
        .unwrap_or(model_path);
    let mut running = state.running.lock().map_err(|_| "قفل Process Manager خراب است.".to_string())?;
    if let Some(mut child) = running.remove(&id) {
        runtime::stop_child(&mut child)?;
        return Ok(true);
    }
    Ok(false)
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .setup(|app| {
            let data_dir = app
                .path()
                .app_data_dir()
                .map_err(|e| format!("مسیر داده برنامه در دسترس نیست: {e}"))?;
            std::fs::create_dir_all(&data_dir)?;
            storage::initialize(&data_dir.join("localai-studio.db"))?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            scan_hardware,
            list_models,
            search_models,
            recommend_model,
            smart_install_plan,
            runtime_status,
            running_models,
            start_model,
            stop_model
        ])
        .run(tauri::generate_context!())
        .expect("اجرای LocalAI Studio ناموفق بود");
}
