use iced::Theme;
use iced_aw::{card::Status, style::selection_list};

use super::ORANGE_COLOR_LIGHT;

pub fn selection_list_style(_theme: &Theme, _status: Status) -> selection_list::Style {
    selection_list::Style {
        background: iced::Background::Color(ORANGE_COLOR_LIGHT),
        ..Default::default()
    }
}
