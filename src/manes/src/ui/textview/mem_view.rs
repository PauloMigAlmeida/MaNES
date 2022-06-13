use gtk4::{TextView, Align, TextBuffer};
use std::rc::Rc;
use crate::manes_bus;

thread_local!(
    static MANES_MEM_VIEW_TEXTVIEW: Rc<TextView> = Rc::new({
        TextView::builder()
            .name("memviewtextview")
            .editable(false)
            .accepts_tab(false)
            .halign(Align::Fill)
            .valign(Align::Fill)
            .monospace(true)
            .focusable(false)
            .can_target(false)
            .buffer(&TextBuffer::builder().text(mem_view_curr_state().as_str()).build())
            .build()
    });
);

pub fn manes_mem_view_textview() -> Rc<TextView> {
    MANES_MEM_VIEW_TEXTVIEW.with(|x| x.clone())
}

pub fn mem_view_curr_state() -> String {
    let rc_bus = manes_bus();
    let bus = rc_bus.as_ref().borrow();

    let mut content = String::new();
    content.push_str("[Memory Area Visualisation]\n\n");

    //TODO replace with it full address space once mappers are in-place (0xffff)
    for i in (0..=0x1fff).step_by(16) {
        let v_0 = bus.cpu_read_u8(i);
        let v_1 = bus.cpu_read_u8(i + 1);
        let v_2 = bus.cpu_read_u8(i + 2);
        let v_3 = bus.cpu_read_u8(i + 3);
        let v_4 = bus.cpu_read_u8(i + 4);
        let v_5 = bus.cpu_read_u8(i + 5);
        let v_6 = bus.cpu_read_u8(i + 6);
        let v_7 = bus.cpu_read_u8(i + 7);
        let v_8 = bus.cpu_read_u8(i + 8);
        let v_9 = bus.cpu_read_u8(i + 9);
        let v_10 = bus.cpu_read_u8(i + 10);
        let v_11 = bus.cpu_read_u8(i + 11);
        let v_12 = bus.cpu_read_u8(i + 12);
        let v_13 = bus.cpu_read_u8(i + 13);
        let v_14 = bus.cpu_read_u8(i + 14);
        let v_15 = bus.cpu_read_u8(i + 15);
        content.push_str(format!("{:04X}: {:02X} {:02X} {:02X} {:02X} \
                                                 {:02X} {:02X} {:02X} {:02X} \
                                                 {:02X} {:02X} {:02X} {:02X} \
                                                 {:02X} {:02X} {:02X} {:02X} \
                                                 \t\
                                                 {}{}{}{}\
                                                 {}{}{}{}\
                                                 {}{}{}{}\
                                                 {}{}{}{}\n",
                                 i,
                                 v_0, v_1, v_2, v_3,
                                 v_4, v_5, v_6, v_7,
                                 v_8, v_9, v_10, v_11,
                                 v_12, v_13, v_14, v_15,
                                 prettify_char(v_0), prettify_char(v_1), prettify_char(v_2), prettify_char(v_3),
                                 prettify_char(v_4), prettify_char(v_5), prettify_char(v_6), prettify_char(v_7),
                                 prettify_char(v_8), prettify_char(v_9), prettify_char(v_10), prettify_char(v_11),
                                 prettify_char(v_12), prettify_char(v_13), prettify_char(v_14), prettify_char(v_15),

                        ).as_str()
        );
    }

    content
}

/// Not all ASCII characters play in our favour when trying to display them in a text view. If we
/// happen to get one of those, we will replace it with '.' for better visualisation
fn prettify_char(c: u8) -> char {
    let mut ch = c as char;
    if c < 0x21 || c > 0x7e {
        ch = '.';
    }
    ch
}