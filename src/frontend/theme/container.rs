use iced::widget::container;

use super::{CARD_SHADOW_OFFSET, DEFAULT_BORDER_RADIUS, DEFAULT_BORDER_WIDTH};

pub struct RaspirusInfoCard;

impl container::StyleSheet for RaspirusInfoCard {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(iced::Color::WHITE)),
            border: iced::Border {
                color: iced::Color::WHITE,
                width: DEFAULT_BORDER_WIDTH,
                radius: DEFAULT_BORDER_RADIUS.into()
            },
            shadow: iced::Shadow { color: iced::Color::BLACK, offset: CARD_SHADOW_OFFSET, blur_radius: 2.0 },
            ..Default::default()
        }
    }
}