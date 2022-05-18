use crate::manes_bus;
use gtk4::{Align, TextBuffer, TextView};
use mos6502_disassembler::disassemble_program;
use std::rc::Rc;
use bus::ROM_START_ADDR;

thread_local!(
    static MANES_ROM_DISASSEMBLY_TEXTVIEW: Rc<TextView> = Rc::new({
        TextView::builder()
            .name("disassemblytextview")
            .editable(false)
            .accepts_tab(false)
            .halign(Align::Fill)
            .valign(Align::Fill)
            .monospace(true)
            .focusable(false)
            .can_target(false)
            .buffer(
                &TextBuffer::builder()
                    .text(rom_disassembly_curr_state().as_str())
                    .build(),
            )
            .build()
    });
);

pub fn manes_rom_disassembly_textview() -> Rc<TextView> {
    MANES_ROM_DISASSEMBLY_TEXTVIEW.with(|x| x.clone())
}

pub fn rom_disassembly_curr_state() -> String {
    let rc_bus = manes_bus();
    let bus = rc_bus.as_ref().borrow();

    let mut content = String::new();
    content.push_str("[ROM Disassembly]\n\n");

    //TODO replace it when Mapper logic is implemented
    // let rom_mem = bus.read_u8_slice(ROM_START_ADDR, 0xffff);
    let rom_mem = bus.read_u8_slice(0x0, 0x1fff);
    content.push_str(disassemble_program(rom_mem, ROM_START_ADDR, true).as_str());
    content
}
