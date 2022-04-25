use gtk4::{ApplicationWindow};
use std::rc::Rc;
use crate::manes_app;

pub const DEFAULT_WINDOW_WIDTH: i32 = 1100;
pub const DEFAULT_WINDOW_HEIGHT: i32 = 700;

thread_local!(
    static MANES_MAIN_UI: Rc<ApplicationWindow> = Rc::new({
        ApplicationWindow::builder()
            .application(manes_app().as_ref())
            .title(format!("MaNES emulator - version: {}", option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")).as_str())
            .default_width(DEFAULT_WINDOW_WIDTH)
            .default_height(DEFAULT_WINDOW_HEIGHT)
            .show_menubar(true)
            .maximized(false)
            .build()
    });
);

pub fn manes_main_ui() -> Rc<ApplicationWindow> {
    MANES_MAIN_UI.with(|x| x.clone())
}

