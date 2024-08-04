use crate::frontend::iced::{Message, Raspirus};

impl Raspirus {
    pub fn main_menu(&self) -> iced::Element<Message> {
        let top_row = iced::widget::row!(
            // language selection
            iced_aw::widgets::DropDown::new(
                // button to trigger dropdow
                iced::widget::Row::new().push(
                    iced::widget::Button::new(iced::widget::Text::new(&self.language))
                        .on_press(Message::LanguageSelectExpand),
                ),
                // dropdown selection list
                iced_aw::widgets::SelectionList::new(
                    &crate::SUPPORTED_LANGUAGES,
                    |_idx: usize, language: String| Message::LanguageChanged(language),
                )
                .height(iced::Length::Shrink),
                // expanded state
                self.language_expanded,
            ),
            // spacer
            iced::widget::horizontal_space(),
            // settings button
            iced::widget::button("settings").on_press(Message::OpenSettings)
        )
        .align_items(iced::Alignment::Center);
        iced::Element::new(top_row)
    }
}
