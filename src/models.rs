use std::collections::VecDeque;

pub const GRAPH_POINTS: usize = 120;

// -------- Sorting --------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortKey {
    Pid,
    Name,
    Cpu,
    Mem,
    Read,
    Write,
}

impl Default for SortKey {
    fn default() -> Self {
        SortKey::Cpu
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDir {
    Asc,
    Desc,
}

impl Default for SortDir {
    fn default() -> Self {
        SortDir::Desc
    }
}

// -------- Messages --------
#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    FilterChanged(String),
    SortBy(SortKey),
    Kill(i32),
    Suspend(i32),
    Resume(i32),
    Boost(i32),
    Lower(i32),
    StartChanged(String),
    StartNow,
    CpuAlertChanged(bool),
    MemAlertChanged(bool),
}

// -------- Models --------
#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize)]
pub struct Thresholds {
    pub cpu_percent: u8,
    pub mem_percent: u8,
}

#[derive(Debug, Clone, Default)]
pub struct Suggestion {
    pub title: String,
    pub detail: String,
}

#[derive(Debug, Clone, Default)]
pub struct SettingsModel {
    pub filter: String,
    pub sort_key: SortKey,
    pub sort_dir: SortDir,
    pub cmd_to_start: String,
    pub alerts_on_cpu: bool,
    pub alerts_on_mem: bool,
    pub thresholds: Thresholds,
}

#[derive(Debug, Clone, Default)]
pub struct GraphSeries {
    pub points: VecDeque<f32>,
}

impl GraphSeries {
    pub fn push(&mut self, v: f32) {
        if self.points.len() >= GRAPH_POINTS {
            self.points.pop_front();
        }
        self.points.push_back(v);
    }
}

#[derive(Debug, Clone, Default)]
pub struct SystemGraphs {
    pub cpu: GraphSeries,
    pub mem: GraphSeries,
    pub disk_read: GraphSeries,
    pub disk_write: GraphSeries,
    pub net_rx: GraphSeries,
    pub net_tx: GraphSeries,
}

#[derive(Debug, Clone, Default)]
pub struct ProcRow {
    pub pid: i32,
    pub name: String,
    pub cpu: f32,
    pub mem_bytes: u64,
    pub read_bps: u64,
    pub write_bps: u64,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct IoSnapshot {
    pub read: u64,
    pub write: u64,
}