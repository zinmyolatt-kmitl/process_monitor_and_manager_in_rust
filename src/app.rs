use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

use iced::widget::{
    button, checkbox, column, container, row, scrollable, text, text_input, Space,
};
use iced::{executor, Alignment, Application, Command, Element, Length, Subscription, Theme, Color};
use iced_widget::canvas;
use sysinfo::{System, Networks};

use crate::platform;
use crate::util;

use util::fmt_bytes;

const TICK: Duration = Duration::from_millis(700);
const GRAPH_POINTS: usize = 120; // ~84 seconds at 700ms

// Base trait for shared rounded look
trait RoundedBase {
    fn base(&self, color: Color) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(color)),
            text_color: Color::WHITE,
            border: iced::Border {
                radius: 6.0.into(),
                width: 0.0,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

struct KillButton;
impl button::StyleSheet for KillButton {
    type Style = iced::Theme;
    fn active(&self, _: &Self::Style) -> button::Appearance {
        self.base(Color::from_rgb(0.6, 0.1, 0.1))
    }
    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        self.base(Color::from_rgb(0.8, 0.2, 0.2))
    }
}
impl RoundedBase for KillButton {}

struct SuspendButton;
impl button::StyleSheet for SuspendButton {
    type Style = iced::Theme;
    fn active(&self, _: &Self::Style) -> button::Appearance {
        self.base(Color::from_rgb(0.25, 0.25, 0.6))
    }
    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        self.base(Color::from_rgb(0.35, 0.35, 0.75))
    }
}
impl RoundedBase for SuspendButton {}

struct ResumeButton;
impl button::StyleSheet for ResumeButton {
    type Style = iced::Theme;
    fn active(&self, _: &Self::Style) -> button::Appearance {
        self.base(Color::from_rgb(0.2, 0.55, 0.2))
    }
    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        self.base(Color::from_rgb(0.3, 0.65, 0.3))
    }
}
impl RoundedBase for ResumeButton {}

struct BoostButton;
impl button::StyleSheet for BoostButton {
    type Style = iced::Theme;
    fn active(&self, _: &Self::Style) -> button::Appearance {
        self.base(Color::from_rgb(0.8, 0.5, 0.1))
    }
    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        self.base(Color::from_rgb(0.95, 0.6, 0.2))
    }
}
impl RoundedBase for BoostButton {}

