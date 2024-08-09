use crate::frontend::iced::{Message, Raspirus};

impl Raspirus {
    pub fn scanning(&self, percentage: f32) -> iced::Element<Message> {
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
                    .push(iced::widget::ProgressBar::new(0.0..=100.0, percentage))
                    .spacing(5),
            )
            .push(
                iced::widget::Row::new()
                    .push(iced::widget::horizontal_space())
                    .push(iced::widget::Text::new(format!("{percentage:.2}%")))
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
}
