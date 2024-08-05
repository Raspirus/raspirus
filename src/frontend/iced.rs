use iced::futures::channel::mpsc;
use log::{debug, error, info};
use std::{path::PathBuf, sync::{Arc, Mutex}};

use crate::backend::yara_scanner::{Skipped, TaggedFile, YaraScanner};

pub struct Raspirus {
    pub state: State,
    pub language: String,
    pub language_expanded: bool,
    pub path_selected: PathBuf,
    pub scan_progress: Arc<Mutex<(mpsc::Sender<Message>, mpsc::Receiver<Message>)>>,
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
        let channel = mpsc::channel(8192);
        info!("Channel built");
        let app = (
            Self {
                state: State::MainMenu,
                language: "en-US".to_owned(),
                language_expanded: false,
                path_selected: PathBuf::new(),
                scan_progress: Arc::new(Mutex::new(channel)),
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
                let sender_c = self.scan_progress.lock().expect("Failed to lock channel").0.clone();
                let mut scanner = YaraScanner::new(sender_c).expect("Failed to build scanner");
                iced::Command::perform(
                    async move {
                        scanner.start(scanner_path)
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
            State::Scanning(_percentage) => todo!(),
            State::Settings => self.settings(),
            State::Results(_tagged, _skipped) => todo!(),
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
            self.scan_progress.clone(),
            move |channel| async move {
                let message = channel.lock().expect("Failed to lock channel").1.try_next();
                match message {
                    Ok(message) => {
                        if let Some(message) = message {
                            (message, channel)
                        } else {
                            (Message::Error("Failed to get message".to_owned()), channel)
                        }
                    },
                    Err(err) => {
                        error!("{err}");
                        (Message::Error(err.to_string()), channel)
                    },
                }
            },
        )
    }
}
