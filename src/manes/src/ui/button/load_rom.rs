use gtk4::prelude::*;
use gtk4::glib::clone;
use gtk4::{ApplicationWindow, Button, FileChooserDialog, FileChooserAction, ResponseType, TextBuffer};
use std::rc::Rc;
use crate::{manes_bus};
use crate::ui::textview::rom_disassembly::{rom_disassembly_curr_state,manes_rom_disassembly_textview};
use crate::ui::textview::mem_view::{mem_view_curr_state, manes_mem_view_textview};
use bus::ROM_START_ADDR;

thread_local!(
    static MANES_LOAD_ROM_BUTTON: Rc<Button> =
        Rc::new(Button::builder().name("loadrom").label("Load ROM").build());
);

pub fn manes_load_rom_button() -> Rc<Button> {
    MANES_LOAD_ROM_BUTTON.with(|x| x.clone())
}

pub fn load_rom_button_events_setup(window: &ApplicationWindow) {
    manes_load_rom_button()
        .as_ref()
        .connect_clicked(
            clone!(@strong window =>
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
                        ResponseType::Ok => {
                            println!("Chose OK");
                            let file = dialog.file().expect("A file must specified");

                            let mut filename = String::from(file.uri().as_str());
                            if filename.starts_with("file://"){
                                filename = filename.replace("file://","");
                            }
                            println!("Loading to ram");

                            println!("filename: {}", filename);

                            manes_bus()
                                .as_ref()
                                .borrow_mut()
                                .load_cartridge(filename.as_str());

                            println!("Disassembling");
                            manes_rom_disassembly_textview()
                                .as_ref()
                                .set_buffer(Some(&TextBuffer::builder()
                                    .text(rom_disassembly_curr_state().as_str())
                                    .build())
                                );

                            manes_rom_disassembly_textview()
                                .as_ref()
                                .set_buffer(Some(&TextBuffer::builder()
                                    .text(rom_disassembly_curr_state().as_str())
                                    .build())
                                );

                            manes_mem_view_textview()
                                .as_ref()
                                .set_buffer(Some(&TextBuffer::builder()
                                    .text(mem_view_curr_state().as_str())
                                    .build())
                                );
                            println!("Done");
                        },
                        ResponseType::Cancel => println!("Chose Cancel"),
                        _ => println!("Chose close"),
                    }
                    dialog.close();
                });
            }
        )
    );

}
