use iced::widget::button::{self, Status};
use iced::{Border, Color, Shadow, Theme};

use super::{
    BLUE_COLOR, BLUE_COLOR_DARK, BLUE_COLOR_LIGHT, DEFAULT_BORDER_WIDTH, DEFAULT_BUTTON_RADIUS,
    DEFAULT_SHADOW_OFFSET, GRAY_BACKGROUND, GRAY_COLOR, ORANGE_COLOR, ORANGE_COLOR_DARK,
    PRIMARY_COLOR, PRIMARY_COLOR_DARK, SECONDARY_COLOR, SECONDARY_COLOR_DARK,
};

pub fn button_primary_style(_theme: &Theme, status: Status) -> button::Style {
    match status {
        Status::Active => button::Style {
            background: Some(iced::Background::Color(PRIMARY_COLOR)),
            text_color: Color::WHITE,
            border: Border::default().rounded(DEFAULT_BUTTON_RADIUS),
            shadow: Shadow {
                color: Color::BLACK,
                offset: DEFAULT_SHADOW_OFFSET,
                blur_radius: 2.0,
            },
        },
        Status::Disabled => button::Style {
            background: Some(iced::Background::Color(GRAY_COLOR)),
            text_color: Color::WHITE,
            border: Border::default().rounded(DEFAULT_BUTTON_RADIUS),
            shadow: Shadow {
                color: Color::BLACK,
                offset: DEFAULT_SHADOW_OFFSET,
                blur_radius: 5.0,
            },
        },
        _ => button::Style {
            background: Some(iced::Background::Color(PRIMARY_COLOR_DARK)),
            text_color: Color::WHITE,
            border: Border::default().rounded(DEFAULT_BUTTON_RADIUS),
            shadow: Shadow {
                color: Color::BLACK,
                offset: DEFAULT_SHADOW_OFFSET,
                blur_radius: 5.0,
            },
        },
    }
}

pub fn button_secondary_style(theme: &Theme, status: Status) -> button::Style {
    match status {
        Status::Active => match theme {
            Theme::Dark => button::Style {
                background: Some(iced::Background::Color(Color::BLACK)),
                text_color: SECONDARY_COLOR,
                border: Border {
                    color: SECONDARY_COLOR,
                    width: DEFAULT_BORDER_WIDTH,
                    radius: DEFAULT_BUTTON_RADIUS.into(),
                },
                shadow: Shadow {
                    color: Color::BLACK,
                    offset: DEFAULT_SHADOW_OFFSET,
                    blur_radius: 2.0,
                },
            },
            _ => button::Style {
                background: Some(iced::Background::Color(GRAY_BACKGROUND)),
                text_color: SECONDARY_COLOR,
                border: Border {
                    color: SECONDARY_COLOR,
                    width: DEFAULT_BORDER_WIDTH,
                    radius: DEFAULT_BUTTON_RADIUS.into(),
                },
                shadow: Shadow {
                    color: Color::BLACK,
                    offset: DEFAULT_SHADOW_OFFSET,
                    blur_radius: 2.0,
                },
            },
        },
        _ => button::Style {
            background: Some(iced::Background::Color(Color::WHITE)),
            text_color: SECONDARY_COLOR_DARK,
            border: Border {
                color: SECONDARY_COLOR_DARK,
                width: DEFAULT_BORDER_WIDTH,
                radius: DEFAULT_BUTTON_RADIUS.into(),
            },
            shadow: Shadow {
                color: Color::BLACK,
                offset: DEFAULT_SHADOW_OFFSET,
                blur_radius: 5.0,
            },
        },
    }
}

pub fn button_blue_style(_theme: &Theme, status: Status) -> button::Style {
    match status {
        Status::Active => button::Style {
            background: Some(iced::Background::Color(BLUE_COLOR)),
            text_color: Color::WHITE,
            border: Border::default().rounded(DEFAULT_BUTTON_RADIUS),
            shadow: Shadow {
                color: Color::BLACK,
                offset: DEFAULT_SHADOW_OFFSET,
                blur_radius: 2.0,
            },
        },
        Status::Disabled => button::Style {
            background: Some(iced::Background::Color(GRAY_COLOR)),
            text_color: Color::WHITE,
            border: Border::default().rounded(DEFAULT_BUTTON_RADIUS),
            shadow: Shadow {
                color: Color::BLACK,
                offset: DEFAULT_SHADOW_OFFSET,
                blur_radius: 5.0,
            },
        },
        _ => button::Style {
            background: Some(iced::Background::Color(BLUE_COLOR_DARK)),
            text_color: Color::WHITE,
            border: Border::default().rounded(DEFAULT_BUTTON_RADIUS),
            shadow: Shadow {
                color: Color::BLACK,
                offset: DEFAULT_SHADOW_OFFSET,
                blur_radius: 5.0,
            },
        },
    }
}