struct LowerButton;
impl button::StyleSheet for LowerButton {
    type Style = iced::Theme;
    fn active(&self, _: &Self::Style) -> button::Appearance {
        self.base(Color::from_rgb(0.65, 0.65, 0.68))
    }
    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        self.base(Color::from_rgb(0.75, 0.75, 0.78))
    }
}
impl RoundedBase for LowerButton {}


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
enum SortDir {
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
struct Thresholds {
    cpu_percent: u8,
    mem_percent: u8,
}

#[derive(Debug, Clone, Default)]
struct Suggestion {
    title: String,
    detail: String,
}

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
struct SettingsModel {
    filter: String,
    sort_key: SortKey,
    sort_dir: SortDir,
    cmd_to_start: String,
    alerts_on_cpu: bool,
    alerts_on_mem: bool,
    thresholds: Thresholds,
}

#[derive(Debug, Clone, Default)]
struct GraphSeries {
    points: VecDeque<f32>,
}
impl GraphSeries {
    fn push(&mut self, v: f32) {
        if self.points.len() >= GRAPH_POINTS {
            self.points.pop_front();
        }
        self.points.push_back(v);
    }
}

#[derive(Debug, Clone, Default)]
struct SystemGraphs {
    cpu: GraphSeries,
    mem: GraphSeries,
    disk_read: GraphSeries,
    disk_write: GraphSeries,
    net_rx: GraphSeries,
    net_tx: GraphSeries,
}

#[derive(Debug, Clone, Default)]
struct ProcRow {
    pid: i32,
    name: String,
    cpu: f32,
    mem_bytes: u64,
    read_bps: u64,
    write_bps: u64,
}

#[derive(Debug, Default, Clone, Copy)]
struct IoSnapshot {
    read: u64,
    write: u64,
}

// -------- Application --------
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
        // Controls row
        let controls = row![
            text_input("Filter (name or PID)", &self.settings.filter)
                .on_input(Message::FilterChanged)
                .width(260.0),
            Space::with_width(10.0),
            text_input("Start command…", &self.settings.cmd_to_start)
                .on_input(Message::StartChanged)
                .on_submit(Message::StartNow)
                .width(Length::FillPortion(2)),
            button("Start").on_press(Message::StartNow),
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        // Header row
        let header = container(
            row![
                container(sortable("PID", SortKey::Pid, &self.settings)).width(70.0),
                container(sortable("Name", SortKey::Name, &self.settings)).width(450),
                container(sortable("CPU %", SortKey::Cpu, &self.settings)).width(80.0),
                container(sortable("Memory", SortKey::Mem, &self.settings)).width(110.0),
                container(sortable("Read/s", SortKey::Read, &self.settings)).width(110.0),
                container(sortable("Write/s", SortKey::Write, &self.settings)).width(110.0),
                container(text("Actions").size(18))
                    .width(Length::FillPortion(2))
                    .center_x()
                    .center_y(), 
            ]
            .spacing(20)
            .align_items(Alignment::Center)
        )
        .padding([12, 10]);
        // Process rows
        let rows = self.filtered_sorted_rows().into_iter().map(|p| {
            container(
                row![
                    text(p.pid).width(70.0),
                    text(p.name.clone()).width(450),
                    text(format!("{:.1}", p.cpu)).width(80.0),
                    text(fmt_bytes(p.mem_bytes)).width(110.0),
                    text(fmt_bytes(p.read_bps) + "/s").width(110.0),
                    text(fmt_bytes(p.write_bps) + "/s").width(110.0),
                    container(
                        row![
                        button(text("Kill").size(15))
                            .on_press(Message::Kill(p.pid))
                            .style(iced::theme::Button::Custom(Box::new(KillButton)))
                            .padding([4, 10]),
                        button(text("Suspend").size(15))
                            .on_press(Message::Suspend(p.pid))
                            .style(iced::theme::Button::Custom(Box::new(SuspendButton)))
                            .padding([4, 10]),
                        button(text("Resume").size(15))
                            .on_press(Message::Resume(p.pid))
                            .style(iced::theme::Button::Custom(Box::new(ResumeButton)))
                            .padding([4, 10]),
                        button(text("Boost").size(15))
                            .on_press(Message::Boost(p.pid))
                            .style(iced::theme::Button::Custom(Box::new(BoostButton)))
                            .padding([4, 10]),
                        button(text("Lower").size(15))
                            .on_press(Message::Lower(p.pid))
                            .style(iced::theme::Button::Custom(Box::new(LowerButton)))
                            .padding([4, 10]),
                    ]
                    .spacing(6) )
                    .padding([0, 8, 0, 0])
                    .width(Length::FillPortion(2))
                ]
                .spacing(20),
            )
            .padding([4, 10])
            .into()
        });

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

        let alert_controls = row![
            text("Alerts:").size(14),
            Space::with_width(10.0),
            checkbox("CPU", self.settings.alerts_on_cpu)
                .on_toggle(Message::CpuAlertChanged),
            Space::with_width(10.0),
            checkbox("Memory", self.settings.alerts_on_mem)
                .on_toggle(Message::MemAlertChanged),
        ]
        .align_items(Alignment::Center);

        let sugg: Element<'_, Message> = if self.suggestions.is_empty() {
            container(text("No suggestions. System looks calm.").size(16))
                .padding(8)
                .into()
        } else {
            let items = self.suggestions.iter().map(|s| {
                container(column![text(&s.title).size(16), text(&s.detail).size(14)])
                    .padding(8)
                    .into()
            });
            
            container(
                scrollable(column(items).spacing(8))
                    .height(Length::Fixed(180.0))
                    .width(Length::Fill)
            )
            .width(Length::Fill) 
            .into()
        };

        container(
        column![
                    controls, 
                    Space::with_height(8),
                    header, 
                    Space::with_height(8),
                    table, 
                    Space::with_height(16),
                    graphs, 
                    Space::with_height(8),
                    alert_controls,
                    Space::with_height(4),
                    sugg
                ]
                .spacing(8)
                .padding(12)
            )
            .into()
    }
}

// -------- Helpers --------
fn sortable<'a>(label: &str, key: SortKey, s: &SettingsModel) -> Element<'a, Message> {
    let mut caption = label.to_string();
    if s.sort_key == key {
        caption.push_str(match s.sort_dir {
            SortDir::Asc => "↑",
            SortDir::Desc => "↓",
        });
    }
    button(text(caption).size(14))
        .on_press(Message::SortBy(key))
        .width(Length::Fill)  
        .into()
}

