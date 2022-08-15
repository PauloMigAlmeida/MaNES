use std::ops::Deref;
use gtk4::prelude::*;
use gtk4::glib::clone;
use gtk4::{ApplicationWindow, Button, TextBuffer};
use std::rc::Rc;
use crate::{cpu_register_curr_state, manes_bus, manes_cpu, manes_cpu_regs_textview, manes_mem_view_textview, manes_rom_disassembly_textview};
use crate::ui::textview::mem_view::mem_view_curr_state;
use crate::ui::textview::rom_disassembly::rom_disassembly_curr_state;

thread_local!(
    static MANES_RESET_BUTTON: Rc<Button> =
        Rc::new(Button::builder().name("reset").label("Reset").build());
);

pub fn manes_reset_button() -> Rc<Button> {
    MANES_RESET_BUTTON.with(|x| x.clone())
}

pub fn load_reset_button_events_setup(window: &ApplicationWindow) {
    manes_reset_button()
        .as_ref()
        .connect_clicked(
            move |_| {
                println!("Clickou");

                manes_bus()
                    .as_ref()
                    .borrow_mut()
                    .reset();

                manes_cpu()
                    .as_ref()
                    .borrow_mut()
                    .reset(manes_bus()
                                    .as_ref()
                                    .borrow_mut()
                                    .deref()
                    );

                //TODO add additional components as required


                manes_rom_disassembly_textview()
                    .as_ref()
                    .set_buffer(Some(&TextBuffer::builder()
                        .text(rom_disassembly_curr_state().as_str())
                        .build())
                    );

                manes_cpu_regs_textview()
                    .as_ref()
                    .set_buffer(Some(&TextBuffer::builder()
                                        .text(cpu_register_curr_state().as_str())
                                        .build())
                    );

                manes_mem_view_textview()
                    .as_ref()
                    .set_buffer(Some(&TextBuffer::builder()
                        .text(mem_view_curr_state().as_str())
                        .build())
                    );
            }
    );

}
