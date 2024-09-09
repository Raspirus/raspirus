use iced_aw::style::selection_list;

use super::ORANGE_COLOR_LIGHT;

#[derive(Default)]
pub struct RaspirusSelectionList;

impl selection_list::StyleSheet for RaspirusSelectionList {
    type Style = iced::Theme;

    fn style(&self, _style: &Self::Style) -> selection_list::Appearance {
        selection_list::Appearance {
            background: iced::Background::Color(ORANGE_COLOR_LIGHT),
            ..Default::default()
        }
    }
}