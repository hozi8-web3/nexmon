use ratatui::widgets::TableState;
use std::collections::VecDeque;
use std::time::Duration;
use sysinfo::{Networks, System};

#[derive(PartialEq)]
pub enum SortColumn {
    Pid,
    Name,
    Cpu,
    Memory,
}

pub struct CpuHistory {
    pub core_name: String,
    pub usage: f32,
    pub history: VecDeque<u64>, // Sparkline data expects u64
}

pub struct NetworkHistory {
    pub interface_name: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_history: VecDeque<u64>,
    pub tx_history: VecDeque<u64>,
}

use crate::system::gpu::GpuInfo;

pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
    pub status: String,
}

pub struct AppState {
    pub system: System,
    pub networks: Networks,
    pub cpu_history: Vec<CpuHistory>,
    pub gpus: Option<Vec<GpuInfo>>,
    pub overall_cpu: f32,
    pub network_history: Vec<NetworkHistory>,
    pub processes: Vec<ProcessInfo>,
    pub sort_column: SortColumn,
    pub sort_ascending: bool,
    pub selected_process: usize,
    pub process_table_state: TableState,
    pub tick_rate: Duration,
    pub should_quit: bool,
    pub search_mode: bool,
    pub search_query: String,
    pub show_loopback: bool,
    pub max_processes: usize,
}

impl AppState {
    pub fn new(tick_rate: u64, show_loopback: bool, max_processes: usize, sort: String) -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        let sort_column = match sort.to_lowercase().as_str() {
            "pid" => SortColumn::Pid,
            "name" => SortColumn::Name,
            "mem" | "memory" => SortColumn::Memory,
            _ => SortColumn::Cpu,
        };

        Self {
            system,
            networks: Networks::new_with_refreshed_list(),
            cpu_history: Vec::new(),
            gpus: None,
            overall_cpu: 0.0,
            network_history: Vec::new(),
            processes: Vec::new(),
            sort_column,
            sort_ascending: false,
            selected_process: 0,
            process_table_state: TableState::default(),
            tick_rate: Duration::from_millis(tick_rate),
            should_quit: false,
            search_mode: false,
            search_query: String::new(),
            show_loopback,
            max_processes,
        }
    }

    pub fn next_process(&mut self) {
        if self.processes.is_empty() {
            return;
        }
        self.selected_process = (self.selected_process + 1).min(self.processes.len() - 1);
        self.process_table_state.select(Some(self.selected_process));
    }

    pub fn previous_process(&mut self) {
        if self.processes.is_empty() {
            return;
        }
        self.selected_process = self.selected_process.saturating_sub(1);
        self.process_table_state.select(Some(self.selected_process));
    }
}
