use iced::{widget::container, Theme};

use super::{CARD_SHADOW_OFFSET, DEFAULT_BORDER_RADIUS, DEFAULT_BORDER_WIDTH};

pub fn card_container_style(theme: &Theme) -> container::Style {
    match theme {
        Theme::Dark => container::Style {
            background: Some(iced::Background::Color(iced::Color::BLACK)),
            border: iced::Border {
                color: iced::Color::BLACK,
                width: DEFAULT_BORDER_WIDTH,
                radius: DEFAULT_BORDER_RADIUS.into(),
            },
            shadow: iced::Shadow {
                color: iced::Color::BLACK,
                offset: CARD_SHADOW_OFFSET,
                blur_radius: 2.0,
            },
            ..Default::default()
        },
        _ => container::Style {
            background: Some(iced::Background::Color(iced::Color::WHITE)),
            border: iced::Border {
                color: iced::Color::WHITE,
                width: DEFAULT_BORDER_WIDTH,
                radius: DEFAULT_BORDER_RADIUS.into(),
            },
            shadow: iced::Shadow {
                color: iced::Color::BLACK,
                offset: CARD_SHADOW_OFFSET,
                blur_radius: 2.0,
            },
            ..Default::default()
        },
    }
}
