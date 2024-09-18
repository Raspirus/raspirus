use iced::widget::toggler;

use super::{DEFAULT_BORDER_WIDTH, RASPIRUS_PALETTE};

#[derive(Default)]
pub struct RaspirusToggler;

impl toggler::StyleSheet for RaspirusToggler {
    type Style = iced::Theme;
    
    fn active(&self, _style: &Self::Style, is_active: bool) -> toggler::Appearance {
        match is_active {
            true => {
                toggler::Appearance {
                    background: RASPIRUS_PALETTE.success,
                    background_border_width: 1.0,
                    background_border_color: iced::Color::BLACK,
                    foreground: iced::Color::WHITE,
                    foreground_border_width: 1.0,
                    foreground_border_color: iced::Color::BLACK,
                }
            },
            false => {
                toggler::Appearance {
                    background: RASPIRUS_PALETTE.danger,
                    background_border_width: 1.0,
                    background_border_color: iced::Color::BLACK,
                    foreground: iced::Color::WHITE,
                    foreground_border_width: 1.0,
                    foreground_border_color: iced::Color::BLACK,
                }
            },
        }
    }
    
    fn hovered(&self, style: &Self::Style, is_active: bool) -> toggler::Appearance {
        self.active(style, is_active)
    }
}