use iced::futures::channel::mpsc;
use log::{debug, error, info};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex}, thread::sleep, time::Duration,
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
                info!("{percentage}");
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
        iced::subscription::unfold(
            "scan_update",
            self.scan_progress.1.clone(),
            |receiver| async {
                // get receiver
                let receiver_c = receiver.clone();
                let mut receiver_l = match receiver_c.lock() {
                    Ok(receiver_l) => receiver_l,
                    Err(err) => return (Message::Error(err.to_string()), receiver),
                };

                loop {
                    let message_try = match receiver_l.try_next() {
                        Ok(message_try) => message_try,
                        Err(_) => {
                            sleep(Duration::from_millis(100));
                            continue
                        },
                    };
                    match message_try {
                        Some(message) => return (message, receiver),
                        None => {
                            sleep(Duration::from_millis(100))
                        },
                    }
                }

            },
        )
    }
}
