use log::{error, info, trace};
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
    pub scan_path: Option<PathBuf>,
    pub pop_up: Option<String>,
    pub scan_progress: (
        Arc<Mutex<mpsc::Sender<Message>>>,
        Arc<Mutex<mpsc::Receiver<Message>>>,
    ),
}

pub enum State {
    MainMenu {
        // dropdown state
        language_expanded: bool,
    },
    Scanning {
        // current displayed percentage
        percentage: f32,
    },
    Settings,
    Results {
        // tagged / skipped files and if the file is expanded in the view
        tagged: Vec<(TaggedFile, bool)>,
        skipped: Vec<(Skipped, bool)>,
    },
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
    Shutdown,
    // update messages
    PathChanged {
        path: PathBuf,
    },
    LanguageChanged {
        language: String,
    },
    ScanComplete {
        tagged: Vec<(TaggedFile, bool)>,
        skipped: Vec<(Skipped, bool)>,
    },
    ToggleCard {
        card: Card,
    },
    Event {
        event: iced::Event,
    },
    // data messages
    ScanPercentage {
        percentage: f32,
    },
    Error {
        case: ErrorCase,
    },
}

#[derive(Debug, Clone)]
pub enum ErrorCase {
    Critical { message: String },
    Warning { message: String },
}

#[derive(Debug, Clone)]
pub enum Card {
    Skipped { card: Skipped },
    Tagged { card: TaggedFile },
}

impl iced::Application for Raspirus {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        let channel = mpsc::channel();
        let app = (
            Self {
                state: State::MainMenu {
                    language_expanded: false,
                },
                language: "en-US".to_owned(),
                scan_path: None,
                pop_up: None,
                scan_progress: (
                    Arc::new(Mutex::new(channel.0)),
                    Arc::new(Mutex::new(channel.1)),
                ),
            },
            iced::Command::none(),
        );
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
                self.state = State::MainMenu {
                    language_expanded: false,
                };
                iced::Command::none()
            }
            Message::StartScan => {
                self.state = State::Scanning { percentage: 0.0 };
                let scanner_path = self.scan_path.clone();
                let sender_c = self.scan_progress.0.clone();

                iced::Command::perform(
                    async move {
                        let scanner =
                            YaraScanner::new(sender_c).map_err(|err| ErrorCase::Critical {
                                message: format!("Failed to build scanner: {err}"),
                            })?;
                        scanner
                            .start(scanner_path.ok_or_else(|| ErrorCase::Warning {
                                message: "Select a path first!".to_owned(),
                            })?)
                            .await
                            .map_err(|err| ErrorCase::Critical { message: err })
                    },
                    |result| match result {
                        Ok((tagged, skipped)) => Message::ScanComplete {
                            tagged: tagged.iter().map(|tag| (tag.clone(), false)).collect(),
                            skipped: skipped.iter().map(|skip| (skip.clone(), false)).collect(),
                        },
                        Err(err) => Message::Error { case: err },
                    },
                )
            }
            Message::ToggleLanguage => {
                // invert expanded state
                match self.state {
                    State::MainMenu { language_expanded } => {
                        self.state = State::MainMenu {
                            language_expanded: !language_expanded,
                        }
                    }
                    _ => {}
                };
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
                |result| Message::PathChanged { path: result },
            ),
            Message::PathChanged { path } => {
                self.scan_path = Some(path);
                iced::Command::none()
            }
            Message::LanguageChanged { language } => {
                // close language dialog
                match self.state {
                    State::MainMenu { .. } => {
                        self.state = State::MainMenu {
                            language_expanded: false,
                        }
                    }
                    _ => {}
                }
                self.language = language;
                iced::Command::none()
            }
            // show popup for warnings and quit for critical errors
            Message::Error { case } => match case {
                ErrorCase::Critical { message } => iced::Command::perform(
                    async move {
                        error!("{message}");
                        native_dialog::MessageDialog::new()
                            .set_text(&message)
                            .set_title("Error occurred")
                            .set_type(native_dialog::MessageType::Error)
                            .show_alert()
                    },
                    |_| Message::Shutdown,
                ),
                ErrorCase::Warning { message } => {
                    self.pop_up = Some(message);
                    iced::Command::none()
                }
            },
            Message::ScanComplete { tagged, skipped } => {
                self.state = State::Results { tagged, skipped };
                iced::Command::none()
            }
            Message::ScanPercentage { percentage } => {
                self.state = State::Scanning { percentage };
                iced::Command::none()
            }
            Message::ToggleCard { card } => {
                match &self.state {
                    State::Results { tagged, skipped } => {
                        self.state = match card {
                            Card::Skipped { card } => State::Results {
                                tagged: tagged.to_vec(),
                                skipped: skipped
                                    .iter()
                                    .map(|(skip, expanded)| {
                                        if *skip == card {
                                            (skip.clone(), !*expanded)
                                        } else {
                                            (skip.clone(), *expanded)
                                        }
                                    })
                                    .collect(),
                            },
                            Card::Tagged { card } => State::Results {
                                tagged: tagged
                                    .iter()
                                    .map(|(tag, expanded)| {
                                        if *tag == card {
                                            (tag.clone(), !*expanded)
                                        } else {
                                            (tag.clone(), *expanded)
                                        }
                                    })
                                    .collect(),
                                skipped: skipped.to_vec(),
                            },
                        }
                    }
                    _ => {}
                }
                iced::Command::none()
            }
            Message::Shutdown => std::process::exit(0),
            Message::Event { event } => {
                match event {
                    iced::Event::Window(_, ref request) => match request {
                        iced::window::Event::CloseRequested => {
                            return iced::Command::perform(
                                async {
                                    info!("Shutting down...");
                                },
                                |_| Message::Shutdown,
                            )
                        }
                        _ => trace!("Ignoring {event:?}"),
                    },
                    _ => trace!("Ignoring {event:?}"),
                }
                iced::Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<Message> {
        let content = match &self.state {
            State::MainMenu { language_expanded } => self.main_menu(*language_expanded),
            State::Scanning { percentage } => self.scanning(*percentage),
            State::Settings => self.settings(),
            State::Results { tagged, skipped } => self.results(tagged.clone(), skipped.clone()),
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
            State::Scanning { .. } => iced::subscription::unfold(
                "scan_update",
                self.scan_progress.1.clone(),
                |receiver| async {
                    // get receiver
                    let receiver_c = receiver.clone();
                    let receiver_l = match receiver_c.lock() {
                        Ok(receiver_l) => receiver_l,
                        Err(err) => {
                            return (
                                Message::Error {
                                    case: ErrorCase::Critical {
                                        message: err.to_string(),
                                    },
                                },
                                receiver,
                            )
                        }
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
            _ => iced::event::listen().map(|event| Message::Event { event }),
        }
    }
}
