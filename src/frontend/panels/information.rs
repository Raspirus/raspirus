use std::borrow::Cow;

use rust_i18n::t;

use crate::frontend::{
    iced::{wrap, Message, Raspirus},
    theme::{
        button::button_primary_style,
        container::card_container_style,
        icon::{info_icon_style, white_icon_style},
        GRAY_COLOR, PRIMARY_COLOR,
    },
};

impl Raspirus {
    fn info_card<'a>(
        icon: iced::widget::Svg<'a>,
        title: Cow<'a, str>,
        value: Cow<'a, str>,
    ) -> iced::widget::Container<'a, Message> {
        iced::widget::container(
            iced::widget::Row::new()
                .push(
                    icon.height(64).width(64).style(info_icon_style),
                )
                //.push(iced::widget::vertical_rule(5))
                .push(iced::widget::Space::with_width(10))
                .push(
                    iced::widget::Column::new()
                        .push(iced::widget::text(title).size(20))
                        .push(iced::widget::Space::with_height(5))
                        .push(iced::widget::text(value).size(14).style(|_| {
                            iced::widget::text::Style {
                                color: Some(GRAY_COLOR),
                            }
                        }))
                        .width(iced::Length::Fill),
                )
                .align_y(iced::Alignment::Center)
                .width(iced::Length::Fill)
                .padding(7),
        )
        .style(card_container_style)
        .padding(5)
    }

    pub fn information(&self) -> iced::Element<Message> {
        let top_row = iced::widget::Column::new()
            .push(
                iced::widget::Row::new()
                    .push(
                        iced::widget::Button::new(
                            iced::widget::Row::new()
                                .push(
                                    iced::widget::Svg::from_path("src/assets/icons/home.svg")
                                        .height(20)
                                        .width(20)
                                        .style(white_icon_style),
                                )
                                .push(
                                    iced::widget::container(iced::widget::text(t!("back_btn"))),
                                ),
                        )
                        .on_press(Message::OpenMain)
                        .style(button_primary_style)
                        .padding(7),
                    )
                    .push(
                        iced::widget::container(
                            iced::widget::text(t!("info_title"))
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
                            iced::widget::Svg::from_path("src/assets/logo-vector.svg")
                                .width(iced::Length::FillPortion(2)),
                        )
                        .push(
                            iced::widget::Svg::from_path("src/assets/usb-vector.svg")
                                .width(iced::Length::FillPortion(2)),
                        )
                        .padding(20)
                        .align_y(iced::Alignment::Center),
                )
                .style(card_container_style),
            )
            .push(Self::info_card(
                iced::widget::Svg::from_path("src/assets/icons/hexagon-letter-r.svg"),
                t!("app_name"),
                t!("app_title"),
            ))
            .push(Self::info_card(
                iced::widget::Svg::from_path("src/assets/icons/file-description.svg"),
                t!("description"),
                t!("description_val"),
            ))
            .push(Self::info_card(
                iced::widget::Svg::from_path("src/assets/icons/user-code.svg"),
                t!("maintainers"),
                "Benjamin Demetz, Felix Hell Björn".into(),
            ))
            .push(Self::info_card(
                iced::widget::Svg::from_path("src/assets/icons/git-commit.svg"),
                t!("version"),
                env!("CARGO_PKG_VERSION").into(),
            ))
            .push(Self::info_card(
                iced::widget::Svg::from_path("src/assets/icons/license.svg"),
                t!("license"),
                t!("license_val"),
            ))
            .push(Self::info_card(
                iced::widget::Svg::from_path("src/assets/icons/globe.svg"),
                t!("website"),
                "https://raspirus.deno.dev".into(),
            ))
            .spacing(20);
        let content = iced::widget::Scrollable::new(wrap(15, options.into()));
        iced::widget::Column::new()
            .push(top_row)
            .push(content)
            .spacing(5)
            .into()
    }
}
