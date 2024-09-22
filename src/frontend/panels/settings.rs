use rust_i18n::t;

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
        let cpus = num_cpus::get();
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
                                .push(iced::widget::container(iced::widget::text(t!("back_btn")))),
                        )
                        .on_press(Message::OpenMain)
                        .style(button_primary_style)
                        .padding(7),
                    )
                    .push(
                        iced::widget::container(
                            iced::widget::text(t!("settings"))
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
                                .push(iced::widget::text(t!("updater_updated")).size(20))
                                .push(iced::widget::Space::with_height(5))
                                .push(
                                    iced::widget::text(format!(
                                        "{} {}",
                                        t!("updater_description"),
                                        config.rules_version
                                    ))
                                    .size(14)
                                    .style(|_| {
                                        iced::widget::text::Style {
                                            color: Some(GRAY_COLOR),
                                        }
                                    }),
                                )
                                .width(iced::Length::Fill),
                        )
                        .push(iced::widget::horizontal_space())
                        .push(
                            iced::widget::Button::new(
                                iced::widget::Row::new()
                                    .push(iced::widget::Text::new(
                                        match update {
                                            UpdateState::Loaded => t!("updater_update"),
                                            UpdateState::Updating => t!("updater_updating"),
                                            UpdateState::Updated => t!("updater_updated"),
                                        }
                                        .to_uppercase()
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
                                        message: t!("updater_running").to_string(),
                                    },
                                },
                            })
                            .padding(10)
                            .style(button_blue_style),
                        )
                        .align_y(iced::Alignment::Center)
                        .padding(iced::Padding::new(7.0).right),
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
                                .push(iced::widget::text(t!("set_max_matches")).size(20))
                                .push(iced::widget::Space::with_height(5))
                                .push(
                                    iced::widget::text(t!("set_max_matches_desc"))
                                        .size(14)
                                        .style(|_| iced::widget::text::Style {
                                            color: Some(GRAY_COLOR),
                                        }),
                                )
                                .width(iced::Length::Fill),
                        )
                        .push(iced::widget::horizontal_space())
                        .push(
                            iced_aw::widgets::NumberInput::new(
                                config.max_matches,
                                config.min_matches + 1..usize::MAX,
                                |matches| Message::ConfigChanged {
                                    value: ConfigValue::MaxMatch(matches),
                                },
                            )
                            .font(iced_fonts::BOOTSTRAP_FONT),
                        )
                        .align_y(iced::Alignment::Center)
                        .padding(iced::padding::Padding::new(7.0).right),
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
                                .push(iced::widget::text(t!("set_min_matches")).size(20))
                                .push(iced::widget::Space::with_height(5))
                                .push(
                                    iced::widget::text(t!("set_min_matches_desc"))
                                        .size(14)
                                        .style(|_| iced::widget::text::Style {
                                            color: Some(GRAY_COLOR),
                                        }),
                                )
                                .width(iced::Length::Fill),
                        )
                        .push(iced::widget::horizontal_space())
                        .push(
                            iced_aw::widgets::NumberInput::new(
                                config.min_matches,
                                0..config.max_matches,
                                |matches| Message::ConfigChanged {
                                    value: ConfigValue::MinMatch(matches),
                                },
                            )
                            .font(iced_fonts::BOOTSTRAP_FONT),
                        )
                        .align_y(iced::Alignment::Center)
                        .padding(iced::padding::Padding::new(7.0).right),
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
                                .push(iced::widget::text(t!("set_logging")).size(20))
                                .push(iced::widget::Space::with_height(5))
                                .push(iced::widget::text(t!("set_logging_desc")).size(14).style(
                                    |_| iced::widget::text::Style {
                                        color: Some(GRAY_COLOR),
                                    },
                                ))
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
                            iced::widget::svg::Svg::from_path("src/assets/icons/cpu.svg")
                                .height(64)
                                .width(64)
                                .style(settings_icon_style),
                        )
                        .push(iced::widget::Space::with_width(10))
                        .push(
                            iced::widget::Column::new()
                                .push(iced::widget::text(t!("set_threads")).size(20))
                                .push(iced::widget::Space::with_height(5))
                                .push(
                                    iced::widget::text(format!(
                                        "{} ({} {})",
                                        t!("set_threads_desc"),
                                        cpus,
                                        t!("set_threads_desc_rec")
                                    ))
                                    .size(14)
                                    .style(|_| {
                                        iced::widget::text::Style {
                                            color: Some(GRAY_COLOR),
                                        }
                                    }),
                                )
                                .width(iced::Length::Fill),
                        )
                        .push(iced::widget::horizontal_space())
                        .push(
                            iced_aw::widgets::NumberInput::new(
                                config.max_threads,
                                1..cpus * 2 + 1,
                                |threads| Message::ConfigChanged {
                                    value: ConfigValue::MaxThreads(threads),
                                },
                            )
                            .font(iced_fonts::BOOTSTRAP_FONT),
                        )
                        .align_y(iced::Alignment::Center)
                        .padding(iced::padding::Padding::new(7.0).right),
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
                                .push(iced::widget::text(t!("set_download_logs")).size(20))
                                .push(iced::widget::Space::with_height(5))
                                .push(
                                    iced::widget::text(t!("set_download_logs_desc"))
                                        .size(14)
                                        .style(|_| iced::widget::text::Style {
                                            color: Some(GRAY_COLOR),
                                        }),
                                )
                                .width(iced::Length::Fill),
                        )
                        .push(iced::widget::horizontal_space())
                        .push(
                            // TODO: Make this button functional
                            iced::widget::Button::new(
                                iced::widget::Row::new()
                                    .push(iced::widget::text(t!("set_download_logs_btn")))
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
                        .align_y(iced::Alignment::Center)
                        .padding(iced::padding::Padding::new(7.0).right),
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
