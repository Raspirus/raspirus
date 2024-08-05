use core::f64;
use log::{debug, error, info};
use std::{path::PathBuf, sync::mpsc};

use crate::backend::yara_scanner::{Skipped, TaggedFile, YaraScanner};

pub struct Raspirus {
    pub state: State,
    pub language: String,
    pub language_expanded: bool,
    pub path_selected: PathBuf,
    pub scan_channel: Option<(mpsc::Sender<Message>, mpsc::Receiver<Message>)>,
}

pub enum State {
    MainMenu,
    Scanning(f64),
    Settings,
    Results(Vec<TaggedFile>, Vec<Skipped>),
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
    ScanComplete((Vec<TaggedFile>, Vec<Skipped>)),
    // data messages
    ScanPercentage(f64),
    Error(String),
}

impl iced::Application for Raspirus {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        let app = (
            Self {
                state: State::MainMenu,
                language: "en-US".to_owned(),
                language_expanded: false,
                path_selected: PathBuf::new(),
                scan_channel: None,
            },
            iced::Command::none(),
        );
        info!("Build app");
        app
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
            }
            Message::OpenMain => {
                self.state = State::MainMenu;
                iced::Command::none()
            }
            Message::StartScan => {
                let scanner_path = self.path_selected.clone();
                let sender_c = self.scan_channel.0.clone();
                iced::Command::perform(
                    async {
                        let mut scanner = YaraScanner::new(Some(sender_c.into())).unwrap();
                        scanner.start(scanner_path).unwrap();
                    },
                    |_| Message::OpenMain,
                )
            }
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
            Message::Error(err) => {
                error!("{err}");
                iced::Command::none()
            }
            Message::ScanComplete((tagged, skipped)) => {
                self.state = State::Results(tagged, skipped);
                iced::Command::none()
            }
            Message::ScanPercentage(percentage) => {
                self.state = State::Scanning(percentage);
                iced::Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<Message> {
        let content = match &self.state {
            State::MainMenu => self.main_menu(),
            State::Scanning(percentage) => todo!(),
            State::Settings => self.settings(),
            State::Results(tagged, skipped) => todo!(),
        };
        iced::Element::new(
            iced::widget::Container::new(content)
                .padding(10)
                .center_x()
                .width(iced::Length::Fill),
        )
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::subscription::unfold(
            "Scan_Update",
            self.scan_channel.as_ref().unwrap().1.recv(),
            |message| async {
                (
                    message
                        .clone()
                        .unwrap_or(Message::Error("Failed to receive message".to_owned())),
                    message,
                )
            },
        )
    }
}
