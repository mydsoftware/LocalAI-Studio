mod core;mod runtime;use serde::Serialize;use sysinfo::System;use tauri::State;use std::sync::Mutex;
#[derive(Default)]struct AppState{running:Mutex<Vec<String>>}
#[derive(Serialize)]struct Hardware{os:String,cpu:String,cores:usize,ram_gb:u64,gpus:Vec<String>}
#[tauri::command]fn scan_hardware()->Hardware{let mut s=System::new_all();s.refresh_all();let cpu=s.cpus().first().map(|c|c.brand().to_string()).unwrap_or_else(||"نامشخص".into());Hardware{os:System::name().unwrap_or_else(||"نامشخص".into()),cpu,cores:s.cpus().len(),ram_gb:s.total_memory()/1024/1024/1024,gpus:Vec::new()}}
#[tauri::command]fn runtime_status()->serde_json::Value{serde_json::json!({"llama_cpp":runtime::LlamaCppRuntime::detect().is_some(),"ollama":false,"transformers":false,"vllm":false,"mlx":false})}
#[tauri::command]fn running_models(state:State<AppState>)->Vec<String>{state.running.lock().unwrap().clone()}
#[tauri::command]fn start_model(name:String,state:State<AppState>)->bool{let mut r=state.running.lock().unwrap();if !r.contains(&name){r.push(name)}true}
#[tauri::command]fn stop_model(name:String,state:State<AppState>)->bool{let mut r=state.running.lock().unwrap();r.retain(|x|x!=&name);true}
#[tauri::command]fn install_plan(model:String,variant:String)->core::InstallationPlan{core::create_install_plan(model,variant,0,0)}
fn main(){tauri::Builder::default().manage(AppState::default()).invoke_handler(tauri::generate_handler![scan_hardware,runtime_status,running_models,start_model,stop_model,install_plan]).run(tauri::generate_context!()).expect("اجرای LocalAI Studio ناموفق بود");}