fn sparkline<'a>(label: &str, series: &'a GraphSeries, color: iced::Color) -> Element<'a, Message> {
    use iced::{Color, Rectangle};
    use iced_widget::canvas::{Frame, Stroke};

    struct Plot<'a>(&'a VecDeque<f32>, Color);

    impl<'a> canvas::Program<Message> for Plot<'a> {
        type State = ();

        fn draw(
            &self,
            _state: &(),
            renderer: &iced::Renderer,
            _theme: &Theme,
            bounds: Rectangle,
            _cursor: iced::mouse::Cursor,
        ) -> Vec<canvas::Geometry> {
            let mut frame = Frame::new(renderer, bounds.size());
            let w = bounds.width;
            let h = bounds.height;
            let data = self.0;

            if data.len() >= 2 {
                let max = data.iter().cloned().fold(1.0, f32::max);
                let step = w / (data.len().saturating_sub(1) as f32);
                let mut builder = iced_widget::canvas::path::Builder::new();

                for (i, v) in data.iter().enumerate() {
                    let x = i as f32 * step;
                    let y = h - (v / max) * h;
                    if i == 0 {
                        builder.move_to([x, y].into());
                    } else {
                        builder.line_to([x, y].into());
                    }
                }

                let path = builder.build();
                
                let stroke = Stroke::default()
                    .with_width(2.0)
                    .with_color(self.1);
                
                frame.stroke(&path, stroke);
            }
            vec![frame.into_geometry()]
        }
    }

    let canvas = canvas(Plot(&series.points, color))
        .width(Length::Fill)
        .height(80.0);

    column![text(label).size(14), canvas]
        .spacing(4)
        .width(Length::FillPortion(1))
        .into()
}

fn graph_card<'a>(label: &str, series: &'a GraphSeries, color: Color) -> Element<'a, Message> {
    let sparkline_widget = sparkline(label, series, color);
    
    container(sparkline_widget)
        .padding(12)
        .width(Length::FillPortion(1))
        .style(|_theme: &Theme| {
            container::Appearance {
                background: Some(iced::Background::Color(Color::from_rgb(0.25, 0.25, 0.25))),
                border: iced::Border {
                    radius: 8.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .into()
}

// -------- Logic --------
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

fn make_suggestions(rows: &[ProcRow], total_cpu: f32, mem_pct: f32) -> Vec<Suggestion> {
    let mut out = Vec::new();
    if total_cpu > 80.0 {
        if let Some(top) = rows.iter().max_by(|a, b| a.cpu.total_cmp(&b.cpu)) {
            out.push(Suggestion {
                title: format!("High CPU: {} at {:.1}%", top.name, top.cpu),
                detail: format!(
                    "Consider suspending or killing PID {} if it's misbehaving.",
                    top.pid
                ),
            });
        }
    }
    if mem_pct > 20.0 {
        if let Some(top) = rows.iter().max_by_key(|p| p.mem_bytes) {
            out.push(Suggestion {
                title: format!(
                    "Memory pressure: {} using {}",
                    top.name,
                    fmt_bytes(top.mem_bytes)
                ),
                detail: format!(
                    "Close unused apps or lower priority of PID {}.",
                    top.pid
                ),
            });
        }
    }
    for p in rows.iter().filter(|p| {
        p.cpu < 0.5 && (p.read_bps + p.write_bps) < 1024 && p.mem_bytes > 500 * 1024 * 1024
    }) {
        out.push(Suggestion {
            title: format!("Idle hog: {} holding {}", p.name, fmt_bytes(p.mem_bytes)),
            detail: format!("You could lower its priority or close it. PID {}", p.pid),
        });
    }
    out
}

fn bytes_per_sec(prev: u64, now: u64, dt_s: f32) -> f32 {
    if now >= prev {
        (now - prev) as f32 / dt_s
    } else {
        0.0
    }
}

fn total_net_bytes(nets: &Networks) -> (u64, u64) {
    let mut rx = 0;
    let mut tx = 0;
    for (_name, data) in nets.iter() {
        rx += data.total_received();
        tx += data.total_transmitted();
    }
    (rx, tx)
}

fn total_disk_bytes(sys: &System) -> (u64, u64) {
    let mut r = 0;
    let mut w = 0;
    for (_pid, process) in sys.processes() {
        let io = process.disk_usage();
        r += io.total_read_bytes;
        w += io.total_written_bytes;
    }
    (r, w)
}