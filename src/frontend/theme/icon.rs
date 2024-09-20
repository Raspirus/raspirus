use iced::{
    widget::svg::{self, Status},
    Theme,
};

use super::{BLUE_COLOR_LIGHT, SECONDARY_COLOR};

pub fn info_icon_style(_theme: &Theme, _status: Status) -> svg::Style {
    svg::Style {
        color: Some(SECONDARY_COLOR),
    }
}

pub fn settings_icon_style(_theme: &Theme, _status: Status) -> svg::Style {
    svg::Style {
        color: Some(BLUE_COLOR_LIGHT),
    }
}

pub fn white_icon_style(_theme: &Theme, _status: Status) -> svg::Style {
    svg::Style {
        color: Some(iced::Color::WHITE),
    }
}
