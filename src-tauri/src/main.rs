mod core;
mod hardware;
mod models;
mod recommendation;
mod runtime;
mod storage;

use serde::Serialize;
use std::sync::Mutex;
use tauri::State;

#[derive(Default)]
struct AppState { running: Mutex<Vec<String>> }

#[derive(Serialize)]
struct RuntimeSummary { id:String, installed:bool, healthy:bool }

#[tauri::command]
fn scan_hardware() -> hardware::HardwareProfile { hardware::scan() }

#[tauri::command]
fn list_models() -> Vec<models::ModelDescriptor> { models::catalog() }

#[tauri::command]
fn recommend_model(model_id:String, variant_id:String) -> Option<recommendation::Recommendation> {
    let hw=hardware::scan();
    let model=models::catalog().into_iter().find(|m|m.id==model_id)?;
    let variant=model.variants.iter().find(|v|v.id==variant_id)?;
    Some(recommendation::recommend(&hw,&model,variant))
}

#[tauri::command]
fn runtime_status() -> Vec<RuntimeSummary> {
    runtime::detect_all().into_iter().map(|r|RuntimeSummary{id:r.id,installed:r.installed,healthy:r.healthy}).collect()
}

#[tauri::command]
fn running_models(state:State<AppState>)->Vec<String>{state.running.lock().unwrap().clone()}

#[tauri::command]
fn start_model(name:String,state:State<AppState>)->bool{let mut r=state.running.lock().unwrap();if !r.contains(&name){r.push(name)}true}

#[tauri::command]
fn stop_model(name:String,state:State<AppState>)->bool{let mut r=state.running.lock().unwrap();r.retain(|x|x!=&name);true}

fn main(){
    let db_path="localai-studio.db";
    let _=storage::initialize(db_path);
    tauri::Builder::default().manage(AppState::default())
      .invoke_handler(tauri::generate_handler![scan_hardware,list_models,recommend_model,runtime_status,running_models,start_model,stop_model])
      .run(tauri::generate_context!()).expect("اجرای LocalAI Studio ناموفق بود");
}
