use iced::{Color, Background};
use iced::widget::{button, container, text_input};

pub struct StaticBg {
    pub bg: Color,
}

impl container::StyleSheet for StaticBg {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.bg)),
            border: iced::Border {
                radius: 6.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub struct StartButton;
impl button::StyleSheet for StartButton {
    type Style = iced::Theme;
    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb(0.1, 0.4, 0.8))),
            text_color: Color::WHITE,
            border: iced::Border {
                radius: 16.0.into(), 
                width: 0.0,
                ..Default::default()
            },
            ..Default::default()
        }
    }
    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb(0.15, 0.55, 1.0))),
            text_color: Color::WHITE,
            border: iced::Border {
                radius: 16.0.into(),
                width: 0.0,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
impl RoundedBase for StartButton {}

pub struct RoundedTextInput;

impl text_input::StyleSheet for RoundedTextInput {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(Color::from_rgb(0.18, 0.18, 0.18)),
            border: iced::Border {
                radius: 7.0.into(),
                width: 1.0,
                color: Color::from_rgb(0.3, 0.3, 0.3),
            },
            icon_color: Color::WHITE,
        }
    }

    fn focused(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(Color::from_rgb(0.22, 0.22, 0.22)),
            border: iced::Border {
                radius: 7.0.into(),
                width: 1.0,
                color: Color::from_rgb(0.4, 0.4, 1.0),
            },
            icon_color: Color::WHITE,
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgb(0.6, 0.6, 0.6)
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        Color::WHITE
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgb(0.4, 0.4, 1.0)
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgb(0.5, 0.5, 0.5)
    }

    fn disabled(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(Color::from_rgb(0.15, 0.15, 0.15)),
            border: iced::Border {
                radius: 6.0.into(),
                width: 1.0,
                color: Color::from_rgb(0.25, 0.25, 0.25),
            },
            icon_color: Color::from_rgb(0.5, 0.5, 0.5),
        }
    }
}

// Base trait for shared rounded look
pub trait RoundedBase {
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

pub struct KillButton;
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

pub struct SuspendButton;
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

pub struct ResumeButton;
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

pub struct BoostButton;
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

pub struct LowerButton;
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