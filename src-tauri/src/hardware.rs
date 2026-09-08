use serde::Serialize;
use std::process::Command;
use sysinfo::System;

#[derive(Debug, Clone, Serialize)]
pub struct HardwareProfile {
    pub os: String,
    pub os_version: String,
    pub architecture: String,
    pub cpu: String,
    pub physical_cores: usize,
    pub logical_cores: usize,
    pub ram_bytes: u64,
    pub available_ram_bytes: u64,
    pub gpus: Vec<GpuInfo>,
    pub accelerators: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GpuInfo {
    pub name: String,
    pub vendor: String,
    pub vram_bytes: u64,
    pub driver: Option<String>,
}

fn command_output(program: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(program).args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn detect_nvidia() -> Vec<GpuInfo> {
    let Some(text) = command_output(
        "nvidia-smi",
        &[
            "--query-gpu=name,memory.total,driver_version",
            "--format=csv,noheader,nounits",
        ],
    ) else {
        return Vec::new();
    };

    text.lines()
        .filter_map(|line| {
            let parts: Vec<_> = line.split(',').map(str::trim).collect();
            if parts.len() < 3 {
                return None;
            }
            let mib = parts[1].parse::<u64>().ok()?;
            Some(GpuInfo {
                name: parts[0].to_string(),
                vendor: "NVIDIA".into(),
                vram_bytes: mib * 1024 * 1024,
                driver: Some(parts[2].to_string()),
            })
        })
        .collect()
}

fn detect_apple_gpu() -> Vec<GpuInfo> {
    if !cfg!(target_os = "macos") {
        return Vec::new();
    }
    let Some(text) = command_output("system_profiler", &["SPDisplaysDataType"]) else {
        return Vec::new();
    };
    let name = text
        .lines()
        .find_map(|line| line.trim().strip_prefix("Chipset Model:"))
        .map(str::trim)
        .unwrap_or("Apple GPU");
    vec![GpuInfo {
        name: name.to_string(),
        vendor: "Apple".into(),
        vram_bytes: 0,
        driver: None,
    }]
}

pub fn scan() -> HardwareProfile {
    let mut system = System::new_all();
    system.refresh_all();

    let cpu = system
        .cpus()
        .first()
        .map(|c| c.brand().trim().to_string())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "نامشخص".into());

    let mut gpus = detect_nvidia();
    if gpus.is_empty() {
        gpus = detect_apple_gpu();
    }

    let mut accelerators = Vec::new();
    if gpus.iter().any(|g| g.vendor == "NVIDIA") {
        accelerators.push("CUDA".into());
    }
    if cfg!(target_os = "macos") && gpus.iter().any(|g| g.vendor == "Apple") {
        accelerators.push("Metal".into());
    }
    if which::which(if cfg!(target_os = "windows") { "vulkaninfo.exe" } else { "vulkaninfo" }).is_ok() {
        accelerators.push("Vulkan".into());
    }
    accelerators.push("CPU".into());

    HardwareProfile {
        os: System::name().unwrap_or_else(|| "نامشخص".into()),
        os_version: System::os_version().unwrap_or_else(|| "نامشخص".into()),
        architecture: std::env::consts::ARCH.into(),
        cpu,
        physical_cores: system.physical_core_count().unwrap_or(0),
        logical_cores: system.cpus().len(),
        ram_bytes: system.total_memory(),
        available_ram_bytes: system.available_memory(),
        gpus,
        accelerators,
    }
}
