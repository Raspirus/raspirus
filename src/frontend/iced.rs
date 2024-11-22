use crate::backend::config_file::Config;
use crate::backend::downloader;
use crate::backend::utils::generic::{
    download_logs, generate_virustotal, profile_path, update_config,
};
use crate::backend::utils::pdf_gen::generate_pdf;
use crate::backend::utils::usb_utils::{list_usb_drives, UsbDevice};
use crate::backend::yara_scanner::{Skipped, TaggedFile, YaraScanner};
use futures::SinkExt;
use iced::{
    futures::{channel::mpsc, Stream},
    stream,
};
use log::{debug, error, info, warn};
use rust_i18n::t;
use std::borrow::Cow;
use std::path::PathBuf;

pub struct Raspirus {
    pub state: State,
    pub scan_path: Option<PathBuf>,
    pub usb_devices: Vec<UsbDevice>,
    pub sender: Option<mpsc::Sender<PathBuf>>,
    pub location_selection: LocationSelection,
    pub dark_mode: bool,
    pub scale: usize,
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
    },
    Scanning {
        // current displayed percentage
        scan_state: ScanState,
    },
    Settings {
        config: Box<Config>,
        update: UpdateState,
        temp_scale: usize,
    },
    Results {
        // tagged / skipped files and if the file is expanded in the view
        tagged: Vec<(TaggedFile, bool)>,
        skipped: Vec<(Skipped, bool)>,
        total: usize,
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

#[derive(Debug, Clone)]
pub enum ScanState {
    Percentage(f32),
    Preparing,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LocationSelection {
    Usb { usb: Option<UsbDevice> },
    Folder { path: Option<PathBuf> },
    File { path: Option<PathBuf> },
}

#[derive(Clone)]
pub struct Language {
    pub file_name: String,
    pub display_name: String,
    pub flag: iced::widget::svg::Handle,
}

impl Language {
    pub fn new(
        file_name: impl std::fmt::Display,
        display_name: impl std::fmt::Display,
        bytes: impl Into<Cow<'static, [u8]>>,
    ) -> Self {
        Self {
            file_name: file_name.to_string(),
            display_name: display_name.to_string(),
            flag: iced::widget::svg::Handle::from_memory(bytes),
        }
    }
}

pub enum Worker {
    Ready { sender: mpsc::Sender<PathBuf> },
    Message { message: Message },
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
        skipped: Vec<(Skipped, bool)>,
        tagged: Vec<(TaggedFile, bool)>,
        total: usize,
        log_path: PathBuf,
    },
    Shutdown,
    StartScan,
    ToggleLanguageSelection,
    ToggleUSBSelection,
    ToggleLocationSelection,
    GenerateVirustotal {
        path: PathBuf,
    },
    ApplyScale {
        scale: usize,
    },
    UpdateRules,
    // update messages
    Open {
        path: PathBuf,
    },
    DownloadLogs,
    ScannerReady {
        sender: mpsc::Sender<PathBuf>,
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
        total: usize,
        log: PathBuf,
    },
    ToggleCard {
        card: Card,
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
    Language(String),
    Dark(bool),
    Scale(usize),
}

#[derive(Debug, Clone)]
pub enum Card {
    Skipped { card: Skipped },
    Tagged { card: TaggedFile },
}

impl Raspirus {
    fn new() -> Self {
        let usb = list_usb_drives().unwrap_or_default();
        let config = crate::CONFIG.lock().expect("Failed to lock config").clone();
        Self {
            state: State::MainMenu {
                expanded_language: false,
                expanded_location: false,
                expanded_usb: false,
            },
            scan_path: usb.first().map(|usb| usb.path.clone()),
            usb_devices: usb.clone(),
            sender: None,
            location_selection: LocationSelection::Usb {
                usb: usb.first().cloned(),
            },
            dark_mode: config.dark_mode,
            scale: config.scale,
        }
    }

    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        debug!("{:?}", message);
        match message {
            // opens settings page
            Message::OpenSettings => {
                self.state = State::Settings {
                    config: Box::new(crate::CONFIG.lock().expect("Failed to lock config").clone()),
                    update: UpdateState::Loaded,
                    temp_scale: self.scale,
                };
                iced::Task::none()
            }
            // opens information page
            Message::OpenInformation => {
                self.state = State::Information;
                iced::Task::none()
            }
            // return back to main menu
            Message::OpenMain => {
                let usb = list_usb_drives().unwrap_or_default().first().cloned();
                self.state = State::MainMenu {
                    expanded_language: false,
                    expanded_location: false,
                    expanded_usb: false,
                };
                if let Some(usb) = usb {
                    self.scan_path = Some(usb.path);
                }
                iced::Task::none()
            }
            // expand language dropdown
            Message::ToggleLanguageSelection => {
                // invert expanded state
                if let State::MainMenu {
                    expanded_language,
                    expanded_location,
                    expanded_usb,
                } = &self.state
                {
                    self.state = State::MainMenu {
                        expanded_language: !expanded_language,
                        expanded_location: *expanded_location,
                        expanded_usb: *expanded_usb,
                    }
                }
                iced::Task::none()
            }
            // show popup for warnings and quit for critical errors
            Message::Error { case } => match case {
                ErrorCase::Critical { message } => iced::Task::done({
                    error!("{message}");
                    rfd::MessageDialog::new()
                        .set_description(&message)
                        .set_title(t!("error_title"))
                        .set_level(rfd::MessageLevel::Error)
                        .show();
                    Message::Shutdown
                }),
                ErrorCase::Warning { message } => {
                    if let State::Scanning {
                        scan_state: ScanState::Preparing,
                    } = self.state
                    {
                        self.state = State::MainMenu {
                            expanded_language: false,
                            expanded_location: false,
                            expanded_usb: false,
                        }
                    }

                    iced::Task::done({
                        warn!("{message}");
                        rfd::MessageDialog::new()
                            .set_description(&message)
                            .set_title(t!("notice_title"))
                            .set_level(rfd::MessageLevel::Warning)
                            .show();
                        Message::None
                    })
                }
            },
            // switch to result page
            Message::ScanComplete {
                tagged,
                skipped,
                total,
                log,
            } => {
                self.state = State::Results {
                    tagged,
                    skipped,
                    total,
                    log,
                };
                iced::Task::none()
            }
            // update local scan percentage
            Message::ScanPercentage { percentage } => {
                self.state = State::Scanning {
                    scan_state: ScanState::Percentage(percentage),
                };
                iced::Task::none()
            }
            // toggle expansion of card in results screen
            Message::ToggleCard { card } => {
                if let State::Results {
                    tagged,
                    skipped,
                    total,
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
                            total: *total,
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
                            total: *total,
                            log: log.clone(),
                        },
                    }
                }
                iced::Task::none()
            }
            // shutdown application
            Message::Shutdown => std::process::exit(0),
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
                            };
                            self.location_selection = LocationSelection::Usb { usb: None };
                        }
                        // if does not contain usb device we do nothing
                        iced::Task::none()
                    }
                    LocationSelection::Folder { path } => {
                        // if contains path to scan and display it
                        if path.is_none() {
                            self.state = State::MainMenu {
                                expanded_language: false,
                                expanded_location: false,
                                expanded_usb: false,
                            };
                            self.location_selection = LocationSelection::Folder { path: None };
                            iced::Task::none()
                        // if does not contain path we open file dialog to pick one
                        } else {
                            iced::Task::none()
                        }
                    }
                    LocationSelection::File { path } => {
                        // if contains path to scan and display it
                        if path.is_none() {
                            self.state = State::MainMenu {
                                expanded_language: false,
                                expanded_location: false,
                                expanded_usb: false,
                            };
                            self.location_selection = LocationSelection::File { path: None };
                            iced::Task::none()
                        // if does not contain path we open file dialog to pick one
                        } else {
                            iced::Task::none()
                        }
                    }
                },
                _ => iced::Task::none(),
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
                            };
                            self.location_selection = LocationSelection::Usb { usb: Some(usb) }
                        // if does not contain usb device we just update to show
                        } else {
                            self.state = State::MainMenu {
                                expanded_language: false,
                                expanded_location: false,
                                expanded_usb: false,
                            };
                            self.location_selection = LocationSelection::Usb { usb }
                        }
                        iced::Task::none()
                    }
                    LocationSelection::Folder { path } => {
                        // if contains path to scan and display it
                        if let Some(path) = path {
                            self.scan_path = Some(path.clone());
                            self.state = State::MainMenu {
                                expanded_language: false,
                                expanded_location: false,
                                expanded_usb: false,
                            };
                            self.location_selection =
                                LocationSelection::Folder { path: Some(path) };
                            iced::Task::none()
                        // if does not contain path we open file dialog to pick one
                        } else {
                            iced::Task::done({
                                match rfd::FileDialog::new()
                                    .set_title(t!("pick_folder"))
                                    .pick_folder()
                                {
                                    None => Message::None,
                                    Some(result) => Message::RequestLocation {
                                        selection: LocationSelection::Folder { path: Some(result) },
                                    },
                                }
                            })
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
                            };
                            self.location_selection =
                                LocationSelection::Folder { path: Some(path) };
                            iced::Task::none()
                        // if does not contain path we open file dialog to pick one
                        } else {
                            iced::Task::done({
                                match rfd::FileDialog::new()
                                    .set_title(t!("pick_file"))
                                    .pick_file()
                                {
                                    None => Message::None,
                                    Some(result) => Message::RequestLocation {
                                        selection: LocationSelection::Folder { path: Some(result) },
                                    },
                                }
                            })
                        }
                    }
                },
                _ => iced::Task::none(),
            },
            // expand list with usb drives
            Message::ToggleUSBSelection => {
                if let State::MainMenu {
                    expanded_language,
                    expanded_location,
                    expanded_usb,
                } = &self.state
                {
                    match list_usb_drives() {
                        Ok(usbs) => {
                            self.usb_devices = usbs;
                            if let LocationSelection::Usb { .. } = &self.location_selection {
                                self.state = State::MainMenu {
                                    expanded_language: *expanded_language,
                                    expanded_location: *expanded_location,
                                    expanded_usb: !*expanded_usb,
                                };
                            }
                            iced::Task::none()
                        }
                        Err(message) => iced::Task::done(Message::Error {
                            case: ErrorCase::Warning { message },
                        }),
                    }
                } else {
                    iced::Task::none()
                }
            }
            // expand dropdown to choose folder, file or usb
            Message::ToggleLocationSelection => {
                if let State::MainMenu {
                    expanded_language,
                    expanded_location,
                    expanded_usb,
                } = &self.state
                {
                    self.state = State::MainMenu {
                        expanded_language: *expanded_language,
                        expanded_location: !*expanded_location,
                        expanded_usb: *expanded_usb,
                    };
                    self.usb_devices = list_usb_drives().unwrap_or_default();
                }
                iced::Task::none()
            }
            // generate hash for file and open in preferred browser
            Message::GenerateVirustotal { path } => iced::Task::done({
                match generate_virustotal(path) {
                    Ok(virus_total) => match open::that(virus_total) {
                        Ok(_) => Message::None,
                        Err(message) => Message::Error {
                            case: ErrorCase::Warning {
                                message: message.to_string(),
                            },
                        },
                    },
                    Err(message) => Message::Error {
                        case: ErrorCase::Warning { message },
                    },
                }
            }),
            // do nothing
            Message::None => iced::Task::none(),
            // send changed config value to backend
            Message::ConfigChanged { value } => match update_config(value) {
                Ok(config) => {
                    self.dark_mode = config.dark_mode;
                    self.scale = config.scale;
                    rust_i18n::set_locale(&config.language);

                    if let State::MainMenu {
                        expanded_location,
                        expanded_usb,
                        ..
                    } = &self.state
                    {
                        self.state = State::MainMenu {
                            expanded_language: false,
                            expanded_location: *expanded_location,
                            expanded_usb: *expanded_usb,
                        };
                    } else if let State::Settings {
                        update, temp_scale, ..
                    } = &self.state
                    {
                        self.state = State::Settings {
                            config: Box::new(config),
                            update: update.clone(),
                            temp_scale: *temp_scale,
                        };
                    }
                    iced::Task::none()
                }
                Err(message) => iced::Task::done(Message::Error {
                    case: ErrorCase::Critical {
                        message: message.clone(),
                    },
                }),
            },
            Message::ApplyScale { scale } => {
                if let State::Settings { config, update, .. } = &self.state {
                    self.state = State::Settings {
                        config: config.clone(),
                        update: update.clone(),
                        temp_scale: scale,
                    }
                }
                iced::Task::none()
            }
            // start rule update
            Message::UpdateRules => {
                if let State::Settings {
                    config, temp_scale, ..
                } = &self.state
                {
                    self.state = State::Settings {
                        config: config.clone(),
                        update: UpdateState::Updating,
                        temp_scale: *temp_scale,
                    };
                }
                iced::Task::perform(
                    async move {
                        match downloader::update().await {
                            Ok(_) => Message::UpdateFinished,
                            Err(err) => match err {
                                downloader::RemoteError::Offline => Message::Error {
                                    case: ErrorCase::Warning {
                                        message: t!("warn_offline").to_string(),
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
                if let State::Settings { temp_scale, .. } = &self.state {
                    self.state = State::Settings {
                        config: Box::new(
                            crate::CONFIG.lock().expect("Failed to lock config").clone(),
                        ),
                        update: UpdateState::Updated,
                        temp_scale: *temp_scale,
                    };
                }
                iced::Task::none()
            }
            // start pdf generation
            Message::DownloadLog {
                skipped,
                tagged,
                total,
                log_path,
            } => iced::Task::done(match generate_pdf(skipped, tagged, total, log_path) {
                Ok(path) => Message::Open { path },
                Err(message) => Message::Error {
                    case: ErrorCase::Warning { message },
                },
            }),
            // open a path
            Message::Open { path } => iced::Task::perform(
                async {
                    info!("Opening {}...", path.to_string_lossy());
                    match open::that(path) {
                        Ok(_) => Message::None,
                        Err(message) => Message::Error {
                            case: ErrorCase::Warning {
                                message: message.to_string(),
                            },
                        },
                    }
                },
                |result| result,
            ),
            Message::OpenTerms => {
                self.state = State::Terms;
                iced::Task::none()
            }
            Message::ScannerReady { sender } => {
                self.sender = Some(sender);
                iced::Task::none()
            }
            Message::StartScan => {
                self.state = State::Scanning {
                    scan_state: ScanState::Preparing,
                };
                let path = self.scan_path.clone();
                let mut sender = self.sender.clone();
                iced::Task::perform(
                    async move {
                        if let Some(sender) = &mut sender {
                            match path {
                                Some(path) => {
                                    sender
                                        .send(path)
                                        .await
                                        .expect("Failed to send path to stream");
                                    Ok(())
                                }
                                None => Err(ErrorCase::Warning {
                                    message: t!("warn_no_path").to_string(),
                                }),
                            }
                        } else {
                            Err(ErrorCase::Critical {
                                message: "No channel ready".to_owned(),
                            })
                        }
                    },
                    |result| {
                        if let Err(case) = result {
                            Message::Error { case }
                        } else {
                            Message::None
                        }
                    },
                )
            }
            Message::DownloadLogs => iced::Task::done(match download_logs() {
                Ok(path) => Message::Open { path },
                Err(message) => Message::Error {
                    case: ErrorCase::Warning { message },
                },
            }),
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        match &self.state {
            State::MainMenu {
                expanded_language,
                expanded_location,
                expanded_usb,
            } => self.main_menu(
                *expanded_language,
                *expanded_location,
                *expanded_usb,
                self.location_selection.clone(),
                &self.usb_devices,
            ),
            State::Scanning { scan_state, .. } => self.scanning(scan_state.clone()),
            State::Settings {
                config,
                update,
                temp_scale,
            } => self.settings(config, update, *temp_scale),
            State::Results {
                tagged,
                skipped,
                total,
                log,
            } => self.results(tagged.clone(), skipped.clone(), *total, log.clone()),
            State::Information => self.information(),
            State::Terms => self.terms(),
        }
    }

    fn scan() -> impl Stream<Item = Worker> {
        stream::channel(100, |mut output| async move {
            // create channel to receive scan commands
            let (sender, mut receiver) = mpsc::channel(100);
            output
                .send(Worker::Ready { sender })
                .await
                .expect("Failed to send job input to stream");

            loop {
                use iced::futures::StreamExt;

                let input = receiver.select_next_some().await;
                info!("Starting scan on {}", input.to_string_lossy());

                // create scanner
                let scanner = YaraScanner::new();
                let (sender, mut receiver) = mpsc::channel(10);
                if !input.exists() {
                    output
                        .send(Worker::Message {
                            message: Message::Error {
                                case: ErrorCase::Warning {
                                    message: "Path no longer valid!".to_owned(),
                                },
                            },
                        })
                        .await
                        .expect("Failed to send error to frontend");
                    continue;
                }
                let (total_size, paths) = profile_path(input);
                let mut scanned_size = 0;

                let handle = tokio::task::spawn(async move { scanner.start(sender, paths) });

                info!("Starting scan of size {total_size}");

                // calculate and forward progress
                while let Some(value) = receiver.next().await {
                    scanned_size += value;
                    debug!("{scanned_size} / {total_size}");
                    let percentage = (scanned_size as f32 / total_size as f32) * 100.0;
                    output
                        .send(Worker::Message {
                            message: Message::ScanPercentage { percentage },
                        })
                        .await
                        .expect("Failed to send progress to frontend");
                }

                let result = handle.await.expect("Failed to wait for handle");

                let message = match result {
                    Ok(message) => Message::ScanComplete {
                        tagged: message
                            .0
                            .iter()
                            .map(|value| (value.clone(), false))
                            .collect(),
                        skipped: message
                            .1
                            .iter()
                            .map(|value| (value.clone(), false))
                            .collect(),
                        total: message.2,
                        log: message.3,
                    },
                    Err(message) => Message::Error {
                        case: ErrorCase::Warning { message },
                    },
                };
                output
                    .send(Worker::Message { message })
                    .await
                    .expect("Failed to send scan result");
            }
        })
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::run(Self::scan).map(|worker| match worker {
            Worker::Ready { sender } => Message::ScannerReady { sender },
            Worker::Message { message } => message,
        })
    }
}

impl Default for Raspirus {
    fn default() -> Self {
        Self::new()
    }
}

pub fn wrap(padding: u16, element: iced::Element<Message>) -> iced::Element<Message> {
    iced::widget::Container::new(element)
        .padding(padding)
        .into()
}
