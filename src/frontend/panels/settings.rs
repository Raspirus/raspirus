use crate::{
    backend::config_file::Config,
    frontend::{
        iced::{wrap, ConfigValue, ErrorCase, Message, Raspirus, UpdateState},
        theme::{
            button::{button_blue_style, button_primary_style},
            container::card_container_style,
            icon::{settings_icon_style, white_icon_style},
            toggle::toggler_style,
            GRAY_COLOR, PRIMARY_COLOR,
        },
    },
};

impl Raspirus {
    pub fn settings(&self, config: &Config, update: &UpdateState) -> iced::Element<Message> {
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
                            iced::widget::text("Settings")
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
                    .align_y(iced::Alignment::Center),
            )
            .push(iced::widget::horizontal_rule(5))
            .padding(10);

        let options = iced::widget::Column::new()
            .push(
                iced::widget::container(
                    iced::widget::Row::new()
                        .push(
                            iced::widget::svg::Svg::from_path(
                                "src/assets/icons/database-import.svg",
                            )
                            .height(64)
                            .width(64)
                            .style(settings_icon_style),
                        )
                        .push(iced::widget::Space::with_width(10))
                        .push(
                            iced::widget::Column::new()
                                .push(iced::widget::text("Updated").size(20))
                                .push(iced::widget::Space::with_height(5))
                                .push(iced::widget::text("Updates the Rules").size(14).style(
                                    |_| iced::widget::text::Style {
                                        color: Some(GRAY_COLOR),
                                    },
                                ))
                                .width(iced::Length::Fill),
                        )
                        .push(iced::widget::horizontal_space())
                        .push(
                            iced::widget::Button::new(
                                iced::widget::Row::new()
                                    .push(iced::widget::Text::new(
                                        match update {
                                            UpdateState::Loaded => "UPDATE ",
                                            UpdateState::Updating => "UPDATING... ",
                                            UpdateState::Updated => "UPDATED ",
                                        }
                                        .to_string(),
                                    ))
                                    .push(
                                        iced::widget::Text::new(
                                            match update {
                                                UpdateState::Loaded => {
                                                    iced_fonts::Bootstrap::ArrowUpCircleFill
                                                }
                                                UpdateState::Updating => {
                                                    iced_fonts::Bootstrap::ArrowClockwise
                                                }
                                                UpdateState::Updated => {
                                                    iced_fonts::Bootstrap::Check
                                                }
                                            }
                                            .to_string(),
                                        )
                                        .font(iced_fonts::BOOTSTRAP_FONT),
                                    ),
                            )
                            .on_press(match update {
                                UpdateState::Loaded | UpdateState::Updated => Message::UpdateRules,
                                _ => Message::Error {
                                    case: ErrorCase::Warning {
                                        message: "Already running update!".to_owned(),
                                    },
                                },
                            })
                            .padding(10)
                            .style(button_blue_style),
                        )
                        .align_y(iced::Alignment::Center), //TODO.padding([7, 20, 7, 7]),
                )
                .style(card_container_style),
            )
            .push(
                iced::widget::container(
                    iced::widget::Row::new()
                        .push(
                            iced::widget::svg::Svg::from_path(
                                "src/assets/icons/arrow-badge-up.svg",
                            )
                            .height(64)
                            .width(64)
                            .style(settings_icon_style),
                        )
                        .push(iced::widget::Space::with_width(10))
                        .push(
                            iced::widget::Column::new()
                                .push(iced::widget::text("Max Matches").size(20))
                                .push(iced::widget::Space::with_height(5))
                                .push(
                                    iced::widget::text("Maximum amount of Rules that should match")
                                        .size(14)
                                        .style(|_| iced::widget::text::Style {
                                            color: Some(GRAY_COLOR),
                                        }),
                                )
                                .width(iced::Length::Fill),
                        )
                        .push(iced::widget::horizontal_space())
                        .push(iced_aw::widgets::NumberInput::new(
                            config.max_matches,
                            0..usize::MAX,
                            |matches| Message::ConfigChanged {
                                value: ConfigValue::MaxMatch(matches),
                            },
                        ))
                        .align_y(iced::Alignment::Center), //TODO.padding([7, 20, 7, 7]),
                )
                .style(card_container_style),
            )
            .push(
                iced::widget::container(
                    iced::widget::Row::new()
                        .push(
                            iced::widget::svg::Svg::from_path(
                                "src/assets/icons/arrow-badge-down.svg",
                            )
                            .height(64)
                            .width(64)
                            .style(settings_icon_style),
                        )
                        .push(iced::widget::Space::with_width(10))
                        .push(
                            iced::widget::Column::new()
                                .push(iced::widget::text("Min Matches").size(20))
                                .push(iced::widget::Space::with_height(5))
                                .push(
                                    iced::widget::text("Minumum amount of Rules that should match")
                                        .size(14)
                                        .style(|_| iced::widget::text::Style {
                                            color: Some(GRAY_COLOR),
                                        }),
                                )
                                .width(iced::Length::Fill),
                        )
                        .push(iced::widget::horizontal_space())
                        .push(iced_aw::widgets::NumberInput::new(
                            config.min_matches,
                            0..config.max_matches,
                            |matches| Message::ConfigChanged {
                                value: ConfigValue::MinMatch(matches),
                            },
                        ))
                        .align_y(iced::Alignment::Center), //TODO.padding([7, 20, 7, 7]),
                )
                .style(card_container_style),
            )
            .push(
                iced::widget::container(
                    iced::widget::Row::new()
                        .push(
                            iced::widget::svg::Svg::from_path(
                                "src/assets/icons/clipboard-data.svg",
                            )
                            .height(64)
                            .width(64)
                            .style(settings_icon_style),
                        )
                        .push(iced::widget::Space::with_width(10))
                        .push(
                            iced::widget::Column::new()
                                .push(iced::widget::text("Logging").size(20))
                                .push(iced::widget::Space::with_height(5))
                                .push(
                                    iced::widget::text("Activates debug logging")
                                        .size(14)
                                        .style(|_| iced::widget::text::Style {
                                            color: Some(GRAY_COLOR),
                                        }),
                                )
                                .width(iced::Length::Fill),
                        )
                        .push(iced::widget::horizontal_space())
                        .push(
                            iced::widget::Toggler::new(config.logging_is_active)
                                .on_toggle(|logging| Message::ConfigChanged {
                                    value: ConfigValue::Logging(logging),
                                })
                                .width(iced::Length::Shrink)
                                .size(25)
                                .style(toggler_style),
                        )
                        .align_y(iced::Alignment::Center)
                        .padding(7),
                )
                .style(card_container_style),
            )
            .push(
                iced::widget::container(
                    iced::widget::Row::new()
                        .push(
                            iced::widget::svg::Svg::from_path("src/assets/icons/file-download.svg")
                                .height(64)
                                .width(64)
                                .style(settings_icon_style),
                        )
                        .push(iced::widget::Space::with_width(10))
                        .push(
                            iced::widget::Column::new()
                                .push(iced::widget::text("Download Logs").size(20))
                                .push(iced::widget::Space::with_height(5))
                                .push(
                                    iced::widget::text("Copies the Logs to your Downloads folder")
                                        .size(14), //TODO.style(GRAY_COLOR),
                                )
                                .width(iced::Length::Fill),
                        )
                        .push(iced::widget::horizontal_space())
                        .push(
                            // TODO: Make this button functional
                            iced::widget::Button::new(
                                iced::widget::Row::new()
                                    .push(iced::widget::text("DOWNLOAD "))
                                    .push(
                                        iced::widget::Text::new(
                                            iced_fonts::Bootstrap::ArrowDownCircleFill.to_string(),
                                        )
                                        .font(iced_fonts::BOOTSTRAP_FONT),
                                    ),
                            )
                            .padding(10)
                            .style(button_blue_style),
                        )
                        .align_y(iced::Alignment::Center), //TODO.padding([7, 20, 7, 7]),
                )
                .style(card_container_style),
            )
            .spacing(20);
        let content = iced::widget::Scrollable::new(wrap(15, options.into()));
        iced::widget::Column::new()
            .push(top_row)
            .push(content)
            .spacing(5)
            .into()
    }
}
