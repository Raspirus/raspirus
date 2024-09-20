use crate::backend::config_file::Config;
use crate::backend::downloader;
use crate::backend::utils::generic::{create_pdf, generate_virustotal, update_config};
use crate::backend::utils::usb_utils::{list_usb_drives, UsbDevice};
use crate::backend::yara_scanner::{Skipped, TaggedFile, YaraScanner};
use log::{debug, error, info, trace, warn};
use std::fmt::Display;
use std::str::FromStr;
use std::sync::mpsc::{self, Receiver, Sender};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use super::theme::RASPIRUS_PALETTE;

type ProgressSender = Arc<Mutex<Sender<Message>>>;
type ProgressReceiver = Arc<Mutex<Receiver<Message>>>;

pub struct Raspirus {
    pub state: State,
    pub language: String,
    pub scan_path: Option<PathBuf>,
    pub usb_devices: Vec<UsbDevice>,
    pub scan_progress: (ProgressSender, ProgressReceiver),
}

#[derive(Debug)]
pub enum State {
    MainMenu {
        /// language dropdown state
        expanded_language: bool,
        /// dropdown to switch between usb folder and file scan
        expanded_location: bool,
        /// dropdown for selecting usb
        expanded_usb: bool,
        selection: LocationSelection,
    },
    Scanning {
        // current displayed percentage
        percentage: f32,
    },
    Settings {
        config: Config,
        update: UpdateState,
    },
    Results {
        // tagged / skipped files and if the file is expanded in the view
        tagged: Vec<(TaggedFile, bool)>,
        skipped: Vec<(Skipped, bool)>,
        log: PathBuf,
    },
    Information,
    Terms,
}

#[derive(Debug, Clone)]
pub enum UpdateState {
    Loaded,
    Updating,
    Updated,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LocationSelection {
    Usb { usb: Option<UsbDevice> },
    Folder { path: Option<PathBuf> },
    File { path: Option<PathBuf> },
}

impl FromStr for LocationSelection {
    type Err = ();

    fn from_str(selection: &str) -> Result<Self, Self::Err> {
        match selection.trim() {
            _ if selection == iced_aw::Bootstrap::UsbDriveFill.to_string() => {
                Ok(LocationSelection::Usb { usb: None })
            }
            _ if selection == iced_aw::Bootstrap::FolderFill.to_string() => {
                Ok(LocationSelection::Folder { path: None })
            }
            _ if selection == iced_aw::Bootstrap::FileEarmarkFill.to_string() => {
                Ok(LocationSelection::File { path: None })
            }
            _ => Err(()),
        }
    }
}

impl Display for LocationSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LocationSelection::Usb { .. } => format!(" {}", iced_aw::Bootstrap::UsbDriveFill),
                LocationSelection::Folder { .. } => format!(" {}", iced_aw::Bootstrap::FolderFill),
                LocationSelection::File { .. } =>
                    format!(" {}", iced_aw::Bootstrap::FileEarmarkFill),
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    // location messages
    OpenSettings,
    OpenInformation,
    OpenTerms,
    OpenMain,
    // action messages
    DownloadLog {
        log_path: PathBuf,
    },
    StartScan,
    Shutdown,
    ToggleLanguageSelection,
    ToggleUSBSelection,
    ToggleLocationSelection,
    GenerateVirustotal {
        path: PathBuf,
    },
    UpdateRules,
    // update messages
    Downloaded {
        pdf_path: PathBuf,
    },
    LanguageChanged {
        language: String,
    },
    /// contains empty enum if just type changed and enum with content if something has been selected
    LocationChanged {
        selection: LocationSelection,
    },
    ConfigChanged {
        value: ConfigValue,
    },
    /// sent when we want the user to pick a location
    RequestLocation {
        selection: LocationSelection,
    },
    ScanComplete {
        tagged: Vec<(TaggedFile, bool)>,
        skipped: Vec<(Skipped, bool)>,
        log: PathBuf,
    },
    ToggleCard {
        card: Card,
    },
    Event {
        event: iced::Event,
    },
    UpdateFinished,
    // data messages
    ScanPercentage {
        percentage: f32,
    },
    Error {
        case: ErrorCase,
    },
    None,
}

