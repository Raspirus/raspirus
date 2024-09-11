use iced::widget::svg;

use super::{BLUE_COLOR_LIGHT, SECONDARY_COLOR};

#[derive(Default)]
pub struct RaspirusInfoIcon;
#[derive(Default)]
pub struct RaspirusSettingsIcon;
#[derive(Default)]
pub struct RaspirusWhiteIcon;

impl svg::StyleSheet for RaspirusInfoIcon {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> svg::Appearance {
        svg::Appearance {
            color: Some(SECONDARY_COLOR),
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

impl svg::StyleSheet for RaspirusWhiteIcon {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> svg::Appearance {
        svg::Appearance {
            color: Some(iced::Color::WHITE),
            ..Default::default()
        }
    }
}
