pub mod messages;
pub mod model;
pub mod widget;

use relm4::{
    gtk::{self, glib::clone, prelude::*},
    RelmWidgetExt, SimpleComponent,
};

use self::{messages::AppMsg, model::AppModel, widget::AppWidgets};

impl SimpleComponent for AppModel {
    type Input = AppMsg;
    type Output = ();
    type Init = u8;
    type Root = gtk::Window;
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        gtk::Window::builder()
            .title("Simple counter")
            .default_width(300)
            .default_height(100)
            .build()
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = AppModel { counter: init };
        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .build();

        let inc_button = gtk::Button::with_label("+");
        let dec_button = gtk::Button::with_label("-");

        let label = gtk::Label::new(Some(&format!("Current counter: {}", model.counter)));
        label.set_margin_all(5);

        root.set_child(Some(&vbox));
        vbox.set_margin_all(5);
        vbox.append(&inc_button);
        vbox.append(&dec_button);
        vbox.append(&label);

        inc_button.connect_clicked(clone!(
            #[strong]
            sender,
            move |_| sender.input(AppMsg::Increment),
        ));

        dec_button.connect_clicked(clone!(
            #[strong]
            sender,
            move |_| sender.input(AppMsg::Decrement)
        ));

        let widgets = AppWidgets { label };

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
