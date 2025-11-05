use std::collections::HashMap;
use std::time::{Duration, Instant};
use iced::widget::{column, row, scrollable, Space};
use iced::{executor, Application, Command, Element, Length, Subscription, Theme, Color};
use sysinfo::{System, Networks};

use crate::platform;
use crate::models::*;
use crate::suggestions::make_suggestions;
use crate::system_monitor::{bytes_per_sec, total_disk_bytes, total_net_bytes};
use crate::graphs::graph_card;
use crate::view::*;

const TICK: Duration = Duration::from_millis(700);

#[derive(Debug)]
pub struct ProcMonApp {
    sys: System,
    networks: Networks,
    last_io: HashMap<i32, IoSnapshot>,
    last_net: (u64, u64),
    last_disk: (u64, u64),
    last_ts: Instant,

    procs: Vec<ProcRow>,
    graphs: SystemGraphs,
    settings: SettingsModel,
    suggestions: Vec<Suggestion>,
}

impl Application for ProcMonApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut sys = System::new_all();
        sys.refresh_all();

        let mut app = ProcMonApp {
            sys,
            networks: Networks::new_with_refreshed_list(),
            last_io: HashMap::new(),
            last_net: (0, 0),
            last_disk: (0, 0),
            last_ts: Instant::now(),
            procs: Vec::new(),
            graphs: SystemGraphs::default(),
            settings: SettingsModel {
                thresholds: Thresholds {
                    cpu_percent: 85,
                    mem_percent: 90,
                },
                alerts_on_cpu: true,
                alerts_on_mem: true,
                sort_key: SortKey::Cpu,
                sort_dir: SortDir::Desc,
                ..Default::default()
            },
            suggestions: Vec::new(),
        };

        app.refresh_now();
        (app, Command::none())
    }

    fn title(&self) -> String {
        "Process Monitor and Manager".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Tick => self.refresh_now(),
            Message::FilterChanged(s) => self.settings.filter = s,
            Message::SortBy(k) => {
                if self.settings.sort_key == k {
                    self.settings.sort_dir = match self.settings.sort_dir {
                        SortDir::Asc => SortDir::Desc,
                        SortDir::Desc => SortDir::Asc,
                    };
                } else {
                    self.settings.sort_key = k;
                    self.settings.sort_dir = SortDir::Desc;
                }
            }
            Message::Kill(pid) => { let _ = platform::kill(pid); }
            Message::Suspend(pid) => { let _ = platform::suspend(pid); }
            Message::Resume(pid) => { let _ = platform::resume(pid); }
            Message::Boost(pid) => { let _ = platform::priority_boost(pid); }
            Message::Lower(pid) => { let _ = platform::priority_lower(pid); }
            Message::StartChanged(s) => self.settings.cmd_to_start = s,
            Message::StartNow => { let _ = platform::start(&self.settings.cmd_to_start); }
            Message::CpuAlertChanged(v) => self.settings.alerts_on_cpu = v,
            Message::MemAlertChanged(v) => self.settings.alerts_on_mem = v,
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        iced::time::every(TICK).map(|_| Message::Tick)
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let controls = controls_row(&self.settings);
        let header = table_header(&self.settings);
        let top = top_bar(self.procs.len());

        // Process rows
        let rows = self.filtered_sorted_rows()
            .into_iter()
            .map(|p| process_row(&p));

        let table = scrollable(column(rows).spacing(2)).height(Length::FillPortion(3));

        // Graphs
        let graphs = row![
            graph_card("CPU", &self.graphs.cpu, Color::from_rgb(1.0, 0.3, 0.3)),
            graph_card("Mem", &self.graphs.mem, Color::from_rgb(0.3, 1.0, 0.3)),
            graph_card("Disk R", &self.graphs.disk_read, Color::from_rgb(0.3, 0.8, 1.0)),
            graph_card("Disk W", &self.graphs.disk_write, Color::from_rgb(1.0, 0.8, 0.3)),
            graph_card("Net RX", &self.graphs.net_rx, Color::from_rgb(1.0, 0.5, 1.0)),
            graph_card("Net TX", &self.graphs.net_tx, Color::from_rgb(0.8, 0.3, 1.0)),
        ]
        .spacing(12)
        .height(Length::FillPortion(1));

        let alerts = alert_controls(&self.settings);
        let sugg = suggestions_view(&self.suggestions);

        column![
            top,
            Space::with_height(4),
            controls,
            Space::with_height(8),
            header,
            Space::with_height(8),
            table,
            Space::with_height(16),
            graphs,
            Space::with_height(8),
            alerts,
            Space::with_height(4),
            sugg
        ]
        .spacing(8)
        .padding(12)
        .into()
    }
}

