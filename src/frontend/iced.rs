use iced::Sandbox;

pub struct Raspirus {
    state: State,
    language: String,
    lang_expanded: bool,
}

pub enum State {
    MainMenu,
    Scanning,
    Settings,
    Results,
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenSettings,
    ReturnMain,
    StartScan,
    LanguageSelectExpand,
    LanguageChanged(usize, String),
}

impl Sandbox for Raspirus {
    type Message = Message;

    fn new() -> Self {
        Self {
            state: State::MainMenu,
            language: "en-US".to_owned(),
            lang_expanded: false,
        }
    }

    fn title(&self) -> String {
        "Raspirus".to_owned()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::OpenSettings => self.state = State::Settings,
            Message::ReturnMain => self.state = State::MainMenu,
            Message::StartScan => todo!(),
            Message::LanguageSelectExpand => self.lang_expanded = true,
            Message::LanguageChanged(_, language) => {
                self.lang_expanded = false;
                self.language = language
            }
        }
    }

    fn view(&self) -> iced::Element<Message> {
        let content = match self.state {
            State::MainMenu => self.main_menu(),
            State::Scanning => todo!(),
            State::Settings => self.settings(),
            State::Results => todo!(),
        };
        iced::Element::new(
            iced::widget::Container::new(content)
                .padding(10)
                .center_x()
                .width(iced::Length::Fill),
        )
    }
}

impl Raspirus {
    fn main_menu(&self) -> iced::Element<Message> {
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
                    Message::LanguageChanged,
                )
                .height(iced::Length::Shrink),
                // expanded state
                self.lang_expanded,
            ),
            // spacer
            iced::widget::horizontal_space(),
            // settings button
            iced::widget::button("settings").on_press(Message::OpenSettings)
        )
        .align_items(iced::Alignment::Center);
        iced::Element::new(top_row)
    }

    fn settings(&self) -> iced::Element<Message> {
        iced::Element::new(iced::widget::Button::new("back").on_press(Message::ReturnMain))
    }
}
