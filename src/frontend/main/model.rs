pub struct AppModel {
    pub counter: u8,
    pub state: State
}

#[derive(Debug)]
pub enum State {
    MainMenu {},
    Scanning {},
    Settings {},
    Results {},
    Information,
    Terms,
}