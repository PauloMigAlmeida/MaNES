use gtk4::glib::clone;
use gtk4::prelude::*;
use gtk4::{Align, Application, ApplicationWindow, Box, Button, CssProvider, FileChooserAction, FileChooserDialog, GLArea, Orientation, Paned, PolicyType, ResponseType, ScrolledWindow, StyleContext, TextBuffer};
use gtk4::gdk::Display;
mod ui;

use ui::globals::{manes_app, manes_bus, manes_cpu};
use ui::cpu_registers::{cpu_register_curr_state, manes_cpu_regs_textview};
use ui::mem_view::{manes_mem_view_textview};
use ui::window::{DEFAULT_WINDOW_WIDTH, manes_main_ui};
use crate::ui::rom_disassembly::manes_rom_disassembly_textview;

fn main() {
    manes_app().connect_activate(|_| load_css());
    manes_app().connect_activate(build_ui);
    manes_app().run();
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("assets/style.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(_app: &Application) {
    let container = Box::builder()
        .halign(Align::Fill)
        .valign(Align::Fill)
        .homogeneous(false)
        .spacing(5)
        .orientation(Orientation::Vertical)
        .build();

    let menu_bar = build_top_bar(manes_main_ui().as_ref());
    let content = build_ui_content(manes_main_ui().as_ref());
    container.append(&menu_bar);
    container.append(&content);

    manes_main_ui().as_ref().set_child(Some(&container));
    manes_main_ui().as_ref().show();
}

fn build_top_bar(window: &ApplicationWindow) -> Box {
    let load_rom_button = Button::builder().name("loadrom").label("Load ROM").build();
    let reset_button = Button::builder().name("reset").label("Reset").build();
    let save_state_button = Button::builder().name("savestate").label("Save State").build();
    let load_state_button = Button::builder().name("loadstate").label("Load State").build();
    let about_button = Button::builder().name("about").label("About").build();

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
    menu_bar.append(&load_state_button);
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


    reset_button.connect_clicked(clone!(@strong window =>
        move |_| {
            println!("{}", window.width());
            println!("{}", window.height());
            // manes_cpu()
            //     .as_ref()
            //     .borrow_mut().stack_push(0x10, manes_bus().as_ref().borrow_mut().deref_mut());

            manes_cpu_regs_textview()
                .as_ref()
                .set_buffer(Some(&TextBuffer::builder()
                                    .text(cpu_register_curr_state().as_str())
                                    .build())
                );
        }
    ));

    menu_bar
}

fn build_ui_content(_window: &ApplicationWindow) -> Paned {
    let left_side_pane = build_left_side_panes();
    let right_side_pane = build_right_side_panes();

    let vertical_pane = Paned::builder()
        .orientation(Orientation::Horizontal)
        .hexpand(true)
        .halign(Align::Fill)
        .vexpand(true)
        .valign(Align::Fill)
        .margin_start(5)
        .margin_end(5)
        .margin_top(5)
        .start_child(&left_side_pane)
        .end_child(&right_side_pane)
        .build();

    vertical_pane.set_position(DEFAULT_WINDOW_WIDTH / 2 - 5 /* margin */);

    vertical_pane
}

fn build_left_side_panes() -> Paned {
    let disassembly_scroll = ScrolledWindow::builder()
        .child(manes_rom_disassembly_textview().as_ref())
        .halign(Align::Fill)
        .valign(Align::Fill)
        .vscrollbar_policy(PolicyType::Always)
        .build();

    let memory_scroll = ScrolledWindow::builder()
        .child(manes_mem_view_textview().as_ref())
        .halign(Align::Fill)
        .valign(Align::Fill)
        .vscrollbar_policy(PolicyType::Always)
        .build();

    Paned::builder()
        .orientation(Orientation::Vertical)
        .hexpand(true)
        .halign(Align::Fill)
        .vexpand(true)
        .valign(Align::Fill)
        .start_child(&disassembly_scroll)
        .end_child(&memory_scroll)
        .build()
}

fn build_right_side_panes() -> Paned {
    let cpu_textview = manes_cpu_regs_textview();

    let game_display = GLArea::builder()
        .halign(Align::Fill)
        .valign(Align::Fill)
        .build();


    Paned::builder()
        .orientation(Orientation::Vertical)
        .hexpand(true)
        .halign(Align::Fill)
        .vexpand(true)
        .valign(Align::Fill)
        .start_child(&game_display)
        .end_child(cpu_textview.as_ref())
        .build()
}
