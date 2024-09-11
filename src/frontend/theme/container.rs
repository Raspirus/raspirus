use iced::widget::container;

use super::{
    CARD_SHADOW_OFFSET, DEFAULT_BORDER_RADIUS, DEFAULT_BORDER_WIDTH, SECONDARY_COLOR,
    SECONDARY_COLOR_LIGHT,
};

#[derive(Default)]
pub struct RaspirusCard;
#[derive(Default)]
pub struct RaspirusIconContainer;

impl container::StyleSheet for RaspirusCard {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
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
}

impl container::StyleSheet for RaspirusIconContainer {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(SECONDARY_COLOR_LIGHT)),
            border: iced::Border {
                color: SECONDARY_COLOR,
                width: 1.0,
                radius: 15.0.into(),
            },
            ..Default::default()
        }
    }
}
