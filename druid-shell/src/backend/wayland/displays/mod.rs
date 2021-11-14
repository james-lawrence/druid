use lazy_static::lazy_static;
use wayland_client::{
    self as wl,
};
use super::error::Error;

lazy_static! {
    pub static ref GLOBAL: Singleton = Singleton::new();
}

pub trait Manager {
    // fn available(&self) -> impl std::iter::Iterator<Output>;
    // fn preferred(&self) -> outputs::Output;
}

pub struct Globals {
    pub display: wl::Display,
}

pub struct Singleton(std::sync::Mutex<std::cell::RefCell<Option<std::sync::Arc<Globals>>>>);

impl Singleton {
    fn new() -> Self {
        Self(std::sync::Mutex::new(std::cell::RefCell::new(None)))
    }
}

// initialize the global singleton in a threadsafe manner.
pub fn unwrap(instance: &Singleton) -> Result<std::sync::Arc<Globals>, Error> {
    let guard = instance.0.lock().unwrap();

    if let Some(m) = guard.borrow().clone() {
        return Ok(m);
    }

    let server = wl::Display::connect_to_env()?;

    // create an event queue (required for receiving events from the server)
    let queue = server.create_event_queue();
    let attached = server.clone().attach(queue.token());
    wl::GlobalManager::new_with_cb(
        &attached,
        move |event, registry, x| match event {
            wl::GlobalEvent::New { id, interface, version } if interface.as_str() == "wl_output" => {
                if version < 3 {
                    panic!("unsupported wayland output version {:?}", version);
                }

                tracing::info!("wayland output created detected");
            }
            wl::GlobalEvent::Removed { id, interface } if interface.as_str() == "wl_output" => {
                tracing::info!("wayland output removal detected");
            }
            _ => (), // ignore other interfaces
        },
    );

    let globals = std::sync::Arc::new(Globals{
        display: server.clone()
    });

    guard.borrow_mut().replace(globals.clone());

    return Ok(globals);
}

