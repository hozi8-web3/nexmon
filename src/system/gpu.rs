use std::collections::VecDeque;
use std::process::Command;

#[derive(Clone)]
pub struct GpuInfo {
    pub name: String,
    pub usage: f32,
    pub mem_used_mb: f64,
    pub mem_total_mb: f64,
    pub temp_c: u32,
    pub history: VecDeque<u64>,
}

pub fn get_gpu_info() -> Option<Vec<GpuInfo>> {
    // Run nvidia-smi with CSV query format:
    // index, name, utilization.gpu, memory.used, memory.total, temperature.gpu
    let output = Command::new("nvidia-smi")
        .args([
            "--query-gpu=index,name,utilization.gpu,memory.used,memory.total,temperature.gpu",
            "--format=csv,noheader,nounits",
        ])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut gpus = Vec::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        if parts.len() != 6 {
            continue;
        }

        let name = parts[1].to_string();
        let usage: f32 = parts[2].parse().unwrap_or(0.0);
        let mem_used_mb: f64 = parts[3].parse().unwrap_or(0.0);
        let mem_total_mb: f64 = parts[4].parse().unwrap_or(0.0);
        let temp_c: u32 = parts[5].parse().unwrap_or(0);

        let mut history = VecDeque::with_capacity(60);
        for _ in 0..60 {
            history.push_back(0);
        }

        gpus.push(GpuInfo {
            name,
            usage,
            mem_used_mb,
            mem_total_mb,
            temp_c,
            history,
        });
    }

    if gpus.is_empty() {
        None
    } else {
        Some(gpus)
    }
}
