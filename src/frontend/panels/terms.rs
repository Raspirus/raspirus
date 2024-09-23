use rust_i18n::t;

use crate::frontend::{
    iced::{wrap, Message, Raspirus},
    theme::{button::button_primary_style, icon::white_icon_style, svg::svg_icon, PRIMARY_COLOR},
};

impl Raspirus {
    pub fn terms(&self) -> iced::Element<Message> {
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
                            iced::widget::text(t!("permissions_title"))
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

        let text = iced::widget::Container::new(
            iced::widget::text(t!("permissions_text"))
                .size(20)
                .wrapping(iced::widget::text::Wrapping::Word),
        )
        .padding([10, 20]);

        let content = iced::widget::Scrollable::new(wrap(15, text.into()));
        iced::widget::Column::new()
            .push(top_row)
            .push(content)
            .spacing(5)
            .into()
    }
}
