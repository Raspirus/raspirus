use crate::{
    backend::config_file::Config,
    frontend::{
        iced::{wrap, ConfigValue, Message, Raspirus, UpdateState},
        theme::{container::RaspirusCard, icon::RaspirusSettingsIcon, GRAY_COLOR},
    },
};

/*
        iced::widget::container(
        iced::widget::Row::new()
            .push(
                icon.height(64).width(64).style(iced::theme::Svg::Custom(Box::new(RaspirusInfoIcon)))
            )
            //.push(iced::widget::vertical_rule(5))
            .push(iced::widget::Space::with_width(10))
            .push(
                iced::widget::Column::new()
                    .push(iced::widget::text(title).size(20))
                    .push(iced::widget::Space::with_height(5))
                    .push(iced::widget::text(value).size(14))
                    .width(iced::Length::Fill)
            )
            .align_items(iced::Alignment::Center)
            .width(iced::Length::Fill)
            .padding(7)
        )
        .style(iced::theme::Container::Custom(Box::new(RaspirusCard)))
*/

impl Raspirus {
    pub fn settings(&self, config: &Config, update: &UpdateState) -> iced::Element<Message> {
        let top_row = iced::widget::Column::new()
            .push(
                iced::widget::Row::new()
                    .push(iced::widget::horizontal_space())
                    .push(iced::widget::Button::new("back").on_press(Message::OpenMain)),
            )
            .push(iced::widget::horizontal_rule(5))
            .padding(10);

        let options = iced::widget::Column::new()
            .push(
                iced::widget::container(
                    iced::widget::Row::new()
                        .push(
                            iced::widget::svg::Svg::from_path("src/assets/icons/database-import.svg")
                                .height(64)
                                .width(64)
                                .style(iced::theme::Svg::Custom(Box::new(RaspirusSettingsIcon))),
                        )
                        .push(iced::widget::Space::with_width(10))
                        .push(
                            iced::widget::Column::new()
                                .push(iced::widget::text("Updated").size(20))
                                .push(iced::widget::Space::with_height(5))
                                .push(iced::widget::text("Updates the Rules")
                                        .size(14)
                                        .style(GRAY_COLOR)
                                    )
                                .width(iced::Length::Fill)
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
                        ).align_items(iced::Alignment::Center)
                        .padding(7),
                )
                .style(iced::theme::Container::Custom(Box::new(RaspirusCard))),
            )
            .push(
                iced::widget::container(
                    iced::widget::Row::new()
                        .push(iced::widget::Text::new("MinMatches"))
                        .push(iced::widget::horizontal_space())
                        .push(iced_aw::widgets::NumberInput::new(
                            config.min_matches,
                            config.max_matches,
                            |matches| Message::ConfigChanged {
                                value: ConfigValue::MinMatch(matches),
                            },
                        )),
                )
                .style(iced::theme::Container::Custom(Box::new(RaspirusCard))),
            )
            .push(
                iced::widget::container(
                    iced::widget::Row::new()
                        .push(iced::widget::Text::new("MaxMatches"))
                        .push(iced::widget::horizontal_space())
                        .push(iced_aw::widgets::NumberInput::new(
                            config.max_matches,
                            usize::MAX,
                            |matches| Message::ConfigChanged {
                                value: ConfigValue::MaxMatch(matches),
                            },
                        )),
                )
                .style(iced::theme::Container::Custom(Box::new(RaspirusCard))),
            )
            .push(
                iced::widget::container(
                    iced::widget::Row::new()
                        .push(iced::widget::Text::new("Logging"))
                        .push(iced::widget::horizontal_space())
                        .push(
                            iced::widget::Toggler::new(None, config.logging_is_active, |logging| {
                                Message::ConfigChanged {
                                    value: ConfigValue::Logging(logging),
                                }
                            })
                            .width(iced::Length::Shrink),
                        ),
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
