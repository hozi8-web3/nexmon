use crate::app::{AppState, CpuHistory, NetworkHistory, ProcessInfo, SortColumn};
use std::collections::VecDeque;

pub fn refresh(app: &mut AppState) {
    app.system.refresh_all();
    
    // CPU
    let cpus = app.system.cpus();
    if app.cpu_history.is_empty() {
        for cpu in cpus {
            let mut history = VecDeque::with_capacity(60);
            for _ in 0..60 { history.push_back(0); }
            app.cpu_history.push(CpuHistory {
                core_name: cpu.name().to_string(),
                usage: 0.0,
                history,
            });
        }
    }

    let mut total_cpu = 0.0;
    for (i, cpu) in app.system.cpus().iter().enumerate() {
        if let Some(history) = app.cpu_history.get_mut(i) {
            history.usage = cpu.cpu_usage();
            total_cpu += history.usage;
            if history.history.len() >= 60 {
                history.history.pop_front();
            }
            history.history.push_back(history.usage as u64);
        }
    }
    app.overall_cpu = if app.system.cpus().is_empty() { 0.0 } else { total_cpu / app.system.cpus().len() as f32 };

    // Network
    for (name, network) in app.system.networks() {
        if !app.show_loopback && name.starts_with("lo") {
            continue;
        }
        
        let rx_delta = network.received();
        let tx_delta = network.transmitted();

        if let Some(net_hist) = app.network_history.iter_mut().find(|n| n.interface_name == *name) {
            net_hist.rx_bytes = rx_delta;
            net_hist.tx_bytes = tx_delta;
            if net_hist.rx_history.len() >= 60 { net_hist.rx_history.pop_front(); }
            if net_hist.tx_history.len() >= 60 { net_hist.tx_history.pop_front(); }
            net_hist.rx_history.push_back(rx_delta);
            net_hist.tx_history.push_back(tx_delta);
        } else {
            let mut rx_history = VecDeque::with_capacity(60);
            let mut tx_history = VecDeque::with_capacity(60);
            for _ in 0..60 { rx_history.push_back(0); tx_history.push_back(0); }
            app.network_history.push(NetworkHistory {
                interface_name: name.to_string(),
                rx_bytes: rx_delta,
                tx_bytes: tx_delta,
                rx_history,
                tx_history,
            });
        }
    }

    // Processes
    app.processes.clear();
    for (pid, process) in app.system.processes() {
        app.processes.push(ProcessInfo {
            pid: pid.as_u32(),
            name: process.name().to_string(), // use name() as fallback if available, it's string slice
            cpu_usage: process.cpu_usage(),
            memory: process.memory(),
            status: format!("{:?}", process.status()),
        });
    }

    // Apply search filter
    if !app.search_query.is_empty() {
        let query = app.search_query.to_lowercase();
        app.processes.retain(|p| p.name.to_lowercase().contains(&query));
    }

    // Sort
    match app.sort_column {
        SortColumn::Pid => app.processes.sort_by(|a, b| a.pid.cmp(&b.pid)),
        SortColumn::Name => app.processes.sort_by(|a, b| a.name.cmp(&b.name)),
        SortColumn::Cpu => app.processes.sort_by(|a, b| a.cpu_usage.partial_cmp(&b.cpu_usage).unwrap_or(std::cmp::Ordering::Equal)),
        SortColumn::Memory => app.processes.sort_by(|a, b| a.memory.cmp(&b.memory)),
    }

    if !app.sort_ascending {
        app.processes.reverse();
    }

    // Truncate
    if app.processes.len() > app.max_processes {
        app.processes.truncate(app.max_processes);
    }
}
