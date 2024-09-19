use crate::frontend::iced::{wrap, Message, Raspirus};

impl Raspirus {
    pub fn terms(&self) -> iced::Element<Message> {
        wrap(
            10,
            iced::widget::Row::new()
                .push(iced::widget::horizontal_space())
                .push(
                    iced::widget::Button::new(
                        iced::widget::Text::new(iced_aw::Bootstrap::House.to_string())
                            .font(iced_aw::BOOTSTRAP_FONT),
                    )
                    .on_press(Message::OpenMain),
                )
                .into(),
        )
    }
}
