use iced::widget::toggler::{self, Status};

use super::RASPIRUS_PALETTE;

pub fn toggler_style(_theme: &iced::Theme, status: Status) -> toggler::Style {
    match status {
        Status::Active { is_toggled } | Status::Hovered { is_toggled } => {
            if is_toggled {
                toggler::Style {
                    background: RASPIRUS_PALETTE.success,
                    background_border_width: 1.0,
                    background_border_color: iced::Color::BLACK,
                    foreground: iced::Color::WHITE,
                    foreground_border_width: 1.0,
                    foreground_border_color: iced::Color::BLACK,
                }
            } else {
                toggler::Style {
                    background: RASPIRUS_PALETTE.danger,
                    background_border_width: 1.0,
                    background_border_color: iced::Color::BLACK,
                    foreground: iced::Color::WHITE,
                    foreground_border_width: 1.0,
                    foreground_border_color: iced::Color::BLACK,
                }
            }
        }
        Status::Disabled => toggler::Style {
            background: RASPIRUS_PALETTE.danger,
            background_border_width: 1.0,
            background_border_color: iced::Color::BLACK,
            foreground: iced::Color::WHITE,
            foreground_border_width: 1.0,
            foreground_border_color: iced::Color::BLACK,
        },
    }
}
