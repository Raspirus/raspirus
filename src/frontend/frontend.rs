use relm4::{
    gtk::{self, glib::clone, prelude::*},
    RelmWidgetExt, SimpleComponent,
};
use relm4::gtk::ArrowType;
use crate::frontend::main::messages::AppMsg;
use crate::frontend::main::model::AppModel;
use crate::frontend::main::model::State::MainMenu;
use crate::frontend::main::widget::AppWidgets;

impl SimpleComponent for AppModel {
    /// The type of the messages that this component can receive.
    type Input = AppMsg;
    /// The type of the messages that this component can send.
    type Output = ();
    /// The type of data with which this component will be initialized.
    type Init = u8;
    /// The root GTK widget that this component will create.
    type Root = gtk::Window;
    /// A data structure that contains the widgets that you will need to update.
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        gtk::Window::builder()
            .title("Raspirus")
            .default_width(400)
            .default_height(500)
            .build()
    }

    fn init(
        counter: Self::Init,
        window: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = AppModel { counter, state: MainMenu {} };
        
        // Header
        let header = gtk::HeaderBar::builder().build();
        
        // Buttons
        let settings_button = gtk::Button::with_label("Settings");
        let info_button = gtk::Button::with_label("Info");
        let start_button = gtk::Button::with_label("Start");
        start_button.set_halign(gtk::Align::Center);
        
        // Labels
        let conditions_label = gtk::Label::new(Option::from("CONDITIONS"));
        conditions_label.set_halign(gtk::Align::Center);
        let title_label = gtk::Label::new(Option::from("Raspirus"));
        
        // Other
        let usb_selector = gtk::DropDown::from_strings(&*vec!["test1", "test2"]);
        usb_selector.set_width_request(200);
        let usb_mode_selector = gtk::DropDown::from_strings(&*vec!["USB", "FILE"]);
        let logo = gtk::Image::from_file("src/assets/logo-vector.svg");
        logo.set_pixel_size(300);
        
        let header_menu = gtk::MenuButton::builder()
            .direction(ArrowType::Down)
            .build();
        
        
        let menu_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .build();
        
        menu_box.append(&settings_button);
        menu_box.append(&info_button);
        
        let menu_model = gtk::PopoverMenu::builder()
            .child(&menu_box)
            .build();
        
        header_menu.set_popover(Some(&menu_model));
        

        header.pack_start(&header_menu);

        let main_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .build();
        
        let input_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(5)
            .halign(gtk::Align::Center)
            .build();
        
        
        input_box.append(&usb_selector);
        input_box.append(&usb_mode_selector);
        

        main_box.append(&logo);
        main_box.append(&input_box);
        main_box.append(&start_button);
        main_box.append(&conditions_label);
        main_box.set_margin_all(5);
        
        window.set_child(Some(&main_box));
        window.set_titlebar(Some(&header));
        
        
        
        
        
/*
        let inc_button = gtk::Button::with_label("Increment");
        let dec_button = gtk::Button::with_label("Decrement");

        let label = gtk::Label::new(Some(&format!("Counter: {}", model.counter)));
        label.set_margin_all(5);

        
        vbox.set_margin_all(5);
        vbox.append(&inc_button);
        vbox.append(&dec_button);
        vbox.append(&label);

        inc_button.connect_clicked(clone!(
            #[strong]
            sender,
            move |_| {
                sender.input(AppMsg::Increment);
            }
        ));

        dec_button.connect_clicked(clone!(
            #[strong]
            sender,
            move |_| {
                sender.input(AppMsg::Decrement);
            }
        ));

        let widgets = AppWidgets { label };

        relm4::ComponentParts { model, widgets }
        
 */
        let widgets = AppWidgets { label: title_label };
        relm4::ComponentParts { model, widgets }

    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            AppMsg::Increment => self.counter = self.counter.wrapping_add(1),
            AppMsg::Decrement => self.counter = self.counter.wrapping_sub(1),
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: relm4::ComponentSender<Self>) {
        widgets
            .label
            .set_label(&format!("Current counter: {}", self.counter));
    }
}
