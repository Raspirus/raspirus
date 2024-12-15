use rust_i18n::t;

use crate::{
    backend::config_file::Config,
    frontend::{
        iced::{wrap, ConfigValue, ErrorCase, Message, Raspirus, UpdateState},
        theme::{
            button::{button_blue_style, button_primary_style},
            container::card_container_style,
            icon::{settings_icon_style, white_icon_style},
            svg::{svg_icon, svg_plain},
            toggle::toggler_style,
            GRAY_COLOR, PRIMARY_COLOR,
        },
    },
};

impl Raspirus {
    pub fn settings(
        &self,
        config: &Config,
        update: &UpdateState,
        temp_scale: usize,
    ) -> iced::Element<Message> {
        let cpus = num_cpus::get();

        let top_row = iced::widget::Column::new()
            .push(
                iced::widget::Row::new()
                    .push(
                        iced::widget::Button::new(
                            iced::widget::Row::new()
                                .push(svg_icon(crate::HOME).style(white_icon_style))
                                .push(iced::widget::container(iced::widget::text(t!("back_btn"))))
                                .spacing(10)
                                .align_y(iced::Alignment::Center),
                        )
                        .on_press(Message::OpenMain)
                        .style(button_primary_style)
                        .padding(10),
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
                            svg_plain(crate::DATABASE_IMPORT)
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
                                        match update {
                                            UpdateState::Loaded => svg_icon(crate::REFRESH),
                                            UpdateState::Updating => {
                                                svg_icon(crate::HOURGLASS_HIGH)
                                            }
                                            UpdateState::Updated => svg_icon(crate::CHECK),
                                        }
                                        .style(white_icon_style),
                                    )
                                    .spacing(10)
                                    .align_y(iced::Alignment::Center),
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
                        .padding(iced::Padding::new(10.0).right),
                )
                .style(card_container_style),
            )
            .push(
                iced::widget::container(
                    iced::widget::Row::new()
                        .push(
                            svg_plain(crate::ARROW_BADGE_UP)
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
                        .push(iced_aw::widgets::NumberInput::new(
                            config.max_matches,
                            config.min_matches + 1..usize::MAX,
                            |matches| Message::ConfigChanged {
                                value: ConfigValue::MaxMatch(matches),
                            },
                        ))
                        .align_y(iced::Alignment::Center)
                        .padding(iced::padding::Padding::new(10.0).right),
                )
                .style(card_container_style),
            )
            .push(
                iced::widget::container(
                    iced::widget::Row::new()
                        .push(
                            svg_plain(crate::ARROW_BADGE_DOWN)
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
                        .push(iced_aw::widgets::NumberInput::new(
                            config.min_matches,
                            0..config.max_matches,
                            |matches| Message::ConfigChanged {
                                value: ConfigValue::MinMatch(matches),
                            },
                        ))
                        .align_y(iced::Alignment::Center)
                        .padding(iced::padding::Padding::new(10.0).right),
                )
                .style(card_container_style),
            )
            .push(
                iced::widget::container(
                    iced::widget::Row::new()
                        .push(
                            svg_plain(crate::CLIPBOARD_DATA)
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
                            svg_plain(crate::ZOOM)
                                .height(64)
                                .width(64)
                                .style(settings_icon_style),
                        )
                        .push(iced::widget::Space::with_width(10))
                        .push(
                            iced::widget::Column::new()
                                .push(iced::widget::text(t!("set_scale")).size(20))
                                .push(iced::widget::Space::with_height(5))
                                .push(iced::widget::text(t!("set_scale_desc")).size(14).style(
                                    |_| iced::widget::text::Style {
                                        color: Some(GRAY_COLOR),
                                    },
                                ))
                                .width(iced::Length::Fill),
                        )
                        .push(iced::widget::horizontal_space())
                        .push(
                            iced::widget::Column::new()
                                .push(iced::widget::Text::new(format!("{}%", temp_scale)))
                                .push(
                                    iced::widget::Slider::new(
                                        25.0..=200.0,
                                        temp_scale as f64,
                                        |scale| Message::ApplyScale {
                                            scale: scale as usize,
                                        },
                                    )
                                    .step(5),
                                )
                                .spacing(5)
                                .align_x(iced::Alignment::Center),
                        )
                        .push(
                            iced::widget::Button::new(iced::widget::Text::new(t!("apply")))
                                .on_press(Message::ConfigChanged {
                                    value: ConfigValue::Scale(temp_scale),
                                })
                                .style(button_blue_style)
                                .padding(10),
                        )
                        .spacing(5)
                        .align_y(iced::Alignment::Center)
                        .padding(iced::padding::Padding::new(10.0).right),
                )
                .style(card_container_style),
            )
            .push(
                iced::widget::container(
                    iced::widget::Row::new()
                        .push(
                            svg_plain(crate::THEME_TOGGLE)
                                .height(64)
                                .width(64)
                                .style(settings_icon_style),
                        )
                        .push(iced::widget::Space::with_width(10))
                        .push(
                            iced::widget::Column::new()
                                .push(iced::widget::text(t!("set_toggle_theme")).size(20))
                                .push(iced::widget::Space::with_height(5))
                                .push(
                                    iced::widget::text(t!("set_toggle_theme_desc"))
                                        .size(14)
                                        .style(|_| iced::widget::text::Style {
                                            color: Some(GRAY_COLOR),
                                        }),
                                )
                                .width(iced::Length::Fill),
                        )
                        .push(iced::widget::horizontal_space())
                        .push(
                            iced::widget::Toggler::new(config.dark_mode)
                                .on_toggle(|_| Message::ConfigChanged {
                                    value: ConfigValue::Dark(!self.dark_mode),
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
                            svg_plain(crate::CPU)
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
                        .push(iced_aw::widgets::NumberInput::new(
                            config.max_threads,
                            1..cpus * 2 + 1,
                            |threads| Message::ConfigChanged {
                                value: ConfigValue::MaxThreads(threads),
                            },
                        ))
                        .align_y(iced::Alignment::Center)
                        .padding(iced::padding::Padding::new(10.0).right),
                )
                .style(card_container_style),
            )
            .push(
                iced::widget::container(
                    iced::widget::Row::new()
                        .push(
                            svg_plain(crate::FILE_DOWNLOAD)
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
                            iced::widget::Button::new(
                                iced::widget::Row::new()
                                    .push(iced::widget::text(t!("set_download_logs_btn")))
                                    .push(svg_icon(crate::DOWNLOAD).style(white_icon_style))
                                    .spacing(10),
                            )
                            .on_press(Message::DownloadLogs)
                            .padding(10)
                            .style(button_blue_style),
                        )
                        .align_y(iced::Alignment::Center)
                        .padding(iced::padding::Padding::new(10.0).right),
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
