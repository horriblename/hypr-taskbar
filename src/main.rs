use std::{cell::RefCell, rc::Rc};

use hypr_taskbar::{server::HyprMgr, taskbar::Taskbar};

fn main() {
    let mut server = HyprMgr::new();
    if let Ok(taskbar) = Taskbar::new() {
        let tb = Rc::new(RefCell::new(taskbar));

        let ref1 = tb.clone();
        server
            .ev_listener
            .add_workspace_change_handler(move |ws| ref1.borrow_mut().on_workspace_change(ws));

        let ref2 = tb.clone();
        server
            .ev_listener
            .add_window_open_handler(move |win| ref2.borrow_mut().on_window_open(win));

        let ref3 = tb.clone();
        server
            .ev_listener
            .add_window_close_handler(move |win| ref3.borrow_mut().on_window_close(win));

        let ref4 = tb.clone();
        server
            .ev_listener
            .add_window_moved_handler(move |move_ev| ref4.borrow_mut().on_window_moved(move_ev))
    } else {
        todo!("log error");
    }

    if let Err(err) = server.start_listener_blocking() {
        todo!("log error: {}", err);
    };
}
