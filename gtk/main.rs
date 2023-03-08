/*
 * USING GTK-RC EXAMPLES (https://github.com/gtk-rs/gtk3-rs/blob/master/examples)
 * EDITED CLONE
 */

use chrono::Local;
use gtk::glib;
use gtk::prelude::*;

fn current_date() -> String {
    format!("{}", Local::now().format("%Y-%m-%d"))
}

fn current_time() -> String {
    format!(
        "{}",
        Local::now().format("24h: %H:%M:%S | 12h: %I:%M:%S %p")
    )
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    let time = current_time();
    let date = current_date();

    window.set_title(&date);
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(260, 40);

    let label = gtk::Label::new(None);
    label.set_text(&time);

    window.add(&label);

    window.show_all();

    let tick = move || {
        let time = current_time();
        let date = current_date();
        window.set_title(&date);
        label.set_text(&time);
        glib::Continue(true)
    };

    glib::timeout_add_seconds_local(1, tick);
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.clock"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}
