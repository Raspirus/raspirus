use crate::frontend::iced::{Message, Raspirus};

impl Raspirus { 
    pub fn scanning(&self, progress: f32) -> iced::Element<Message> {
        iced::Element::new(iced::widget::ProgressBar::new(0.0..=100.0, progress))
    }
}
