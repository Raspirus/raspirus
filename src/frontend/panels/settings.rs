use crate::{
    backend::config_file::Config,
    frontend::{
        iced::{wrap, ConfigValue, Message, Raspirus, UpdateState},
        theme::{
            button::RaspirusButtonPrimary,
            container::RaspirusCard,
            icon::{RaspirusSettingsIcon, RaspirusWhiteIcon},
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
                                        .style(iced::theme::Svg::Custom(Box::new(
                                            RaspirusWhiteIcon,
                                        ))),
                                )
                                .push(
                                    iced::widget::container(iced::widget::text("HOME"))
                                        .padding([0, 0, 0, 5]),
                                ),
                        )
                        .on_press(Message::OpenMain)
                        .style(iced::theme::Button::Custom(Box::new(RaspirusButtonPrimary)))
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
                                .style(PRIMARY_COLOR),
                        )
                        .padding([0, 10]),
                    )
                    .padding([5, 0])
                    .push(iced::widget::horizontal_space())
                    .align_items(iced::Alignment::Center),
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
                            .style(iced::theme::Svg::Custom(Box::new(RaspirusSettingsIcon))),
                        )
                        .push(iced::widget::Space::with_width(10))
                        .push(
                            iced::widget::Column::new()
                                .push(iced::widget::text("Updated").size(20))
                                .push(iced::widget::Space::with_height(5))
                                .push(
                                    iced::widget::text("Updates the Rules")
                                        .size(14)
                                        .style(GRAY_COLOR),
                                )
                                .width(iced::Length::Fill),
                        )
                        .push(iced::widget::horizontal_space())
                        .push(
                            iced::widget::Button::new(
                                iced::widget::Text::new(
                                    match update {
                                        UpdateState::Loaded => {
                                            iced_aw::Bootstrap::ArrowUpCircleFill
                                        }
                                        UpdateState::Updating => iced_aw::Bootstrap::ArrowClockwise,
                                        UpdateState::Updated => iced_aw::Bootstrap::Check,
                                    }
                                    .to_string(),
                                )
                                .font(iced_aw::BOOTSTRAP_FONT),
                            )
                            .on_press(Message::UpdateRules),
                        )
                        .align_items(iced::Alignment::Center)
                        .padding(7),
                )
                .style(iced::theme::Container::Custom(Box::new(RaspirusCard))),
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
                            .style(iced::theme::Svg::Custom(Box::new(RaspirusSettingsIcon))),
                        )
                        .push(iced::widget::Space::with_width(10))
                        .push(
                            iced::widget::Column::new()
                                .push(iced::widget::text("Max Matches").size(20))
                                .push(iced::widget::Space::with_height(5))
                                .push(
                                    iced::widget::text("Maximum amount of Rules that should match")
                                        .size(14)
                                        .style(GRAY_COLOR),
                                )
                                .width(iced::Length::Fill),
                        )
                        .push(iced::widget::horizontal_space())
                        .push(iced_aw::widgets::NumberInput::new(
                            config.max_matches,
                            usize::MAX,
                            |matches| Message::ConfigChanged {
                                value: ConfigValue::MaxMatch(matches),
                            },
                        ))
                        .align_items(iced::Alignment::Center)
                        .padding(7),
                )
                .style(iced::theme::Container::Custom(Box::new(RaspirusCard))),
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
                            .style(iced::theme::Svg::Custom(Box::new(RaspirusSettingsIcon))),
                        )
                        .push(iced::widget::Space::with_width(10))
                        .push(
                            iced::widget::Column::new()
                                .push(iced::widget::text("Min Matches").size(20))
                                .push(iced::widget::Space::with_height(5))
                                .push(
                                    iced::widget::text("Minumum amount of Rules that should match")
                                        .size(14)
                                        .style(GRAY_COLOR),
                                )
                                .width(iced::Length::Fill),
                        )
                        .push(iced::widget::horizontal_space())
                        .push(iced_aw::widgets::NumberInput::new(
                            config.min_matches,
                            config.max_matches,
                            |matches| Message::ConfigChanged {
                                value: ConfigValue::MinMatch(matches),
                            },
                        ))
                        .align_items(iced::Alignment::Center)
                        .padding(7),
                )
                .style(iced::theme::Container::Custom(Box::new(RaspirusCard))),
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
                            .style(iced::theme::Svg::Custom(Box::new(RaspirusSettingsIcon))),
                        )
                        .push(iced::widget::Space::with_width(10))
                        .push(
                            iced::widget::Column::new()
                                .push(iced::widget::text("Logging").size(20))
                                .push(iced::widget::Space::with_height(5))
                                .push(
                                    iced::widget::text("Activates debug logging")
                                        .size(14)
                                        .style(GRAY_COLOR),
                                )
                                .width(iced::Length::Fill),
                        )
                        .push(iced::widget::horizontal_space())
                        .push(
                            iced::widget::Toggler::new(None, config.logging_is_active, |logging| {
                                Message::ConfigChanged {
                                    value: ConfigValue::Logging(logging),
                                }
                            })
                            .width(iced::Length::Shrink),
                        )
                        .align_items(iced::Alignment::Center)
                        .padding(7),
                )
                .style(iced::theme::Container::Custom(Box::new(RaspirusCard))),
            )
            .push(
                iced::widget::container(
                    iced::widget::Row::new()
                        .push(
                            iced::widget::svg::Svg::from_path("src/assets/icons/file-download.svg")
                                .height(64)
                                .width(64)
                                .style(iced::theme::Svg::Custom(Box::new(RaspirusSettingsIcon))),
                        )
                        .push(iced::widget::Space::with_width(10))
                        .push(
                            iced::widget::Column::new()
                                .push(iced::widget::text("Download logs").size(20))
                                .push(iced::widget::Space::with_height(5))
                                .push(
                                    iced::widget::text("Copies the Logs to your Downloads folder")
                                        .size(14)
                                        .style(GRAY_COLOR),
                                )
                                .width(iced::Length::Fill),
                        )
                        .push(iced::widget::horizontal_space())
                        .push(
                            // TODO: Make this button functional
                            iced::widget::Button::new(
                                iced::widget::Text::new(
                                    iced_aw::Bootstrap::ArrowDownCircleFill.to_string(),
                                )
                                .font(iced_aw::BOOTSTRAP_FONT),
                            ),
                        )
                        .align_items(iced::Alignment::Center)
                        .padding(7),
                )
                .style(iced::theme::Container::Custom(Box::new(RaspirusCard))),
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
