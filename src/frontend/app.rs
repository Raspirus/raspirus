use relm4::{
    actions::{RelmAction, RelmActionGroup},
    gtk, main_application, Component, ComponentParts, ComponentSender, SimpleComponent,
};

use gtk::prelude::{
    ApplicationExt, ApplicationWindowExt, GtkWindowExt, OrientableExt, SettingsExt, WidgetExt,
};
use gtk::{gio, glib};
use crate::globals::APP_ID;

pub struct AppModel {}

#[derive(Debug)]
pub enum AppMsg {
    Quit,
}

relm4::new_action_group!(pub(super) WindowActionGroup, "win");
relm4::new_stateless_action!(PreferencesAction, WindowActionGroup, "preferences");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");



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

    menu! {
        primary_menu: {
            section! {
                "_Preferences" => PreferencesAction,
                "_About" => AboutAction,
            }
        }
    }

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

                gtk::HeaderBar {
                    pack_end = &gtk::MenuButton {
                        set_icon_name: "open-menu-symbolic",
                        set_menu_model: Some(&primary_menu),
                    }
                },

                gtk::Label {
                    set_label: "Hello world!",
                    set_vexpand: true,
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
        let mut actions = RelmActionGroup::<WindowActionGroup>::new();


        /*
                let shortcuts_action = {
            let shortcuts = widgets.shortcuts.clone();
            RelmAction::<ShortcutsAction>::new_stateless(move |_| {
                shortcuts.present();
            })
        };

        let about_action = {
            RelmAction::<AboutAction>::new_stateless(move |_| {
                AboutDialog::builder().launch(()).detach();
            })
        };

        actions.add_action(shortcuts_action);
        actions.add_action(about_action);
         */
        

        actions.register_for_widget(&widgets.main_window);
        widgets.load_window_size();

        ComponentParts { model, widgets }

    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            AppMsg::Quit => main_application().quit(),
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