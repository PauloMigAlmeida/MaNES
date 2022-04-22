use gtk::glib::clone;
use gtk::prelude::*;
use gtk::{
    Align, Application, ApplicationWindow, Box, Button, FileChooserAction, FileChooserDialog,
    Orientation, Paned, ResponseType,
};
use gtk4 as gtk;
use gtk4::{TextBuffer, TextView};

fn main() {
    let application = Application::builder()
        .application_id("com.github.paulomigalmeida.MaNES")
        .build();

    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(app: &Application) {
    let version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");
    let title = format!("MaNES emulator - version: {}", version);

    let window = ApplicationWindow::builder()
        .application(app)
        .title(&title[..])
        .default_width(800)
        .default_height(600)
        .show_menubar(true)
        .build();

    let container = Box::builder()
        .halign(Align::Fill)
        .valign(Align::Fill)
        .homogeneous(false)
        .spacing(5)
        .orientation(Orientation::Vertical)
        .build();

    let menu_bar = build_top_bar(&window);
    let content = build_ui_content(&window);
    container.append(&menu_bar);
    container.append(&content);

    window.set_child(Some(&container));
    window.show();
}

fn build_top_bar(window: &ApplicationWindow) -> Box {
    let load_rom_button = Button::with_label("Load ROM");
    let reset_button = Button::with_label("Reset");
    let save_state_button = Button::with_label("Save State");
    let about_button = Button::with_label("About");

    let menu_bar = Box::builder()
        .hexpand(false)
        .halign(Align::Fill)
        .vexpand(false)
        .valign(Align::Start)
        .margin_start(5)
        .margin_end(5)
        .margin_top(5)
        .homogeneous(true)
        .spacing(5)
        .orientation(Orientation::Horizontal)
        .build();

    menu_bar.append(&load_rom_button);
    menu_bar.append(&reset_button);
    menu_bar.append(&save_state_button);
    menu_bar.append(&about_button);

    load_rom_button.connect_clicked(clone!(@strong window =>
        move |_| {
            let f = FileChooserDialog::new(
                Some("Load ROM"),
                Some(&window),
                FileChooserAction::Open,
                &[("OK", ResponseType::Ok), ("Cancel", ResponseType::Cancel)]
            );
            f.set_modal(true);
            f.show();

            f.connect_response( | dialog, resp| {
                match resp {
                    ResponseType::Ok => println!("Chose OK"),
                    ResponseType::Cancel => println!("Chose Cancel"),
                    _ => println!("Chose close"),
                }
                dialog.close();
            });
        }
    ));

    menu_bar
}

fn build_ui_content(window: &ApplicationWindow) -> Paned {
    let ola1 = Button::with_label("Ola 1");
    let ola2 = Button::with_label("Ola 2");

    let disassembly_textview = TextView::builder()
        .editable(true)
        .accepts_tab(true)
        .hexpand(true)
        .vexpand(true)
        .buffer(&TextBuffer::builder().text("Ola").build())
        .build();

    let vertical_pane = Paned::builder()
        .orientation(Orientation::Horizontal)
        .hexpand(false)
        .halign(Align::Fill)
        .vexpand(false)
        .valign(Align::Fill)
        .margin_start(5)
        .margin_end(5)
        .margin_top(5)
        .start_child(&disassembly_textview)
        .end_child(&ola2)
        .build();

    vertical_pane.set_position(400); // derive this from window size somehow

    vertical_pane
}
