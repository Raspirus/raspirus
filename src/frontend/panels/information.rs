use crate::frontend::{iced::{wrap, Message, Raspirus}, theme::{container::RaspirusInfoCard, icon::RaspirusInfoIcon}};

// Icons from Tabler.io: https://tabler.io/icons

impl Raspirus {
    fn info_card<'a>(icon: iced::widget::svg::Svg, title: &'a str, value: &'a str) -> iced::widget::Container<'a, Message> {
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
        .style(iced::theme::Container::Custom(Box::new(RaspirusInfoCard)))
    }

    pub fn information(&self) -> iced::Element<Message> {
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
                Self::info_card(
                    iced::widget::svg::Svg::from_path("src/assets/icons/hexagon-letter-r.svg"),
                    "Name",
                    "Raspirus",
                    )
                )
                .push(
                    Self::info_card(
                        iced::widget::svg::Svg::from_path("src/assets/icons/file-description.svg"),
                        "Description",
                        "Simple signatures-based antivirus for single-board computers like Raspberry Pi",
                    )
                )
                .push(
                    Self::info_card(
                        iced::widget::svg::Svg::from_path("src/assets/icons/user-code.svg"),
                        "Maintainers",
                        "Benjamin Demetz, Felix Hell Björn",
                    )
                )
                .push(
                    Self::info_card(
                        iced::widget::svg::Svg::from_path("src/assets/icons/git-commit.svg"),
                        "Version",
                        env!("CARGO_PKG_VERSION"),
                    )
                )
                .push(
                    Self::info_card(
                        iced::widget::svg::Svg::from_path("src/assets/icons/license.svg"),
                        "License",
                        "Open Source GPLv3",
                    )
                )
                .push(
                    Self::info_card(
                        iced::widget::svg::Svg::from_path("src/assets/icons/globe.svg"),
                        "Website",
                        "https://raspirus.deno.dev",
                    )
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
