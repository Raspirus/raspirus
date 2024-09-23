use iced::font;
use rust_i18n::t;

use crate::frontend::{
    iced::{wrap, Message, Raspirus, ScanState},
    theme::SECONDARY_COLOR,
};

impl Raspirus {
    pub fn scanning(&self, scan_state: ScanState) -> iced::Element<Message> {
        let middle_row = match scan_state {
            ScanState::Percentage(percentage) => iced::widget::Column::new()
                .push(
                    iced::widget::Row::new()
                        .push(iced::widget::horizontal_space())
                        .push(iced::widget::container::Container::new(
                            iced::widget::text(t!("scanner_scanning"))
                                .size(80)
                                .align_x(iced::alignment::Horizontal::Center)
                                .font(font::Font {
                                    weight: iced::font::Weight::Bold,
                                    ..font::Font::DEFAULT
                                })
                                .style(|_| iced::widget::text::Style {
                                    color: Some(SECONDARY_COLOR),
                                }),
                        ))
                        .push(iced::widget::horizontal_space())
                        .spacing(5),
                )
                .push(
                    iced::widget::Row::new()
                        .push(
                            iced::widget::container::Container::new(
                                iced::widget::ProgressBar::new(0.0..=100.0, percentage),
                            )
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
                .spacing(5),
            ScanState::Indexing => iced::widget::Column::new()
                .push(
                    iced::widget::Row::new()
                        .push(iced::widget::horizontal_space())
                        .push(iced::widget::container::Container::new(
                            iced::widget::text(t!("scanner_indexing"))
                                .size(80)
                                .align_x(iced::alignment::Horizontal::Center)
                                .font(font::Font {
                                    weight: iced::font::Weight::Bold,
                                    ..font::Font::DEFAULT
                                })
                                .style(|_| iced::widget::text::Style {
                                    color: Some(SECONDARY_COLOR),
                                }),
                        ))
                        .push(iced::widget::horizontal_space())
                        .spacing(5),
                )
                .spacing(5),
        };

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