impl ProcMonApp {
    fn refresh_now(&mut self) {
        self.sys.refresh_all();
        self.networks.refresh();

        let now = Instant::now();
        let dt = now.duration_since(self.last_ts).as_secs_f32().max(0.001);
        self.last_ts = now;

        // System graphs
        let total_cpu = self.sys.global_cpu_info().cpu_usage();
        let used_mem = self.sys.used_memory();
        let total_mem = self.sys.total_memory().max(1);
        let mem_pct = (used_mem as f32) * 100.0 / (total_mem as f32);

        let (disk_r_total, disk_w_total) = total_disk_bytes(&self.sys);
        let disk_r_bps = bytes_per_sec(self.last_disk.0, disk_r_total, dt);
        let disk_w_bps = bytes_per_sec(self.last_disk.1, disk_w_total, dt);
        self.last_disk = (disk_r_total, disk_w_total);

        let (net_rx_total, net_tx_total) = total_net_bytes(&self.networks);
        let net_rx_bps = bytes_per_sec(self.last_net.0, net_rx_total, dt);
        let net_tx_bps = bytes_per_sec(self.last_net.1, net_tx_total, dt);
        self.last_net = (net_rx_total, net_tx_total);

        self.graphs.cpu.push(total_cpu);
        self.graphs.mem.push(mem_pct);
        self.graphs.disk_read.push(disk_r_bps as f32);
        self.graphs.disk_write.push(disk_w_bps as f32);
        self.graphs.net_rx.push(net_rx_bps as f32);
        self.graphs.net_tx.push(net_tx_bps as f32);

        // Process table
        let mut rows: Vec<ProcRow> = Vec::with_capacity(self.sys.processes().len());
        for (pid, proc_) in self.sys.processes() {
            let pid_i32 = pid.as_u32() as i32;
            let name = proc_.name().to_string();
            let cpu = proc_.cpu_usage();
            let mem_bytes = proc_.memory();
            let io = proc_.disk_usage();
            let prev = self
                .last_io
                .get(&pid_i32)
                .copied()
                .unwrap_or(IoSnapshot {
                    read: io.total_read_bytes,
                    write: io.total_written_bytes,
                });
            let read_bps = bytes_per_sec(prev.read, io.total_read_bytes, dt) as u64;
            let write_bps = bytes_per_sec(prev.write, io.total_written_bytes, dt) as u64;
            self.last_io.insert(
                pid_i32,
                IoSnapshot {
                    read: io.total_read_bytes,
                    write: io.total_written_bytes,
                },
            );

            rows.push(ProcRow {
                pid: pid_i32,
                name,
                cpu,
                mem_bytes,
                read_bps,
                write_bps,
            });
        }
        self.procs = rows;

        // Suggestions
        self.suggestions = make_suggestions(
            &self.procs,
            if self.settings.alerts_on_cpu { total_cpu } else { 0.0 },
            if self.settings.alerts_on_mem { mem_pct } else { 0.0 },
        );
    }

    fn filtered_sorted_rows(&self) -> Vec<ProcRow> {
        let mut v: Vec<ProcRow> = self.procs.iter().cloned().collect();
        let filt = self.settings.filter.trim().to_lowercase();
        if !filt.is_empty() {
            v.retain(|p| {
                p.name.to_lowercase().contains(&filt) || p.pid.to_string().contains(&filt)
            });
        }
        v.sort_by(|a, b| {
            use std::cmp::Ordering::*;
            let ord = match self.settings.sort_key {
                SortKey::Pid => a.pid.cmp(&b.pid),
                SortKey::Name => a.name.cmp(&b.name),
                SortKey::Cpu => a.cpu.partial_cmp(&b.cpu).unwrap_or(Equal),
                SortKey::Mem => a.mem_bytes.cmp(&b.mem_bytes),
                SortKey::Read => a.read_bps.cmp(&b.read_bps),
                SortKey::Write => a.write_bps.cmp(&b.write_bps),
            };
            match self.settings.sort_dir {
                SortDir::Asc => ord,
                SortDir::Desc => ord.reverse(),
            }
        });
        v
    }
}