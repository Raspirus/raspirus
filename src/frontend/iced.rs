use std::path::PathBuf;
use iced::futures::channel::mpsc;
use log::debug;

use crate::backend::yara_scanner::YaraScanner;

pub struct Raspirus {
    pub state: State,
    pub language: String,
    pub language_expanded: bool,
    pub path_selected: PathBuf,
}

pub enum State {
    MainMenu,
    Scanning,
    Settings,
    Results,
}

#[derive(Debug, Clone)]
pub enum Message {
    // location messages
    OpenSettings,
    OpenMain,
    // action messages
    StartScan,
    ToggleLanguage,
    SelectPath,
    // update messages
    PathChanged(PathBuf),
    LanguageChanged(String),
}

impl iced::Application for Raspirus {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        (Self {
            state: State::MainMenu,
            language: "en-US".to_owned(),
            language_expanded: false,
            path_selected: PathBuf::new(),
        }, iced::Command::none())
    }

    fn title(&self) -> String {
        "Raspirus".to_owned()
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        debug!("Message: {:?}", message);
        match message { 
            Message::OpenSettings => {
                self.state = State::Settings;
                iced::Command::none()
            },
            Message::OpenMain => {
                self.state = State::MainMenu;
                iced::Command::none()
            },
            Message::StartScan => {
                let scanner_path = self.path_selected.clone();
                iced::Command::perform(async {
                let mut scanner = YaraScanner::new(None).unwrap();
                scanner.start(scanner_path).unwrap();
            }, |_| Message::OpenMain)
            },
            Message::ToggleLanguage => {
                self.language_expanded = !self.language_expanded;
                iced::Command::none()
            }
            Message::SelectPath => iced::Command::perform(
                async {
                    native_dialog::FileDialog::new()
                        .set_location("~")
                        .show_open_single_dir()
                        .expect("Failed to select file")
                        .unwrap_or_default()
                },
                |result| Message::PathChanged(result),
            ),
            Message::PathChanged(new_path) => {
                self.path_selected = new_path;
                iced::Command::none()
            }
            Message::LanguageChanged(language) => {
                self.language_expanded = false;
                self.language = language;
                iced::Command::none()
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
