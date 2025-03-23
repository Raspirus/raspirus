use relm4::{gtk, Component, ComponentParts, ComponentSender, SimpleComponent};

use gtk::prelude::GtkWindowExt;

pub struct ResultsPage {}

#[derive(Debug)]
pub enum ResultsPageMsg {}

#[relm4::component(pub)]
impl SimpleComponent for ResultsPage {
    type Input = ResultsPageMsg;
    type Output = ();
    type Init = ();
    type Widgets = ResultsPageWidgets;

    view! {
        gtk::Window {
            gtk::Label {
                set_label: "This is the Results Page!"
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
        window.set_title(Some("Results"));

        ComponentParts { model, widgets }
    }

}

impl ResultsPageWidgets {}