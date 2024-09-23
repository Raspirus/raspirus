use std::borrow::Cow;

use rust_i18n::t;

use crate::frontend::{
    iced::{wrap, Message, Raspirus},
    theme::{
        button::button_primary_style,
        container::card_container_style,
        icon::{info_icon_style, white_icon_style},
        svg::{svg_icon, svg_plain},
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
                .push(icon.height(64).width(64).style(info_icon_style))
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
                                .push(svg_icon(crate::HOME).style(white_icon_style))
                                .push(iced::widget::container(iced::widget::text(t!("back_btn"))))
                                .spacing(10)
                                .align_y(iced::Alignment::Center),
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
                        .push(svg_plain(crate::LOGO_VECTOR).width(iced::Length::FillPortion(2)))
                        .push(svg_plain(crate::USB_VECTOR).width(iced::Length::FillPortion(2)))
                        .padding(20)
                        .align_y(iced::Alignment::Center),
                )
                .style(card_container_style),
            )
            .push(Self::info_card(
                svg_plain(crate::HEXAGON_LETTER),
                t!("app_name"),
                t!("app_title"),
            ))
            .push(Self::info_card(
                svg_plain(crate::FILE_DESCRIPTION),
                t!("description"),
                t!("description_val"),
            ))
            .push(Self::info_card(
                svg_plain(crate::USER_CODE),
                t!("maintainers"),
                "Benjamin Demetz, Felix Hell Björn".into(),
            ))
            .push(Self::info_card(
                svg_plain(crate::GIT_COMMIT),
                t!("version"),
                env!("CARGO_PKG_VERSION").into(),
            ))
            .push(Self::info_card(
                svg_plain(crate::LICENSE),
                t!("license"),
                t!("license_val"),
            ))
            .push(Self::info_card(
                svg_plain(crate::GLOBE),
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
