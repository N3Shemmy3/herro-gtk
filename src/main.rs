mod custom_button;

use std::cell::Cell;
use std::rc::Rc;

use custom_button::CustomButton;
use glib::clone;
use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow, Switch, Orientation, Box, glib, Align, Button};
use gtk::ffi::GtkSwitch;
use gtk::glib::closure_local;

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);
    // Run the application
    app.run()
}


fn build_ui(app: &Application) {
    let button = Button::new();

    button.connect_clicked(|button| {
       button.set_label("Herro hero")
    });

    button.connect_closure(
        "clicked",
        false,
        closure_local!(move |button: Button| {
            button.set_label("Herro Nagatoro")
        }),
    );

    let gtk_box = Box::builder()
        .margin_start(12)
        .margin_top(12)
        .margin_end(12)
        .margin_bottom(12)
        .valign(Align::Center)
        .halign(Align::Center)
        .spacing(12)
        .orientation(Orientation::Vertical)
        .build();

    gtk_box.append(&button);
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK APP")
        .child(&gtk_box)
        .build();


    window.present();
}
