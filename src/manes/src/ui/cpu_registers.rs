use crate::manes_cpu;

pub fn cpu_register_curr_state() -> String {
    let rc_cpu = manes_cpu();
    let cpu = rc_cpu.as_ref().borrow();

    let mut content = String::new();
    content.push_str(format!("_A: {0:02X} [{0:03}] \n", cpu.a).as_str());
    content.push_str(format!("_X: {0:02X} [{0:03}] \n ", cpu.x).as_str());
    content.push_str(format!("_Y: {0:02X} [{0:03}] \n", cpu.y).as_str());
    content.push_str(format!("PC: {0:02X} [{0:03}] \n", cpu.pc).as_str());
    content.push_str(format!("SP: {0:02X} [{0:03}] \n", cpu.sp).as_str());
    content.push_str(format!("FL: {0:02X} [{0:03}] \n", cpu.flags).as_str());

    content
}