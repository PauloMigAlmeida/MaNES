use gtk4::{TextView, Align, TextBuffer};
use std::{rc::Rc};

use crate::manes_cpu;

thread_local!(
    static MANES_CPU_REGS_TEXTVIEW: Rc<TextView> = Rc::new({
        TextView::builder()
            .name("cpuregstextview")
            .editable(false)
            .accepts_tab(false)
            .halign(Align::Fill)
            .valign(Align::Fill)
            .monospace(true)
            .focusable(false)
            .can_target(false)
            .buffer(&TextBuffer::builder().text(cpu_register_curr_state().as_str()).build())
            .build()
    });
);

pub fn manes_cpu_regs_textview() -> Rc<TextView> {
    MANES_CPU_REGS_TEXTVIEW.with(|x| x.clone())
}

pub fn cpu_register_curr_state() -> String {
    let rc_cpu = manes_cpu();
    let cpu = rc_cpu.as_ref().borrow();

    let mut content = String::new();
    content.push_str("[CPU Registers]\n\n");
    content.push_str(format!(" A: {0:02X} [{0:03}] \n", cpu.a).as_str());
    content.push_str(format!(" X: {0:02X} [{0:03}] \n", cpu.x).as_str());
    content.push_str(format!(" Y: {0:02X} [{0:03}] \n", cpu.y).as_str());
    content.push_str(format!("PC: {0:02X} [{0:03}] \n", cpu.pc).as_str());
    content.push_str(format!("SP: {0:02X} [{0:03}] \n", cpu.sp).as_str());
    content.push_str(format!("FL: {0:02X} [{0:03}] \n", cpu.flags).as_str());

    content
}