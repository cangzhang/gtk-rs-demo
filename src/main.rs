use std::sync::{Arc, Mutex};
// use std::thread;

use gtk::{prelude::*, Button, Text};
use gtk::{Application, ApplicationWindow};

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("press me")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let count = Arc::new(Mutex::new(0));
    // let count = count.clone();
    button.connect_clicked(move |button| {
        *count.lock().unwrap() += 1;
        let cnt = *count.lock().unwrap();
        let text = format!("clicked {cnt}");
        button.set_label(&text);
    });

    let input = Text::builder().build();

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        // .child(&input)
        .build();

    // Present window
    window.present();
}
