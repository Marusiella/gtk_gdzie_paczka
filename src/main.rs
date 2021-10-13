use gtk::{Builder, prelude::{BuilderExt, BuilderExtManual, WidgetExt}};


fn main() {
    // Create a new application
    gtk::init().unwrap();
    let app: Builder = Builder::from_file("./glade.glade");
    let button: gtk::Button = app.object("button").unwrap();
    let window: gtk::Window = app.object("window").unwrap();
    window.connect_destroy(|_| {
        gtk::main_quit();
    });
    window.show_all();

    gtk::main();
}