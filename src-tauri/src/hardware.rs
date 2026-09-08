use serde::Serialize;
use sysinfo::System;

#[derive(Debug, Clone, Serialize)]
pub struct HardwareProfile {
    pub os: String,
    pub cpu: String,
    pub physical_cores: usize,
    pub logical_cores: usize,
    pub ram_bytes: u64,
    pub gpus: Vec<GpuInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GpuInfo { pub name: String, pub vendor: String, pub vram_bytes: u64 }

pub fn scan() -> HardwareProfile {
    let mut system = System::new_all();
    system.refresh_all();
    let cpu = system.cpus().first().map(|c| c.brand().to_string()).unwrap_or_else(|| "نامشخص".into());
    HardwareProfile {
        os: System::name().unwrap_or_else(|| "نامشخص".into()),
        cpu,
        physical_cores: system.physical_core_count().unwrap_or(0),
        logical_cores: system.cpus().len(),
        ram_bytes: system.total_memory(),
        gpus: Vec::new(),
    }
}
