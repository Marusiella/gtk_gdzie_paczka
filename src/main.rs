use std::{thread};

use glib::{Continue, MainContext};
use gtk::{Builder, glib::clone, prelude::{BuilderExtManual, ButtonExt, EntryExt, LabelExt, WidgetExt}};

use serde_json::*;
fn main() {
    // Create a new application
    gtk::init().unwrap();
    let app: Builder = Builder::from_file("./glade.glade");
    let button: gtk::Button = app.object("gdzie_jest_moja_paczka").unwrap();
    let window: gtk::Window = app.object("window").unwrap();
    let label: gtk::Label = app.object("label").unwrap();
    let paczkomat: gtk::Label = app.object("paczkomat").unwrap();
    let size: gtk::Label = app.object("size").unwrap();
    let input: gtk::Entry = app.object("input").unwrap();
    let (sender, receiver) = MainContext::channel(glib::PRIORITY_DEFAULT);
    window.connect_destroy(|_| {
        gtk::main_quit();
    });
    button.connect_clicked(clone!(@weak label => move |_|{
        label.set_text_with_mnemonic("Sprawdzanie");
    }));
    label.set_text_with_mnemonic("ttt");
    button.connect_clicked(clone!(@weak input => move |_| {
        let sender = sender.clone();
        let b = input.buffer().text();
        // The long running operation runs now in a separate thread
        thread::spawn(move || {
            // Deactivate the button until the operation is done
            let body = reqwest::blocking::get(
                format!("https://api-shipx-pl.easypack24.net/v1/tracking/{}",b),
            )
            .unwrap()
            .text()
            .unwrap();
            let x: Value = serde_json::from_str(&body).unwrap();
            sender
                .send(x)
                .expect("Could not send through channel");
        });
    }));

    // The main loop executes the closure as soon as it receives the message
    receiver.attach(None, clone!(@weak label, @weak paczkomat, @weak size => @default-return glib::Continue(true), move |t| {
        match t["status"].as_str() {
            Some(o) => label.set_text(&format!("Status: {}",o)),
            None => label.set_text("zły numer paczki/brak internetu"),
        }
        match t["custom_attributes"]["target_machine_id"].as_str() {
            Some(o) => paczkomat.set_text(&format!("Paczkomat docelowy: {}",o)),
            None => paczkomat.set_text("brak info. o pacz. docelowym"),
        }
        match t["custom_attributes"]["size"].as_str() {
            Some(o) => size.set_text(&format!("Rozmiar: {}",o)),
            None => size.set_text("brak inf. na temat rozmiaru"),
        }
        
        Continue(true)
    }));

    window.show_all();

    gtk::main();
}
