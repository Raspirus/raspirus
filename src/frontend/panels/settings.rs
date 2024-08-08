use crate::frontend::iced::{Message, Raspirus};

impl Raspirus {
    pub fn settings(&self) -> iced::Element<Message> {
        iced::Element::new(iced::widget::Button::new("back").on_press(Message::OpenMain))
    }
}
