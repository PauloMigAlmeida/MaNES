use gtk4::{TextView, Align, TextBuffer};
use std::rc::Rc;
use crate::manes_bus;

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
            .buffer(&TextBuffer::builder().text(rom_disassembly_curr_state().as_str()).build())
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

    for i in (0x8000..=0xffff).step_by(16) {
        let v_0 = bus.read_u8(i);
        let v_1 = bus.read_u8(i + 1);
        let v_2 = bus.read_u8(i + 2);
        let v_3 = bus.read_u8(i + 3);
        let v_4 = bus.read_u8(i + 4);
        let v_5 = bus.read_u8(i + 5);
        let v_6 = bus.read_u8(i + 6);
        let v_7 = bus.read_u8(i + 7);
        let v_8 = bus.read_u8(i + 8);
        let v_9 = bus.read_u8(i + 9);
        let v_10 = bus.read_u8(i + 10);
        let v_11 = bus.read_u8(i + 11);
        let v_12 = bus.read_u8(i + 12);
        let v_13 = bus.read_u8(i + 13);
        let v_14 = bus.read_u8(i + 14);
        let v_15 = bus.read_u8(i + 15);
        content.push_str(format!("{:04X}: {:02X}{:02X}{:02X}{:02X}\
                                                 {:02X}{:02X}{:02X}{:02X}\
                                                 {:02X}{:02X}{:02X}{:02X}\
                                                 {:02X}{:02X}{:02X}{:02X}\n",
                                 i,
                                 v_0, v_1, v_2, v_3,
                                 v_4, v_5, v_6, v_7,
                                 v_8, v_9, v_10, v_11,
                                 v_12, v_13, v_14, v_15).as_str()
        );
    }

    content
}

