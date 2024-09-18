use iced::font;

use crate::frontend::{
    iced::{wrap, Message, Raspirus},
    theme::SECONDARY_COLOR,
};

impl Raspirus {
    pub fn scanning(&self, percentage: f32) -> iced::Element<Message> {
        let middle_row = iced::widget::Column::new()
            .push(
                iced::widget::Row::new()
                    .push(iced::widget::horizontal_space())
                    .push(
                        iced::widget::container::Container::new(
                            iced::widget::text("Scanning...")
                                .size(80)
                                .horizontal_alignment(iced::alignment::Horizontal::Center)
                                .font(font::Font {
                                    weight: iced::font::Weight::Bold,
                                    ..font::Font::DEFAULT
                                })
                                .style(SECONDARY_COLOR),
                        )
                        .padding([0, 0, 10, 0]),
                    )
                    .push(iced::widget::horizontal_space())
                    .spacing(5),
            )
            .push(
                iced::widget::Row::new()
                    .push(
                        iced::widget::container::Container::new(iced::widget::ProgressBar::new(
                            0.0..=100.0,
                            percentage,
                        ))
                        .padding([10, 20]),
                    )
                    .spacing(5),
            )
            .push(
                iced::widget::Row::new()
                    .push(iced::widget::horizontal_space())
                    .push(
                        iced::widget::Text::new(format!("{percentage:.2}%"))
                            .font(font::Font {
                                weight: iced::font::Weight::Bold,
                                ..font::Font::DEFAULT
                            })
                            .size(30),
                    )
                    .push(iced::widget::horizontal_space())
                    .spacing(5),
            )
            .spacing(5);

        wrap(
            10,
            iced::widget::Column::new()
                .push(iced::widget::vertical_space())
                .push(middle_row)
                .push(iced::widget::vertical_space())
                .into(),
        )
    }
}
