use gtk4::Application;
use std::{cell::RefCell, rc::Rc};
use bus::mos6502::Mos6502;
use bus::Bus;

thread_local!(
    static MANES_APPLICATION: Rc<Application> = Rc::new({
        let application_id = "com.github.paulomigalmeida.MaNES";
        println!("Setting up application with id '{}'", application_id);
        Application::builder()
            .application_id(application_id)
            .build()
    });

    static MANES_MOS6502_CPU: Rc<RefCell<Mos6502>> = Rc::new(
        RefCell::new(Mos6502::new())
    );

    static MANES_BUS: Rc<RefCell<Bus>> = Rc::new(
        RefCell::new(Bus::new())
    );
);


pub fn manes_app() -> Rc<Application> {
    MANES_APPLICATION.with(|x| x.clone())
}

pub fn manes_cpu() -> Rc<RefCell<Mos6502>> {
    MANES_MOS6502_CPU.with(|x| x.clone())
}

pub fn manes_bus() -> Rc<RefCell<Bus>> {
    MANES_BUS.with(|x| x.clone())
}
