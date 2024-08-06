use futures::StreamExt;
use iced::futures::channel::mpsc;
use log::{debug, error, info};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::backend::yara_scanner::{Skipped, TaggedFile, YaraScanner};

pub struct Raspirus {
    pub state: State,
    pub language: String,
    pub language_expanded: bool,
    pub path_selected: PathBuf,
    pub scan_progress: (
        Arc<Mutex<mpsc::UnboundedSender<Message>>>,
        Arc<Mutex<mpsc::UnboundedReceiver<Message>>>,
    ),
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
    // none message
    None,
}

impl iced::Application for Raspirus {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        let channel = mpsc::unbounded();
        info!("Channel built");
        let app = (
            Self {
                state: State::MainMenu,
                language: "en-US".to_owned(),
                language_expanded: false,
                path_selected: PathBuf::new(),
                scan_progress: (
                    Arc::new(Mutex::new(channel.0)),
                    Arc::new(Mutex::new(channel.1)),
                ),
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
                self.state = State::Scanning(0.0);
                let scanner_path = self.path_selected.clone();
                let sender_c = self.scan_progress.0.clone();

                iced::Command::perform(
                    async move {
                        let scanner = YaraScanner::new(sender_c).expect("Failed to build scanner");
                        scanner.start(scanner_path).await
                    },
                    |result| match result {
                        Ok((tagged, skipped)) => Message::ScanComplete((tagged, skipped)),
                        Err(err) => Message::Error(err),
                    },
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
            Message::None => iced::Command::none(),
        }
    }

    fn view(&self) -> iced::Element<Message> {
        let content = match &self.state {
            State::MainMenu => self.main_menu(),
            State::Scanning(percentage) => {
                iced::widget::Text::new(format!("{percentage:.2}%")).into()
            }
            State::Settings => self.settings(),
            State::Results(tagged, skipped) => {
                println!("{:?}, {:?}", tagged, skipped);
                iced::widget::Text::new("done").into()
            }
        };
        iced::Element::new(
            iced::widget::Container::new(content)
                .padding(10)
                .center_x()
                .width(iced::Length::Fill),
        )
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        match self.state {
            State::Scanning(_) => iced::subscription::unfold(
                "Scan_Update",
                self.scan_progress.clone(),
                move |channel| async move {
                    let channel_c = channel.clone();
                    let mut receiver = match channel_c.1.lock() {
                        Ok(receiver) => receiver,
                        Err(err) => {
                            error!("Failed to lock receiver: {err}");
                            return (Message::Error(format!("Failed to lock receiver: {err}")), channel)
                        },
                    };
                    match receiver.try_next() {
                        Ok(message) => {
                            if let Some(message) = message {
                                (message, channel)
                            } else {
                                (Message::Error("Failed to get message".to_owned()), channel)
                            }
                        }
                        Err(_) => (Message::None, channel),
                    }
                },
            ),
            _ => iced::Subscription::none(),
        }
    }
}
