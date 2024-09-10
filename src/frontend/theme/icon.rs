use iced::widget::svg;

use super::{BLUE_COLOR_LIGHT, SECONDARY_COLOR_LIGHT};

#[derive(Default)]
pub struct RaspirusInfoIcon;
#[derive(Default)]
pub struct RaspirusSettingsIcon;

impl svg::StyleSheet for RaspirusInfoIcon {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> svg::Appearance {
        svg::Appearance {
            color: Some(SECONDARY_COLOR_LIGHT),
            ..Default::default()
        }
    }
}

impl svg::StyleSheet for RaspirusSettingsIcon {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> svg::Appearance {
        svg::Appearance {
            color: Some(BLUE_COLOR_LIGHT),
            ..Default::default()
        }
    }
}
