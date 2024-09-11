use iced::widget::button;
use iced::{Border, Color, Shadow};

use super::{
    BLUE_COLOR, BLUE_COLOR_DARK, BLUE_COLOR_LIGHT, DEFAULT_BORDER_WIDTH, DEFAULT_BUTTON_RADIUS,
    DEFAULT_SHADOW_OFFSET, GRAY_BACKGROUND, GRAY_COLOR, ORANGE_COLOR, ORANGE_COLOR_DARK,
    PRIMARY_COLOR, PRIMARY_COLOR_DARK, SECONDARY_COLOR, SECONDARY_COLOR_DARK,
};

#[derive(Default)]
pub struct RaspirusButtonPrimary;
#[derive(Default)]
pub struct RaspirusButtonSecondary;
#[derive(Default)]
pub struct RaspirusButtonBlue;
#[derive(Default)]
pub struct RaspirusButtonOrange;
#[derive(Default)]
pub struct RaspirusButtonSelect;
#[derive(Default)]
pub struct RaspirusButtonTransparent;

impl iced::widget::button::StyleSheet for RaspirusButtonPrimary {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(PRIMARY_COLOR)),
            text_color: Color::WHITE,
            border: Border::with_radius(DEFAULT_BUTTON_RADIUS),
            shadow: Shadow {
                color: Color::BLACK,
                offset: DEFAULT_SHADOW_OFFSET,
                blur_radius: 2.0,
            },
            ..Default::default()
        }
    }
    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(PRIMARY_COLOR_DARK)),
            text_color: Color::WHITE,
            border: Border::with_radius(DEFAULT_BUTTON_RADIUS),
            shadow: Shadow {
                color: Color::BLACK,
                offset: DEFAULT_SHADOW_OFFSET,
                blur_radius: 5.0,
            },
            ..Default::default()
        }
    }
    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        self.hovered(_style)
    }
}

impl iced::widget::button::StyleSheet for RaspirusButtonSecondary {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
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
            ..Default::default()
        }
    }
    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
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
            ..Default::default()
        }
    }
    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        self.hovered(_style)
    }
}

impl iced::widget::button::StyleSheet for RaspirusButtonBlue {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(BLUE_COLOR)),
            text_color: Color::WHITE,
            border: Border::with_radius(DEFAULT_BUTTON_RADIUS),
            shadow: Shadow {
                color: Color::BLACK,
                offset: DEFAULT_SHADOW_OFFSET,
                blur_radius: 2.0,
            },
            ..Default::default()
        }
    }
    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(BLUE_COLOR_DARK)),
            text_color: Color::WHITE,
            border: Border::with_radius(DEFAULT_BUTTON_RADIUS),
            shadow: Shadow {
                color: Color::BLACK,
                offset: DEFAULT_SHADOW_OFFSET,
                blur_radius: 5.0,
            },
            ..Default::default()
        }
    }
    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        self.hovered(_style)
    }
}

impl iced::widget::button::StyleSheet for RaspirusButtonOrange {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(ORANGE_COLOR)),
            text_color: Color::WHITE,
            border: Border::with_radius(DEFAULT_BUTTON_RADIUS),
            shadow: Shadow {
                color: Color::BLACK,
                offset: DEFAULT_SHADOW_OFFSET,
                blur_radius: 2.0,
            },
            ..Default::default()
        }
    }
    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(ORANGE_COLOR_DARK)),
            text_color: Color::WHITE,
            border: Border::with_radius(DEFAULT_BUTTON_RADIUS),
            shadow: Shadow {
                color: Color::BLACK,
                offset: DEFAULT_SHADOW_OFFSET,
                blur_radius: 5.0,
            },
            ..Default::default()
        }
    }
    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        self.hovered(_style)
    }
}

impl iced::widget::button::StyleSheet for RaspirusButtonSelect {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(Color::WHITE)),
            text_color: Color::BLACK,
            border: Border {
                color: BLUE_COLOR_LIGHT,
                width: DEFAULT_BORDER_WIDTH,
                radius: 0.5.into(),
            },
            shadow: Shadow {
                color: Color::BLACK,
                offset: DEFAULT_SHADOW_OFFSET,
                blur_radius: 2.0,
            },
            ..Default::default()
        }
    }
    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(GRAY_COLOR)),
            text_color: Color::BLACK,
            border: Border {
                color: BLUE_COLOR_DARK,
                width: DEFAULT_BORDER_WIDTH,
                radius: 0.5.into(),
            },
            shadow: Shadow {
                color: Color::BLACK,
                offset: DEFAULT_SHADOW_OFFSET,
                blur_radius: 5.0,
            },
            ..Default::default()
        }
    }
    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        self.hovered(_style)
    }
}

impl iced::widget::button::StyleSheet for RaspirusButtonTransparent {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
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
            ..Default::default()
        }
    }
    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
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
            ..Default::default()
        }
    }
    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        self.hovered(_style)
    }
}