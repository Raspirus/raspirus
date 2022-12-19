use iced::{Element, Sandbox, widget::{Column, Text, Row, Button, Container}, Length};
/* Is the main page of the program and gets immediately displayed when the program starts.
It has the following components:
- A button to start the scan
- A button that opens a new window that displays some information about the project
- A button that opens a page to set some settings
- A dropdown menu that automatically displays all connected USB drives with their path and name

When the start button is clicked, the path of the selected USB drive is given to the filescanner
and the scan is started. While the scan is going, a small loading animation is shown (On a new page)

The application has a fixe size of 800x480, which is equal to the size of the Raspberry touchscreen

Figma: https://www.figma.com/file/pkgpwieNbhYiOi4Gz6Uyt6/Raspirus?node-id=0%3A1&t=LXB7UphT9lWoIjev-0
Maybe use: https://github.com/tauri-apps/wry
*/

#[derive(Default)]
pub struct MainWindow {
    title_text: String,
    drive_list: Vec<String>,
    selected_drive: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    StartScanner,
    ShowInfo,
    ShowSettings,
}

impl Sandbox for MainWindow {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }
    
    fn title(&self) -> String {
        String::from("Raspirus")
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn should_exit(&self) -> bool {
        false
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::StartScanner => {
                // TODO: Implement start_scanner function
            }
            Message::ShowInfo => {
                // TODO: Show info page
            }
            Message::ShowSettings => {
                // TODO: Show settings page
            }
        }
    }

    fn view(self: &MainWindow) -> Element<Self::Message> {
        let main_page = Column::new()
            .spacing(20)
            .padding(20)
            .push(Text::new("Title:").size(20))
            .push(Text::new(&self.title_text).size(20))
            .push(
                Row::new()
                    .spacing(20)
                    .push(Button::new("Start").on_press(Message::StartScanner))
                    .push(Button::new("Info").on_press(Message::ShowInfo))
                    .push(
                        Button::new("Settings")
                            .on_press(Message::ShowSettings),
                    ),
            );

        Container::new(main_page)
            .max_height(480)
            .max_width(800)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}