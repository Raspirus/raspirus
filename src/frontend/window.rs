use gtk::prelude::*;

pub fn build(app: &gtk::Application) {
    let button = gtk::Button::builder()
       .label("Sondbutton")
       .margin_top(12)
       .margin_bottom(12)
       .margin_start(12)
       .margin_end(12)
       .build();
    
    button.connect_clicked(|button| {
        button.set_label("Drucktr button");
        println!("Sond passiert");
    });

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Raspirus")
        .child(&button)
        .build(); 

    window.present();
}
