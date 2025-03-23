use relm4::{gtk, main_application, Component, ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};

use gtk::prelude::{
    ApplicationExt, GtkWindowExt, OrientableExt, WidgetExt,
};
use gtk::glib;
use relm4::gtk::prelude::{BoxExt, ButtonExt, PopoverExt};

pub struct AppModel {}

#[derive(Debug)]
pub enum AppMsg {
    Quit,
}



#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    /// The type of the messages that this component can receive.
    type Input = AppMsg;
    /// The type of the messages that this component can send.
    type Output = ();
    /// The type of data with which this component will be initialized.
    type Init = ();
    /// A data structure that contains the widgets that you will need to update.
    type Widgets = AppWidgets;

    view! {
        main_window = gtk::ApplicationWindow::new(&main_application()) {
            set_visible: true,
            set_default_width: 400,
            set_default_height: 500,

            connect_close_request[sender] => move |_| {
                sender.input(AppMsg::Quit);
                glib::Propagation::Stop
            },
            
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_vexpand: true,
                set_margin_all: 5,
                set_spacing: 5,
                
                gtk::Image {
                    set_from_file: Some("src/assets/logo-vector.svg"),
                    set_vexpand: true,
                    set_margin_all: 20,
                },
                
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 5,
                    set_hexpand: true,
                    set_halign: gtk::Align::Center,
                    
                    gtk::DropDown {
                        set_hexpand: true
                        // TODO: Add Items here
                    },
                    
                    gtk::DropDown {
                        set_hexpand: false,
                        // TODO: Add items here
                    },
                },
                
                gtk::Button {
                    set_label: "START",
                    set_halign: gtk::Align::Center,
                },


                gtk::Label {
                    set_label: "Conditions go here",
                    set_halign: gtk::Align::Center,
                    set_hexpand: true,
                }
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
        let menu_btn = gtk::MenuButton::builder().build();
        let menu_box = gtk::Box::builder().spacing(5).build();
        let menu_popover = gtk::PopoverMenu::builder().build();


        let settings_button = gtk::Button::with_label("Settings");
        let info_button = gtk::Button::with_label("Info");
        
        menu_box.append(&settings_button);
        menu_box.append(&info_button);
        menu_box.set_orientation(gtk::Orientation::Vertical);
        menu_popover.set_child(Some(&menu_box));
        menu_btn.set_popover(Some(&menu_popover));
        menu_btn.set_icon_name("open-menu-symbolic");
        menu.pack_start(&menu_btn);
        
        window.set_titlebar(Some(&menu));


        widgets.load_window_size();

        ComponentParts { model, widgets }

    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            AppMsg::Quit => main_application().quit(),
            _ => println!("Command not implemented yet"),
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        widgets.save_window_size().unwrap();
    }
}

impl AppWidgets {
    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        /*
        let settings = gio::Settings::new(APP_ID);
        let (width, height) = self.main_window.default_size();

        settings.set_int("window-width", width)?;
        settings.set_int("window-height", height)?;

        settings.set_boolean("is-maximized", self.main_window.is_maximized())?;
        */
        Ok(())
    }

    fn load_window_size(&self) {
        /*
        TODO: Setup GIO Settings!
        ERROR:  Settings schema 'io.github.raspirus.raspirus' is not installed
        let settings = gio::Settings::new(APP_ID);

        let width = settings.int("window-width");
        let height = settings.int("window-height");
        let is_maximized = settings.boolean("is-maximized");

        self.main_window.set_default_size(width, height);

        if is_maximized {
            self.main_window.maximize();
        }
        
         */
    }
}