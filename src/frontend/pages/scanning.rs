use relm4::{gtk, Component, ComponentParts, ComponentSender, SimpleComponent};

use gtk::prelude::GtkWindowExt;

pub struct ScanningPage {}

#[derive(Debug)]
pub enum ScanningPageMsg {}

#[relm4::component(pub)]
impl SimpleComponent for ScanningPage {
    type Input = ScanningPageMsg;
    type Output = ();
    type Init = ();
    type Widgets = ScanningPageWidgets;

    view! {
        gtk::Window {
            gtk::Label {
                set_label: "This is the Scanning Page!"
            }
        }
    }

    fn init(
        starter: Self::Init,
        window: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};
        let widgets = view_output!();

        // HEADER - TITLEBAR - MENU
        let menu = gtk::HeaderBar::builder().build();
        let home_button = gtk::Button::with_label("Home");
        menu.pack_start(&home_button);
        window.set_titlebar(Some(&menu));
        window.set_title(Some("Scanning"));

        ComponentParts { model, widgets }
    }

}

impl ScanningPageWidgets {}