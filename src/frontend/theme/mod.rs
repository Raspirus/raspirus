use iced::{color, Color};

pub mod button;

/// Colors for the Raspirus theme
pub const PRIMARY_COLOR: Color = color!(0xd7105e);
pub const PRIMARY_COLOR_DARK: Color = color!(0x960b42);

pub const SECONDARY_COLOR: Color = color!(0x01a56d);
pub const SECONDARY_COLOR_DARK: Color = color!(0x01734c);

pub const BLUE_COLOR: Color = color!(0x2181d4);
pub const BLUE_COLOR_DARK: Color = color!(0x1a67aa);

pub const ORANGE_COLOR: Color = color!(0xFB923C);
pub const ORANGE_COLOR_DARK: Color = color!(0xf47105);

/// Default settings
pub const DEFAULT_BUTTON_RADIUS: f32 = 2.5;
pub const DEFAULT_SHADOW_OFFSET: iced::Vector = iced::Vector::new(1.0, 1.0);
pub const DEFAULT_BORDER_WIDTH: f32 = 3.0;

// EXAMPLE
// button().style(Button::Custom(Box::new(RaspirusButtonSecondary)))