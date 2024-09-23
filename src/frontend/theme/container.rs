use iced::{widget::container, Theme};

use super::{
    BLUE_COLOR_LIGHT, CARD_SHADOW_OFFSET, DEFAULT_BORDER_RADIUS, DEFAULT_BORDER_WIDTH, DEFAULT_BUTTON_RADIUS, GRAY_COLOR, ORANGE_COLOR_LIGHT
};

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

pub fn container_selection_list(_theme: &Theme) -> container::Style {
    container::Style {
        border: iced::Border::default()
            .rounded(DEFAULT_BUTTON_RADIUS)
            .color(GRAY_COLOR)
            .width(1),
        background: Some(iced::Background::Color(ORANGE_COLOR_LIGHT)),
        ..Default::default()
    }
}

pub fn container_selection_list_lang(_theme: &Theme) -> container::Style {
    container::Style {
        border: iced::Border::default()
            .rounded(DEFAULT_BUTTON_RADIUS)
            .color(GRAY_COLOR)
            .width(1),
        background: Some(iced::Background::Color(BLUE_COLOR_LIGHT)),
        ..Default::default()
    }
}