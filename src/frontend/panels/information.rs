use crate::frontend::{
    iced::{wrap, Message, Raspirus},
    theme::{
        button::RaspirusButtonPrimary,
        container::{RaspirusCard, RaspirusIconContainer},
        icon::{RaspirusInfoIcon, RaspirusWhiteIcon},
        GRAY_COLOR, PRIMARY_COLOR,
    },
};

// Icons from Tabler.io: https://tabler.io/icons

impl Raspirus {
    fn info_card<'a>(
        icon: iced::widget::svg::Svg,
        title: &'a str,
        value: &'a str,
    ) -> iced::widget::Container<'a, Message> {
        iced::widget::container(
            iced::widget::Row::new()
                .push(
                    iced::widget::container(
                        icon.height(48)
                            .width(48)
                            .style(iced::theme::Svg::Custom(Box::new(RaspirusInfoIcon))),
                    )
                    .padding(15)
                    .style(iced::theme::Container::Custom(Box::new(
                        RaspirusIconContainer,
                    ))),
                )
                //.push(iced::widget::vertical_rule(5))
                .push(iced::widget::Space::with_width(10))
                .push(
                    iced::widget::Column::new()
                        .push(iced::widget::text(title).size(20))
                        .push(iced::widget::Space::with_height(5))
                        .push(iced::widget::text(value).size(14).style(GRAY_COLOR))
                        .width(iced::Length::Fill),
                )
                .align_items(iced::Alignment::Center)
                .width(iced::Length::Fill)
                .padding(7),
        )
        .style(iced::theme::Container::Custom(Box::new(RaspirusCard)))
    }

    pub fn information(&self) -> iced::Element<Message> {
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
                            iced::widget::text("Information")
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
                            iced::widget::svg::Svg::from_path("src/assets/logo-vector.svg")
                                .width(iced::Length::FillPortion(2)),
                        )
                        .push(
                            iced::widget::svg::Svg::from_path("src/assets/usb-vector.svg")
                                .width(iced::Length::FillPortion(2)),
                        )
                        .padding(20)
                        .align_items(iced::Alignment::Center),
                )
                .style(iced::theme::Container::Custom(Box::new(RaspirusCard))),
            )
            .push(Self::info_card(
                iced::widget::svg::Svg::from_path("src/assets/icons/hexagon-letter-r.svg"),
                "Name",
                "Raspirus",
            ))
            .push(Self::info_card(
                iced::widget::svg::Svg::from_path("src/assets/icons/file-description.svg"),
                "Description",
                "Simple signatures-based antivirus for single-board computers like Raspberry Pi",
            ))
            .push(Self::info_card(
                iced::widget::svg::Svg::from_path("src/assets/icons/user-code.svg"),
                "Maintainers",
                "Benjamin Demetz, Felix Hell Björn",
            ))
            .push(Self::info_card(
                iced::widget::svg::Svg::from_path("src/assets/icons/git-commit.svg"),
                "Version",
                env!("CARGO_PKG_VERSION"),
            ))
            .push(Self::info_card(
                iced::widget::svg::Svg::from_path("src/assets/icons/license.svg"),
                "License",
                "Open Source GPLv3",
            ))
            .push(Self::info_card(
                iced::widget::svg::Svg::from_path("src/assets/icons/globe.svg"),
                "Website",
                "https://raspirus.deno.dev",
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
