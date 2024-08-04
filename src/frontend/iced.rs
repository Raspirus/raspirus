use iced::Sandbox;

pub struct Raspirus {
    pub state: State,
    pub language: String,
    pub language_expanded: bool,
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
    LanguageChanged(String),
}

impl Sandbox for Raspirus {
    type Message = Message;

    fn new() -> Self {
        Self {
            state: State::MainMenu,
            language: "en-US".to_owned(),
            language_expanded: false,
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
            Message::LanguageSelectExpand => self.language_expanded = !self.language_expanded,
            Message::LanguageChanged(language) => {
                self.language_expanded = false;
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
