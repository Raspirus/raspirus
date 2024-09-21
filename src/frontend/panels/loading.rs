use crate::frontend::iced::{Message, Raspirus};

impl Raspirus {
    pub fn loading(&self) -> iced::Element<Message> {
        iced::widget::Column::new()
            .push(iced::widget::vertical_space())
            .push(iced::widget::Text::new("Loading"))
            .push(iced::widget::vertical_space())
            .spacing(5)
            .into()
    }
}