pub fn button_orange_style(_theme: &Theme, status: Status) -> button::Style {
    match status {
        Status::Active => button::Style {
            background: Some(iced::Background::Color(ORANGE_COLOR)),
            border: Border::default().rounded(DEFAULT_BUTTON_RADIUS),
            shadow: Shadow {
                color: Color::BLACK,
                offset: DEFAULT_SHADOW_OFFSET,
                blur_radius: 2.0,
            },
            ..Default::default()
        },
        _ => button::Style {
            background: Some(iced::Background::Color(ORANGE_COLOR_DARK)),
            border: Border::default().rounded(DEFAULT_BUTTON_RADIUS),
            shadow: Shadow {
                color: Color::BLACK,
                offset: DEFAULT_SHADOW_OFFSET,
                blur_radius: 5.0,
            },
            ..Default::default()
        },
    }
}

pub fn button_select_style(theme: &Theme, status: Status) -> button::Style {
    match status {
        Status::Active => match theme {
            Theme::Dark => button::Style {
                background: Some(iced::Background::Color(Color::BLACK)),
                text_color: Color::WHITE,
                border: Border {
                    color: BLUE_COLOR_LIGHT,
                    width: DEFAULT_BORDER_WIDTH,
                    radius: 5.into(),
                },
                shadow: Shadow {
                    color: Color::BLACK,
                    offset: DEFAULT_SHADOW_OFFSET,
                    blur_radius: 2.0,
                },
            },
            _ => button::Style {
                background: Some(iced::Background::Color(Color::WHITE)),
                text_color: Color::BLACK,
                border: Border {
                    color: BLUE_COLOR_LIGHT,
                    width: DEFAULT_BORDER_WIDTH,
                    radius: 5.into(),
                },
                shadow: Shadow {
                    color: Color::BLACK,
                    offset: DEFAULT_SHADOW_OFFSET,
                    blur_radius: 2.0,
                },
            },
        },
        _ => button::Style {
            background: Some(iced::Background::Color(GRAY_BACKGROUND)),
            text_color: Color::BLACK,
            border: Border {
                color: BLUE_COLOR_DARK,
                width: DEFAULT_BORDER_WIDTH,
                radius: 5.into(),
            },
            shadow: Shadow {
                color: Color::BLACK,
                offset: DEFAULT_SHADOW_OFFSET,
                blur_radius: 5.0,
            },
        },
    }
}

pub fn button_transparent_style(_theme: &Theme, status: Status) -> button::Style {
    match status {
        Status::Active => button::Style {
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            text_color: PRIMARY_COLOR,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
            shadow: Shadow {
                color: Color::TRANSPARENT,
                offset: DEFAULT_SHADOW_OFFSET,
                blur_radius: 0.0,
            },
        },
        _ => button::Style {
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            text_color: PRIMARY_COLOR_DARK,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
            shadow: Shadow {
                color: Color::TRANSPARENT,
                offset: DEFAULT_SHADOW_OFFSET,
                blur_radius: 0.0,
            },
        },
    }
}

pub fn button_selectionlist(theme: &Theme, status: Status) -> button::Style {
    if let Status::Hovered = status {
        let mut theme = button_selectionlist_selected(theme, status);
        theme.text_color.a = 0.75;
        return theme;
    }
    button::Style {
        background: Some(iced::Background::Color(Color::TRANSPARENT)),
        text_color: button::Style::default().text_color.scale_alpha(0.25),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        shadow: Shadow {
            color: Color::TRANSPARENT,
            offset: DEFAULT_SHADOW_OFFSET,
            blur_radius: 0.0,
        },
    }
}

pub fn button_selectionlist_selected(_theme: &Theme, _status: Status) -> button::Style {
    button::Style {
        background: Some(iced::Background::Color(Color::TRANSPARENT)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        shadow: Shadow {
            color: Color::TRANSPARENT,
            offset: DEFAULT_SHADOW_OFFSET,
            blur_radius: 0.0,
        },
        ..Default::default()
    }
}
