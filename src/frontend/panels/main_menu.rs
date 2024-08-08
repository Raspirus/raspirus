use std::path::Path;

use crate::frontend::iced::{Message, Raspirus};

impl Raspirus {
    pub fn main_menu(&self) -> iced::Element<Message> {
        let top_row = iced::widget::Row::new()
            // language selection
            .push(iced_aw::widgets::DropDown::new(
                // button to trigger dropdown
                iced::widget::Row::new().push(
                    iced::widget::Button::new(iced::widget::Text::new(&self.language))
                        .on_press(Message::ToggleLanguage),
                ),
                // dropdown selection list
                iced_aw::widgets::SelectionList::new(
                    &crate::SUPPORTED_LANGUAGES,
                    |_idx: usize, language: String| Message::LanguageChanged(language),
                )
                .height(iced::Length::Shrink),
                // expanded state
                self.language_expanded,
            ))
            // spacer
            .push(iced::widget::horizontal_space())
            // settings button
            .push(iced::widget::button("settings").on_press(Message::OpenSettings))
            // ite allignment
            .align_items(iced::Alignment::Center);

        let middle_row = iced::widget::Column::new()
            .push(
                iced::widget::Row::new()
                    .push(
                        iced::widget::TextInput::new("", &self.path_selected.to_string_lossy())
                            .on_input(|content| {
                                Message::PathChanged(Path::new(&content).to_path_buf())
                            }),
                    )
                    //.push(iced::widget::horizontal_space().width(10))
                    .push(
                        iced::widget::Button::new(
                            iced::widget::Text::new(iced_aw::Bootstrap::Folder.to_string())
                                .font(iced_aw::BOOTSTRAP_FONT),
                        )
                        .on_press(Message::SelectPath),
                    )
                    .spacing(5),
            )
            .push(
                iced::widget::Button::new(iced::widget::Text::new("Start"))
                    .on_press(Message::StartScan),
            )
            .spacing(5);

        iced::widget::Column::new()
            .push(top_row)
            .push(iced::widget::vertical_space())
            .push(middle_row)
            .push(iced::widget::vertical_space())
            .spacing(5)
            .into()
    }
}
