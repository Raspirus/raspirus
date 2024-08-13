use crate::{
    backend::config_file::Config,
    frontend::iced::{wrap, ConfigValue, Message, Raspirus, UpdateState},
};

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
                iced::widget::Row::new()
                    .push(iced::widget::Text::new("Updated"))
                    .push(iced::widget::horizontal_space())
                    .push(
                        iced::widget::Button::new(
                            iced::widget::Text::new(
                                match update {
                                    UpdateState::Loaded => iced_aw::Bootstrap::ArrowUpCircleFill,
                                    UpdateState::Updating => iced_aw::Bootstrap::ArrowClockwise,
                                    UpdateState::Updated => iced_aw::Bootstrap::Check,
                                }
                                .to_string(),
                            )
                            .font(iced_aw::BOOTSTRAP_FONT),
                        )
                        .on_press(Message::UpdateRules),
                    ),
            )
            .push(
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
            .push(
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
            .push(
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
            .spacing(15);
        let content = iced::widget::Scrollable::new(wrap(15, options.into()));
        iced::widget::Column::new()
            .push(top_row)
            .push(content)
            .spacing(5)
            .into()
    }
}
