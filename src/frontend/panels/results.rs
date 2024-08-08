use crate::frontend::iced::{Message, Raspirus};

impl Raspirus {
    pub fn results(&self) -> iced::Element<Message> {
        match &self.state {
            crate::frontend::iced::State::Results(tagged, skipped) => {
                let mut tagged_list = iced::widget::Column::new()
                    .push(iced::widget::Text::new("Found files"))
                    .spacing(5);

                for (tag, expanded) in tagged {
                    let expanded = *expanded;
                    let mut descriptions = iced::widget::Column::new().height(iced::Length::Shrink);

                    if expanded {
                        // build rules list for each match
                        for rule in tag.descriptions.clone() {
                            descriptions = descriptions.push(
                                iced::widget::Row::new()
                                    .push(iced::widget::Text::new(rule.rule_name))
                                    .push(iced::widget::horizontal_space())
                                    .push(iced::widget::Text::new(rule.rule_description)),
                            )
                        }
                    }

                    // build the dropdown
                    tagged_list = tagged_list.push({
                        let mut card = iced_aw::widgets::Card::new(
                            iced::widget::Row::new()
                                .push(iced::widget::Text::new(tag.path.to_string_lossy()))
                                .push(iced::widget::horizontal_space())
                                .push(iced_aw::widgets::Badge::new(iced::widget::Text::new(
                                    format!("{}", tag.rule_count),
                                )))
                                .push(
                                    iced::widget::Button::new(
                                        iced::widget::Text::new(
                                            if expanded {
                                                iced_aw::Bootstrap::CaretDownFill
                                            } else {
                                                iced_aw::Bootstrap::CaretLeftFill
                                            }
                                            .to_string(),
                                        )
                                        .font(iced_aw::BOOTSTRAP_FONT),
                                    )
                                    .on_press(
                                        Message::ToggleCard(crate::frontend::iced::Card::Tagged(
                                            tag.clone(),
                                        )),
                                    ),
                                )
                                .spacing(5)
                                .height(iced::Length::Shrink),
                            descriptions,
                        );
                        if !expanded {
                            card = card
                                .foot(iced::widget::horizontal_space().height(0))
                                .padding_foot(0.into())
                                .padding_body(0.into());
                        }
                        card
                    });
                }

                let mut skipped_list = iced::widget::Column::new()
                    .push(iced::widget::Text::new("Skipped files"))
                    .spacing(5);

                for (skip, expanded) in skipped {
                    let expanded = *expanded;
                    let mut description = iced::widget::Column::new()
                        .height(iced::Length::Shrink);

                    if expanded {
                        description = description.push(iced::widget::Text::new(skip.reason.clone()));
                    }
 
                    // build the dropdown
                    skipped_list = skipped_list.push({
                        let mut card = iced_aw::widgets::Card::new(
                            iced::widget::Row::new()
                                .push(iced::widget::Text::new(skip.path.to_string_lossy()))
                                .push(iced::widget::horizontal_space())
                                .push(
                                    iced::widget::Button::new(
                                        iced::widget::Text::new(
                                            if expanded {
                                                iced_aw::Bootstrap::CaretDownFill
                                            } else {
                                                iced_aw::Bootstrap::CaretLeftFill
                                            }
                                            .to_string(),
                                        )
                                        .font(iced_aw::BOOTSTRAP_FONT),
                                    )
                                    .on_press(
                                        Message::ToggleCard(crate::frontend::iced::Card::Skipped(
                                            skip.clone(),
                                        )),
                                    ),
                                )
                                .spacing(5)
                                .height(iced::Length::Shrink),
                            description,
                        );
                        if !expanded {
                            card = card
                                .foot(iced::widget::horizontal_space().height(0))
                                .padding_foot(0.into())
                                .padding_body(0.into());
                        }
                        card
                    });
                }

                iced::widget::Column::new()
                    .push(tagged_list)
                    .push(skipped_list)
                    .spacing(5)
                    .into()
            }
            _ => iced::widget::Text::new("Wrong state").into(),
        }
    }
}
