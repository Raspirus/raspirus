use iced::widget::svg;

use super::SECONDARY_COLOR_LIGHT;

#[derive(Default)]
pub struct RaspirusInfoIcon;

impl svg::StyleSheet for RaspirusInfoIcon {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> svg::Appearance {
        svg::Appearance {
            color: Some(SECONDARY_COLOR_LIGHT),
            ..Default::default()
        }
    }
}