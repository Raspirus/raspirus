use iced::Theme;
use iced_aw::{card::Status, style::selection_list};

use super::{BLUE_COLOR, ORANGE_COLOR, ORANGE_COLOR_LIGHT};

pub fn selection_list_style(_theme: &Theme, status: Status) -> selection_list::Style {
    match status {
        Status::Active => selection_list::Style {
            background: iced::Background::Color(ORANGE_COLOR_LIGHT),
            ..Default::default()
        },
        Status::Hovered => selection_list::Style {
            background: iced::Background::Color(ORANGE_COLOR),
            ..Default::default()
        },
        _ => selection_list::Style {
            background: iced::Background::Color(ORANGE_COLOR_LIGHT),
            ..Default::default()
        },
    }
}
pub fn lang_selection_list_style(_theme: &Theme, status: Status) -> selection_list::Style {
    match status {
        Status::Active => selection_list::Style {
            ..Default::default()
        },
        Status::Hovered => selection_list::Style {
            ..Default::default()
        },
        _ => selection_list::Style {
            background: iced::Background::Color(BLUE_COLOR),
            ..Default::default()
        },
    }
}
