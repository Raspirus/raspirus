use crate::{
    backend::config_file::Config,
    frontend::iced::{wrap, Message, Raspirus, UpdateState},
};

impl Raspirus {
    pub fn settings(&self, config: &Config, update: &UpdateState) -> iced::Element<Message> {
        let top_row = iced::widget::Row::new()
            .push(iced::widget::horizontal_space())
            .push(iced::widget::Button::new("back").on_press(Message::OpenMain));

        let mut options = iced::widget::Column::new();
        options = options.push(iced::widget::Container::new(
            iced::widget::Row::new()
                .push(iced::widget::Text::new("Updated"))
                .push(iced::widget::horizontal_space())
                .push(
                    iced::widget::Button::new(iced::widget::Text::new(match update {
                        UpdateState::Loaded => iced_aw::Bootstrap::ArrowUpCircleFill,
                        UpdateState::Updating => iced_aw::Bootstrap::ArrowClockwise,
                        UpdateState::Updated => iced_aw::Bootstrap::Check,
                    }.to_string()).font(iced_aw::BOOTSTRAP_FONT))
                        .on_press(Message::UpdateRules),
                ),
        ));
        let content = iced::widget::Scrollable::new(options);
        iced::widget::Column::new()
            .push(top_row)
            .push(iced::widget::horizontal_rule(5))
            .push(content)
            .spacing(5)
            .padding(10)
            .into()
    }
}
