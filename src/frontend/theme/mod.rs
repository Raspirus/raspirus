use iced::{color, Color};

pub mod button;
pub mod container;
pub mod icon;
pub mod selection_list;
pub mod toggle;

/// Colors for the Raspirus theme
pub const PRIMARY_COLOR: Color = color!(0xd7105e);
pub const PRIMARY_COLOR_DARK: Color = color!(0x960b42);

pub const SECONDARY_COLOR: Color = color!(0x01a56d);
pub const SECONDARY_COLOR_DARK: Color = color!(0x01734c);
pub const SECONDARY_COLOR_LIGHT: Color = color!(0xc8ffec);

pub const BLUE_COLOR: Color = color!(0x2181d4);
pub const BLUE_COLOR_DARK: Color = color!(0x1a67aa);
pub const BLUE_COLOR_LIGHT: Color = color!(0x007BFF);

pub const ORANGE_COLOR: Color = color!(0xFB923C);
pub const ORANGE_COLOR_DARK: Color = color!(0xf47105);
pub const ORANGE_COLOR_LIGHT: Color = color!(0xfdc89e);

pub const GRAY_COLOR: Color = color!(0x808080);
pub const GRAY_BACKGROUND: Color = color!(0xE5E7EB);

/// Default settings
pub const DEFAULT_BUTTON_RADIUS: f32 = 5.0;
pub const DEFAULT_SHADOW_OFFSET: iced::Vector = iced::Vector::new(1.0, 1.0);
pub const CARD_SHADOW_OFFSET: iced::Vector = iced::Vector::new(0.5, 0.5);
pub const DEFAULT_BORDER_WIDTH: f32 = 3.0;
pub const DEFAULT_BORDER_RADIUS: f32 = 10.0;

/// Theme color palette
pub const RASPIRUS_PALETTE: iced::theme::Palette = iced::theme::Palette {
    background: Color {
        r: 0.898,
        g: 0.906,
        b: 0.922,
        a: 1.0,
    },
    text: Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    },
    primary: Color {
        r: 0.36862746,
        g: 0.4862745,
        b: 0.8862745,
        a: 1.0,
    },
    success: Color {
        r: 0.07058824,
        g: 0.4,
        b: 0.30980393,
        a: 1.0,
    },
    danger: Color {
        r: 0.7647059,
        g: 0.25882354,
        b: 0.24705882,
        a: 1.0,
    },
};

// EXAMPLE
// button().style(Button::Custom(Box::new(RaspirusButtonSecondary)))
