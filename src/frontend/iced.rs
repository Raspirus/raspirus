//use iced::futures::channel::mpsc;
use log::{error, info};
use std::sync::mpsc;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use crate::backend::yara_scanner::{Skipped, TaggedFile, YaraScanner};

pub struct Raspirus {
    pub state: State,
    pub language: String,
    pub language_expanded: bool,
    pub path_selected: PathBuf,
    pub scan_progress: (
        Arc<Mutex<mpsc::Sender<Message>>>,
        Arc<Mutex<mpsc::Receiver<Message>>>,
    ),
}

pub enum State {
    MainMenu,
    Scanning(f32),
    Settings,
    Results(Vec<(TaggedFile, bool)>, Vec<(Skipped, bool)>),
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
    ScanComplete((Vec<(TaggedFile, bool)>, Vec<(Skipped, bool)>)),
    ToggleCard(Card),
    // data messages
    ScanPercentage(f32),
    Error(String),
    // none message
    None,
}

#[derive(Debug, Clone)]
pub enum Card {
    Skipped(Skipped),
    Tagged(TaggedFile),
}

impl iced::Application for Raspirus {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        let channel = mpsc::channel();
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
                        let scanner = YaraScanner::new(sender_c)
                            .map_err(|err| format!("Failed to build scanner: {err}"))?;
                        scanner.start(scanner_path).await
                    },
                    |result| match result {
                        Ok((tagged, skipped)) => Message::ScanComplete((
                            tagged.iter().map(|tag| (tag.clone(), false)).collect(),
                            skipped.iter().map(|skip| (skip.clone(), false)).collect(),
                        )),
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
            Message::ToggleCard(card) => {
                match &self.state {
                    State::Results(tagged, skipped) => {
                        self.state = match card {
                            Card::Skipped(skipped_card) => State::Results(
                                tagged.to_vec(),
                                skipped
                                    .iter()
                                    .map(|(skip, expanded)| {
                                        if *skip == skipped_card {
                                            (skip.clone(), !*expanded)
                                        } else {
                                            (skip.clone(), *expanded)
                                        }
                                    })
                                    .collect(),
                            ),
                            Card::Tagged(tagged_card) => State::Results(
                                tagged
                                    .iter()
                                    .map(|(tag, expanded)| {
                                        if *tag == tagged_card {
                                            (tag.clone(), !*expanded)
                                        } else {
                                            (tag.clone(), *expanded)
                                        }
                                    })
                                    .collect(),
                                skipped.to_vec(),
                            ),
                        }
                    }
                    _ => {}
                }
                iced::Command::none()
            }
            Message::None => iced::Command::none(),
        }
    }

    fn view(&self) -> iced::Element<Message> {
        let content = match &self.state {
            State::MainMenu => self.main_menu(),
            State::Scanning(percentage) => self.scanning(),
            State::Settings => self.settings(),
            State::Results(tagged, skipped) => self.results()
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
                "scan_update",
                self.scan_progress.1.clone(),
                |receiver| async {
                    // get receiver
                    let receiver_c = receiver.clone();
                    let receiver_l = match receiver_c.lock() {
                        Ok(receiver_l) => receiver_l,
                        Err(err) => return (Message::Error(err.to_string()), receiver),
                    };

                    loop {
                        match receiver_l.recv() {
                            Ok(message) => return (message, receiver),
                            Err(_) => {
                                sleep(Duration::from_millis(100));
                                continue;
                            }
                        }
                    }
                },
            ),
            _ => iced::Subscription::none(),
        }
    }
}
