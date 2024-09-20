use crate::frontend::{iced::{wrap, Message, Raspirus}, theme::{button::RaspirusButtonPrimary, icon::RaspirusWhiteIcon, PRIMARY_COLOR}};

impl Raspirus {
    pub fn terms(&self) -> iced::Element<Message> {
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
                            iced::widget::text("Terms and Conditions")
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
        
        let text = iced::widget::Container::new(
            iced::widget::text("Hello")
        );

        let content = iced::widget::Scrollable::new(wrap(15, text.into()));
        iced::widget::Column::new()
            .push(top_row)
            .push(content)
            .spacing(5)
            .into()
    }
}
