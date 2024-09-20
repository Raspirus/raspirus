use iced::{widget::container, Theme};

use super::{
    CARD_SHADOW_OFFSET, DEFAULT_BORDER_RADIUS, DEFAULT_BORDER_WIDTH, SECONDARY_COLOR,
    SECONDARY_COLOR_LIGHT,
};

pub fn card_container_style(_theme: &Theme) -> container::Style {
    container::Style {
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
    }
}

pub fn icon_container_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(SECONDARY_COLOR_LIGHT)),
        border: iced::Border {
            color: SECONDARY_COLOR,
            width: 1.0,
            radius: 15.0.into(),
        },
        ..Default::default()
    }
}