#[derive(Debug, Clone)]
pub enum ErrorCase {
    Critical { message: String },
    Warning { message: String },
}

#[derive(Debug, Clone)]
pub enum ConfigValue {
    MinMatch(usize),
    MaxMatch(usize),
    Logging(bool),
    MaxThreads(usize),
}

#[derive(Debug, Clone)]
pub enum Card {
    Skipped { card: Skipped },
    Tagged { card: TaggedFile },
}

impl iced::Application for Raspirus {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        let channel = mpsc::channel();
        let usb = list_usb_drives().unwrap_or_default().first().cloned();
        (
            Self {
                state: State::MainMenu {
                    expanded_language: false,
                    expanded_location: false,
                    expanded_usb: false,
                    selection: LocationSelection::Usb { usb: usb.clone() },
                },
                language: "en-US".to_owned(),
                scan_path: if let Some(usb) = usb {
                    Some(usb.path)
                } else {
                    None
                },
                scan_progress: (
                    Arc::new(Mutex::new(channel.0)),
                    Arc::new(Mutex::new(channel.1)),
                ),
                usb_devices: list_usb_drives().unwrap_or_default(),
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        "Raspirus".to_owned()
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match &message {
            Message::Event { .. } => {}
            others => debug!("{:?}", others),
        }
        match message {
            // opens settings page
            Message::OpenSettings => {
                self.state = State::Settings {
                    config: crate::CONFIG.lock().expect("Failed to lock config").clone(),
                    update: UpdateState::Loaded,
                };
                iced::Command::none()
            }
            // opens information page
            Message::OpenInformation => {
                self.state = State::Information;
                iced::Command::none()
            }
            // return back to main menu
            Message::OpenMain => {
                let usb = list_usb_drives().unwrap_or_default().first().cloned();
                self.state = State::MainMenu {
                    expanded_language: false,
                    expanded_location: false,
                    expanded_usb: false,
                    selection: LocationSelection::Usb { usb: usb.clone() },
                };
                if let Some(usb) = usb {
                    self.scan_path = Some(usb.path);
                }
                iced::Command::none()
            }
            // start scan with current path
            Message::StartScan => {
                self.state = State::Scanning { percentage: 0.0 };
                let scanner_path = self.scan_path.clone();
                let sender_c = self.scan_progress.0.clone();

                iced::Command::perform(
                    async move {
                        let scanner = YaraScanner::new(sender_c)
                            .map_err(|err| ErrorCase::Critical {
                                message: format!("Failed to build scanner: {err}"),
                            })?
                            .set_path(scanner_path.ok_or_else(|| ErrorCase::Warning {
                                message: "Select a path first!".to_owned(),
                            })?)
                            .map_err(|err| ErrorCase::Critical { message: err })?;
                        scanner
                            .start()
                            .await
                            .map_err(|err| ErrorCase::Critical { message: err })
                    },
                    |result| match result {
                        Ok((tagged, skipped, log)) => Message::ScanComplete {
                            tagged: tagged.iter().map(|tag| (tag.clone(), false)).collect(),
                            skipped: skipped.iter().map(|skip| (skip.clone(), false)).collect(),
                            log,
                        },
                        Err(err) => Message::Error { case: err },
                    },
                )
            }
            // expand language dropdown
            Message::ToggleLanguageSelection => {
                // invert expanded state
                if let State::MainMenu {
                    expanded_language,
                    expanded_location,
                    expanded_usb,
                    selection,
                } = &self.state
                {
                    self.state = State::MainMenu {
                        expanded_language: !expanded_language,
                        expanded_location: *expanded_location,
                        expanded_usb: *expanded_usb,
                        selection: selection.clone(),
                    }
                }
                iced::Command::none()
            }
            // update locally selected language
            Message::LanguageChanged { language } => {
                // close language dialog
                if let State::MainMenu {
                    expanded_location,
                    expanded_usb,
                    selection,
                    ..
                } = &self.state
                {
                    self.state = State::MainMenu {
                        expanded_language: false,
                        expanded_location: *expanded_location,
                        expanded_usb: *expanded_usb,
                        selection: selection.clone(),
                    }
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
                ErrorCase::Warning { message } => iced::Command::perform(
                    async move {
                        warn!("{message}");
                        native_dialog::MessageDialog::new()
                            .set_text(&message)
                            .set_title("Notice")
                            .set_type(native_dialog::MessageType::Warning)
                            .show_alert()
                    },
                    |_| Message::None,
                ),
            },
            // switch to result page
            Message::ScanComplete {
                tagged,
                skipped,
                log,
            } => {
                self.state = State::Results {
                    tagged,
                    skipped,
                    log,
                };
                iced::Command::none()
            }
            // update local scan percentage
            Message::ScanPercentage { percentage } => {
                self.state = State::Scanning { percentage };
                iced::Command::none()
            }
            // toggle expansion of card in results screen
            Message::ToggleCard { card } => {
                if let State::Results {
                    tagged,
                    skipped,
                    log,
                } = &self.state
                {
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
                            log: log.clone(),
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
                            log: log.clone(),
                        },
                    }
                }
                iced::Command::none()
            }
            // shutdown application
            Message::Shutdown => std::process::exit(0),
            // work with window events
            Message::Event { event } => {
                match event {
                    iced::Event::Window(_, iced::window::Event::CloseRequested) => {
                        return iced::Command::perform(
                            async {
                                info!("Shutting down...");
                            },
                            |_| Message::Shutdown,
                        )
                    }
                    _ => trace!("Ignoring {event:?}"),
                }
                iced::Command::none()
            }
            // update local scan path to selected media
            Message::LocationChanged { selection } => match &self.state {
                State::MainMenu { .. } => match selection {
                    LocationSelection::Usb { usb } => {
                        // if contains usb device we update to scan and display it
                        if usb.is_none() {
                            self.state = State::MainMenu {
                                expanded_language: false,
                                expanded_location: false,
                                expanded_usb: false,
                                selection: LocationSelection::Usb { usb: None },
                            }
                        }
                        // if does not contain usb device we do nothing
                        iced::Command::none()
                    }
                    LocationSelection::Folder { path } => {
                        // if contains path to scan and display it
                        if path.is_none() {
                            self.state = State::MainMenu {
                                expanded_language: false,
                                expanded_location: false,
                                expanded_usb: false,
                                selection: LocationSelection::Folder { path: None },
                            };
                            iced::Command::none()
                        // if does not contain path we open file dialog to pick one
                        } else {
                            iced::Command::none()
                        }
                    }
                    LocationSelection::File { path } => {
                        // if contains path to scan and display it
                        if path.is_none() {
                            self.state = State::MainMenu {
                                expanded_language: false,
                                expanded_location: false,
                                expanded_usb: false,
                                selection: LocationSelection::File { path: None },
                            };
                            iced::Command::none()
                        // if does not contain path we open file dialog to pick one
                        } else {
                            iced::Command::none()
                        }
                    }
                },
                _ => iced::Command::none(),
            },
            // either change to allow for selection of usb, file or folder
            // or update current path to selection
            Message::RequestLocation { selection } => match &self.state {
                State::MainMenu { .. } => match selection {
                    LocationSelection::Usb { usb } => {
                        // if contains usb device we update to scan and display it
                        if let Some(usb) = usb {
                            self.scan_path = Some(usb.path.clone());
                            self.state = State::MainMenu {
                                expanded_language: false,
                                expanded_location: false,
                                expanded_usb: false,
                                selection: LocationSelection::Usb { usb: Some(usb) },
                            }
                        // if does not contain usb device we just update to show
                        } else {
                            self.state = State::MainMenu {
                                expanded_language: false,
                                expanded_location: false,
                                expanded_usb: false,
                                selection: LocationSelection::Usb { usb },
                            }
                        }
                        iced::Command::none()
                    }
                    LocationSelection::Folder { path } => {
                        // if contains path to scan and display it
                        if let Some(path) = path {
                            self.scan_path = Some(path.clone());
                            self.state = State::MainMenu {
                                expanded_language: false,
                                expanded_location: false,
                                expanded_usb: false,
                                selection: LocationSelection::Folder { path: Some(path) },
                            };
                            iced::Command::none()
                        // if does not contain path we open file dialog to pick one
                        } else {
                            iced::Command::perform(
                                async {
                                    native_dialog::FileDialog::new()
                                        .set_location("~")
                                        .set_title("Pick a folder")
                                        .show_open_single_dir()
                                        .expect("Failed to select folder")
                                },
                                |result| match result {
                                    None => Message::None,
                                    Some(result) => Message::RequestLocation {
                                        selection: LocationSelection::Folder { path: Some(result) },
                                    },
                                },
                            )
                        }
                    }
                    LocationSelection::File { path } => {
                        // if contains path to scan and display it
                        if let Some(path) = path {
                            self.scan_path = Some(path.clone());
                            self.state = State::MainMenu {
                                expanded_language: false,
                                expanded_location: false,
                                expanded_usb: false,
                                selection: LocationSelection::Folder { path: Some(path) },
                            };
                            iced::Command::none()
                        // if does not contain path we open file dialog to pick one
                        } else {
                            iced::Command::perform(
                                async {
                                    native_dialog::FileDialog::new()
                                        .set_location("~")
                                        .set_title("Pick a file")
                                        .show_open_single_file()
                                        .expect("Failed to select file")
                                },
                                |result| match result {
                                    None => Message::None,
                                    Some(result) => Message::RequestLocation {
                                        selection: LocationSelection::Folder { path: Some(result) },
                                    },
                                },
                            )
                        }
                    }
                },
                _ => iced::Command::none(),
            },
            // expand list with usb drives
            Message::ToggleUSBSelection => {
                if let State::MainMenu {
                    expanded_language,
                    expanded_location,
                    expanded_usb,
                    selection,
                } = &self.state
                {
                    if let LocationSelection::Usb { usb } = &selection {
                        if usb.is_some() {
                            self.state = State::MainMenu {
                                expanded_language: *expanded_language,
                                expanded_location: *expanded_location,
                                expanded_usb: !*expanded_usb,
                                selection: selection.clone(),
                            }
                        } else {
                            let usbs = list_usb_drives().inspect(|usbs| {
                                self.usb_devices.clone_from(usbs);
                            });
                            let usb = usbs.unwrap_or_default().first().cloned();
                            self.state = State::MainMenu {
                                expanded_language: *expanded_language,
                                expanded_location: *expanded_location,
                                expanded_usb: !*expanded_usb,
                                selection: LocationSelection::Usb { usb: usb.clone() },
                            };
                            if let Some(usb) = usb {
                                self.scan_path = Some(usb.path);
                            }
                        }
                    }
                }
                iced::Command::none()
            }
            // expand dropdown to choose folder, file or usb
            Message::ToggleLocationSelection => {
                if let State::MainMenu {
                    expanded_language,
                    expanded_location,
                    expanded_usb,
                    selection,
                } = &self.state
                {
                    self.state = State::MainMenu {
                        expanded_language: *expanded_language,
                        expanded_location: !*expanded_location,
                        expanded_usb: *expanded_usb,
                        selection: selection.clone(),
                    }
                }
                iced::Command::none()
            }
            // generate hash for file and open in preferred browser
            Message::GenerateVirustotal { path } => iced::Command::perform(
                async {
                    open::that(
                        generate_virustotal(path)
                            .map_err(|message| ErrorCase::Warning { message })?,
                    )
                    .map_err(|message| ErrorCase::Warning {
                        message: message.to_string(),
                    })
                },
                |result: Result<(), ErrorCase>| match result {
                    Ok(_) => Message::None,
                    Err(err) => Message::Error { case: err },
                },
            ),
            // do nothing
            Message::None => iced::Command::none(),
            // send changed config value to backend
            Message::ConfigChanged { value } => match update_config(value) {
                Ok(_) => {
                    self.state = State::Settings {
                        config: crate::CONFIG.lock().expect("Failed to lock config").clone(),
                        update: UpdateState::Loaded,
                    };
                    iced::Command::none()
                }
                Err(message) => iced::Command::perform(async {}, |_| Message::Error {
                    case: ErrorCase::Critical { message },
                }),
            },
            // start rule update
            Message::UpdateRules => {
                if let State::Settings { config, .. } = &self.state {
                    self.state = State::Settings {
                        config: config.clone(),
                        update: UpdateState::Updating,
                    };
                }
                iced::Command::perform(
                    async move {
                        match downloader::update().await {
                            Ok(_) => Message::UpdateFinished,
                            Err(err) => match err {
                                downloader::RemoteError::Offline => Message::Error {
                                    case: ErrorCase::Warning {
                                        message: "You appear to be offline".to_owned(),
                                    },
                                },
                                downloader::RemoteError::Other(message) => Message::Error {
                                    case: ErrorCase::Warning { message },
                                },
                            },
                        }
                    },
                    |result| result,
                )
            }
            // update is finished
            Message::UpdateFinished => {
                if let State::Settings { .. } = &self.state {
                    self.state = State::Settings {
                        config: crate::CONFIG.lock().expect("Failed to lock config").clone(),
                        update: UpdateState::Updated,
                    };
                }
                iced::Command::none()
            }
            // start pdf generation
            Message::DownloadLog { log_path } => iced::Command::perform(
                async move {
                    match create_pdf(log_path) {
                        Ok(pdf_path) => Message::Downloaded { pdf_path },
                        Err(message) => Message::Error {
                            case: ErrorCase::Warning { message },
                        },
                    }
                },
                |result| result,
            ),
            // open pdf log
            Message::Downloaded { pdf_path } => iced::Command::perform(
                async {
                    open::that(pdf_path).map_err(|message| ErrorCase::Warning {
                        message: message.to_string(),
                    })
                },
                |result: Result<(), ErrorCase>| match result {
                    Ok(_) => Message::None,
                    Err(err) => Message::Error { case: err },
                },
            ),
            Message::OpenTerms => {
                self.state = State::Terms;
                iced::Command::none()
            }
        }
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::custom("Raspirus".to_owned(), RASPIRUS_PALETTE)
    }

    fn view(&self) -> iced::Element<Message> {
        match &self.state {
            State::MainMenu {
                expanded_language,
                expanded_location,
                expanded_usb,
                selection,
            } => self.main_menu(
                *expanded_language,
                *expanded_location,
                *expanded_usb,
                selection.clone(),
                &self.usb_devices,
            ),
            State::Scanning { percentage } => self.scanning(*percentage),
            State::Settings { config, update } => self.settings(config, update),
            State::Results {
                tagged,
                skipped,
                log,
            } => self.results(tagged.clone(), skipped.clone(), log.clone()),
            State::Information => self.information(),
            State::Terms => self.terms(),
        }
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        // subsribe to the scan progress update or event stream. this also doubles as quit
        // prevention during scanning
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
            // relay window events as messages
            _ => iced::event::listen().map(|event| Message::Event { event }),
        }
    }
}

pub fn wrap(padding: u16, element: iced::Element<Message>) -> iced::Element<Message> {
    iced::widget::Container::new(element)
        .padding(padding)
        .into()
}
