use std::path::PathBuf;

use iced::widget::text::Wrapping;

use crate::{
    backend::yara_scanner::{Skipped, TaggedFile},
    frontend::{
        iced::{wrap, Card, Message, Raspirus},
        theme::{button::button_primary_style, icon::white_icon_style, PRIMARY_COLOR},
    },
};

impl Raspirus {
    pub fn results(
        &self,
        tagged: Vec<(TaggedFile, bool)>,
        skipped: Vec<(Skipped, bool)>,
        log_path: PathBuf,
    ) -> iced::Element<Message> {
        let top_row = iced::widget::Column::new()
            .push(
                iced::widget::Row::new()
                    .push(
                        iced::widget::Button::new(
                            iced::widget::Row::new()
                                .push(
                                    iced::widget::svg::Svg::from_path("src/assets/icons/home.svg")
                                        .height(20)
                                        .width(20)
                                        .style(white_icon_style),
                                )
                                .push(
                                    iced::widget::container(iced::widget::text("HOME")), //TODO.padding([0, 0, 0, 5]),
                                ),
                        )
                        .on_press(Message::OpenMain)
                        .style(button_primary_style)
                        .padding(7),
                    )
                    .push(
                        iced::widget::container(
                            iced::widget::text("Results")
                                .size(30)
                                .font(iced::font::Font {
                                    weight: iced::font::Weight::Bold,
                                    ..iced::font::Font::DEFAULT
                                })
                                .style(|_| iced::widget::text::Style {
                                    color: Some(PRIMARY_COLOR),
                                }),
                        )
                        .padding([0, 10]),
                    )
                    .padding([5, 0])
                    .push(iced::widget::horizontal_space())
                    .push(
                        iced::widget::Button::new(
                            iced::widget::Text::new(iced_fonts::Bootstrap::Download.to_string())
                                .font(iced_fonts::BOOTSTRAP_FONT),
                        )
                        .on_press(Message::DownloadLog { log_path }),
                    )
                    .align_y(iced::Alignment::Center),
            )
            .push(iced::widget::horizontal_rule(5))
            .padding(10);

        let mut tagged_list = iced::widget::Column::new()
            .push(iced::widget::Text::new(format!(
                "Found files ({})",
                tagged.len()
            )))
            .spacing(5);

        for (tag, expanded) in tagged {
            //let expanded = *expanded;
            let mut descriptions = iced::widget::Column::new().height(iced::Length::Shrink);

            if expanded {
                // build rules list for each match
                for rule in tag.descriptions.clone() {
                    descriptions = descriptions.push(
                        iced::widget::Row::new()
                            .push(iced::widget::Text::new(rule.rule_name))
                            .push(iced::widget::horizontal_space())
                            .push(iced::widget::Text::new(rule.rule_description)),
                    )
                }
            }

            // build the dropdown
            tagged_list = tagged_list.push({
                let mut card = iced_aw::widgets::Card::new(
                    iced::widget::Row::new()
                        .push(
                            iced::widget::Text::new(format!("{}", tag.path.to_string_lossy()))
                                .wrapping(Wrapping::Glyph),
                        )
                        .push(iced::widget::horizontal_space())
                        .push(iced_aw::widgets::Badge::new(iced::widget::Text::new(
                            format!("{}", tag.rule_count),
                        )))
                        .push(
                            iced::widget::Button::new(
                                iced::widget::Text::new(
                                    if expanded {
                                        iced_fonts::Bootstrap::CaretDownFill
                                    } else {
                                        iced_fonts::Bootstrap::CaretLeftFill
                                    }
                                    .to_string(),
                                )
                                .font(iced_fonts::BOOTSTRAP_FONT),
                            )
                            .on_press(Message::ToggleCard {
                                card: Card::Tagged { card: tag.clone() },
                            }),
                        )
                        .push(
                            iced::widget::Button::new(
                                iced::widget::Text::new(
                                    iced_fonts::Bootstrap::BoxArrowUpLeft.to_string(),
                                )
                                .font(iced_fonts::BOOTSTRAP_FONT),
                            )
                            .on_press(Message::GenerateVirustotal { path: tag.path }),
                        )
                        .spacing(5)
                        .height(iced::Length::Shrink),
                    descriptions,
                );
                if !expanded {
                    card = card
                        .foot(iced::widget::horizontal_space().height(0))
                        .padding_foot(0.into())
                        .padding_body(0.into());
                }
                card
            });
        }

        let mut skipped_list = iced::widget::Column::new()
            .push(iced::widget::Text::new(format!(
                "Skipped files ({})",
                skipped.len()
            )))
            .spacing(5);

        for (skip, expanded) in skipped {
            //let expanded = *expanded;
            let mut description = iced::widget::Column::new().height(iced::Length::Shrink);

            if expanded {
                description = description.push(iced::widget::Text::new(skip.reason.clone()));
            }

            // build the dropdown
            skipped_list = skipped_list.push({
                let mut card = iced_aw::widgets::Card::new(
                    iced::widget::Row::new()
                        .push(
                            iced::widget::Text::new(format!("{}", skip.path.to_string_lossy()))
                                .wrapping(Wrapping::Glyph),
                        )
                        .push(iced::widget::horizontal_space())
                        .push(
                            iced::widget::Button::new(
                                iced::widget::Text::new(
                                    if expanded {
                                        iced_fonts::Bootstrap::CaretDownFill
                                    } else {
                                        iced_fonts::Bootstrap::CaretLeftFill
                                    }
                                    .to_string(),
                                )
                                .font(iced_fonts::BOOTSTRAP_FONT),
                            )
                            .on_press(Message::ToggleCard {
                                card: Card::Skipped { card: skip.clone() },
                            }),
                        )
                        .spacing(5)
                        .height(iced::Length::Shrink),
                    description,
                );
                if !expanded {
                    card = card
                        .foot(iced::widget::horizontal_space().height(0))
                        .padding_foot(0.into())
                        .padding_body(0.into());
                }
                card
            });
        }
        iced::widget::Scrollable::new(wrap(
            15,
            iced::widget::Column::new()
                .push(top_row)
                .push(tagged_list)
                .push(iced::widget::horizontal_rule(5))
                .push(skipped_list)
                .spacing(5)
                .into(),
        ))
        .style(|_, _1| iced::widget::scrollable::Style {
            vertical_rail: iced::widget::scrollable::Rail {
                scroller: iced::widget::scrollable::Scroller {
                    color: iced::Color::TRANSPARENT,
                    border: iced::Border::default(),
                },
                background: None,
                border: iced::Border::default(),
            },
            container: iced::widget::container::Style::default(),
            horizontal_rail: iced::widget::scrollable::Rail {
                scroller: iced::widget::scrollable::Scroller {
                    color: iced::Color::TRANSPARENT,
                    border: iced::Border::default(),
                },
                background: None,
                border: iced::Border::default(),
            },
            gap: None,
        })
        .into()
    }
}
