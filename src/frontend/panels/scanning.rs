use crate::frontend::iced::{Message, Raspirus};

impl Raspirus {
    pub fn scanning(&self) -> iced::Element<Message> {
        match self.state {
            crate::frontend::iced::State::Scanning(progress) => {
                let middle_row = iced::widget::Column::new()
                    .push(
                        iced::widget::Row::new()
                            .push(iced::widget::horizontal_space())
                            .push(iced::widget::Text::new("scanning"))
                            .push(iced::widget::horizontal_space())
                            .spacing(5),
                    )
                    .push(
                        iced::widget::Row::new()
                            .push(iced::widget::ProgressBar::new(0.0..=100.0, progress))
                            .spacing(5),
                    )
                    .push(
                        iced::widget::Row::new()
                            .push(iced::widget::horizontal_space())
                            .push(iced::widget::Text::new(format!("{progress:.2}%")))
                            .push(iced::widget::horizontal_space())
                            .spacing(5),
                    )
                    .spacing(5);

                iced::widget::Column::new()
                    .push(iced::widget::vertical_space())
                    .push(middle_row)
                    .push(iced::widget::vertical_space())
                    .into()
            }
            _ => iced::widget::Text::new("Wrong state").into(),
        }
    }
}
