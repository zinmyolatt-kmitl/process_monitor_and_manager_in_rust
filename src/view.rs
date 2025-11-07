use iced::{Alignment, Color, Element, Length};
use iced::widget::{button, checkbox, column, container, row, scrollable, text, text_input, Space};
use crate::models::{Message, ProcRow, SettingsModel, SortDir, SortKey, Suggestion};
use crate::styles::*;
use crate::util::fmt_bytes;

pub fn sortable<'a>(label: &str, key: SortKey, s: &SettingsModel) -> Element<'a, Message> {
    let mut caption = label.to_string();
    if s.sort_key == key {
        caption.push_str(match s.sort_dir {
            SortDir::Asc => " ↑",
            SortDir::Desc => " ↓",
        });
    }
    button(text(caption).size(14).shaping(text::Shaping::Advanced))
        .on_press(Message::SortBy(key))
        .width(Length::Fill)
        .into()
}

pub fn controls_row<'a>(settings: &SettingsModel) -> Element<'a, Message> {
    row![
        Space::with_width(150.0),
        text_input("Filter (name or PID)", &settings.filter)
            .on_input(Message::FilterChanged)
            .width(360.0)
            .style(iced::theme::TextInput::Custom(Box::new(RoundedTextInput))),
        Space::with_width(Length::FillPortion(1)),
        row![
            text_input("Start command…", &settings.cmd_to_start)
                .on_input(Message::StartChanged)
                .on_submit(Message::StartNow)
                .width(Length::Fixed(260.0))
                .style(iced::theme::TextInput::Custom(Box::new(RoundedTextInput))),
            Space::with_width(10.0),
            button("Start")
                .on_press(Message::StartNow)
                .padding([6, 24])
                .style(iced::theme::Button::Custom(Box::new(StartButton))),
        ]
        .align_items(Alignment::Center),
        Space::with_width(Length::FillPortion(1)),
    ]
    .spacing(10)
    .align_items(Alignment::Center)
    .into()
}

pub fn table_header<'a>(settings: &SettingsModel) -> Element<'a, Message> {
    #[cfg(target_os = "windows")]
    let name_width = 450;
    #[cfg(all(target_family = "unix", not(target_os = "macos")))]
    let name_width = 510;
    #[cfg(target_os = "macos")]
    let name_width = Length::FillPortion(3);

    container(
        row![
            container(sortable("PID", SortKey::Pid, settings)).width(70.0),
            container(sortable("Name", SortKey::Name, settings)).width(name_width),
            container(sortable("CPU %", SortKey::Cpu, settings)).width(80.0),
            container(sortable("Memory", SortKey::Mem, settings)).width(110.0),
            container(sortable("Read/s", SortKey::Read, settings)).width(110.0),
            container(sortable("Write/s", SortKey::Write, settings)).width(110.0),
            container(text("Actions").size(18))
                .width(Length::FillPortion(2))
                .center_x()
                .center_y(),
        ]
        .spacing(20)
        .align_items(Alignment::Center)
    )
    .padding([12, 10])
    .into()
}

pub fn top_bar<'a>(proc_count: usize, dot_phase: usize) -> Element<'a, Message> {
    let dots = ".".repeat(dot_phase);
    let status_text = format!("{} Processes currently running", proc_count);

    let dot_display = text(format!("{:<3}", dots))
        .size(16)
        .style(Color::from_rgb(1.0, 1.0, 0.0))
        .font(iced::Font::MONOSPACE);

    row![
        text("ProcDeck – Process Monitor & Manager")
            .size(23)
            .style(Color::from_rgb(0.6, 0.8, 1.0)),
        Space::with_width(Length::Fill),
        row![
            text(status_text)
                .size(16)
                .style(Color::from_rgb(1.0, 1.0, 0.0)),
            dot_display,
        ]
        .spacing(2)
    ]
    .align_items(Alignment::Center)
    .padding([8, 30])
    .into()
}


pub fn process_row<'a>(p: &ProcRow) -> Element<'a, Message> {
    #[cfg(target_os = "windows")]
    let name_width = 450;
    #[cfg(all(target_family = "unix", not(target_os = "macos")))]
    let name_width = 510;
    #[cfg(target_os = "macos")]
    let name_width = Length::FillPortion(3);

    container(
        row![
            text(p.pid).width(70.0),
            text(p.name.clone()).width(name_width),
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
                .spacing(6)
            )
            .padding([0, 8, 0, 0])
            .width(Length::FillPortion(2))
        ]
        .spacing(20),
    )
    .padding([4, 10])
    .into()
}

pub fn alert_controls<'a>(settings: &SettingsModel) -> Element<'a, Message> {
    row![
        text("Alerts:").size(14),
        Space::with_width(10.0),
        checkbox("CPU", settings.alerts_on_cpu)
            .on_toggle(Message::CpuAlertChanged),
        Space::with_width(10.0),
        checkbox("Memory", settings.alerts_on_mem)
            .on_toggle(Message::MemAlertChanged),
    ]
    .align_items(Alignment::Center)
    .into()
}

pub fn suggestions_view<'a>(suggestions: &[Suggestion]) -> Element<'a, Message> {
    if suggestions.is_empty() {
        container(
            text("No suggestions. System looks calm.")
                .size(16)
                .style(Color::from_rgb(0.4, 0.85, 0.4)),
        )
        .padding(8)
        .into()
    } else {
        let items = suggestions.iter().map(|s| {
            let color = if s.title.contains("CPU") {
                Color::from_rgb(1.0, 0.4, 0.4)
            } else if s.title.contains("Idle") {
                Color::from_rgb(0.6, 0.6, 1.0)
            } else if s.title.contains("Memory") {
                Color::from_rgb(1.0, 0.8, 0.4)
            } else {
                Color::from_rgb(0.9, 0.9, 0.9)
            };

            let bg_color = if s.title.contains("CPU") {
                Color::from_rgb(0.25, 0.1, 0.1)
            } else if s.title.contains("Idle") {
                Color::from_rgb(0.15, 0.15, 0.25)
            } else if s.title.contains("Memory") {
                Color::from_rgb(0.25, 0.2, 0.1)
            } else {
                Color::from_rgb(0.2, 0.2, 0.2)
            };

            container(
                column![
                    text(&s.title)
                        .size(16)
                        .style(color),
                    text(&s.detail)
                        .size(14)
                        .style(Color::from_rgb(0.8, 0.8, 0.8)),
                ]
                .spacing(2),
            )
            .padding([8, 10])
            .style(iced::theme::Container::Custom(Box::new(StaticBg { bg: bg_color })))
            .into()
        });

        let suggestion_height = 60.0;
        let spacing = 8.0;
        let num_suggestions = suggestions.len();
        
        let container_height = if num_suggestions <= 3 {
            Length::Shrink
        } else {
            Length::Fixed(3.0 * suggestion_height + 2.0 * spacing)
        };

        container(
            scrollable(column(items).spacing(8))
                .height(container_height)
                .width(Length::Fill),
        )
        .width(Length::Fill)
        .into()
    }
}