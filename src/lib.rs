pub mod taskbar;

pub mod server {
    use hyprland::{event_listener::EventListener, shared::HResult};

    pub struct HyprMgr {
        // TODO abstract away and make private
        pub ev_listener: EventListener,
    }

    // listens to hyprland events
    impl HyprMgr {
        pub fn new() -> HyprMgr {
            eprintln!("initializing event listener...");
            HyprMgr {
                ev_listener: EventListener::new(),
            }
        }

        pub fn start_listener_blocking(self) -> HResult<()> {
            eprintln!("started blocking listener");
            self.ev_listener.start_listener()
        }
    }
}
