use std::borrow::{Borrow, BorrowMut};
use gtk4::glib::clone;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box, Button, FileChooserAction, FileChooserDialog,
    GLArea, Orientation, Paned, PolicyType, ResponseType, ScrolledWindow, TextBuffer, TextView,
};
use bus::Bus;
use std::{cell::RefCell, rc::Rc};
use std::ops::{Deref, DerefMut};
use mos6502::Mos6502;
use crate::ui::cpu_registers::cpu_register_curr_state;

mod ui;
use crate::ui::globals::{manes_app, manes_bus, manes_cpu, manes_cpu_regs_textview};

fn main() {
    manes_app().connect_activate(build_ui);
    manes_app().run();
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


    reset_button.connect_clicked(|_| {

        // manes_cpu()
        //     .as_ref()
        //     .borrow_mut().stack_push(0x10, manes_bus().as_ref().borrow_mut().deref_mut());

        manes_cpu_regs_textview()
            .as_ref()
            .set_buffer(Some(&TextBuffer::builder()
                                .text(cpu_register_curr_state().as_str())
                                .build())
            );
    });

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

    vertical_pane.set_position(400); // derive this from window size somehow

    vertical_pane
}

fn build_left_side_panes() -> Paned {
    let disassembly_textview = TextView::builder()
        .editable(true)
        .accepts_tab(false)
        .halign(Align::Fill)
        .valign(Align::Fill)
        .buffer(&TextBuffer::builder().text("ROM Disassembly").build())
        .build();

    let disassembly_scroll = ScrolledWindow::builder()
        .child(&disassembly_textview)
        .halign(Align::Fill)
        .valign(Align::Fill)
        .vscrollbar_policy(PolicyType::Always)
        .build();

    let memory_textview = TextView::builder()
        .editable(true)
        .accepts_tab(false)
        .halign(Align::Fill)
        .valign(Align::Fill)
        .buffer(
            &TextBuffer::builder()
                .text("Memory Area Visualisation")
                .build(),
        )
        .build();

    let memory_scroll = ScrolledWindow::builder()
        .child(&memory_textview)
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
