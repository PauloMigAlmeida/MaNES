use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Align, Orientation, Box};

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

    let load_rom_button = Button::with_label("Load ROM");
    let reset_button = Button::with_label("Reset");
    let save_state_button = Button::with_label("Save State");
    let about_button = Button::with_label("About");
    load_rom_button.connect_clicked(|_| {
        eprintln!("Clicked!");
    });
    
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
    
    window.set_child(Some(&menu_bar));    

    window.show();
}