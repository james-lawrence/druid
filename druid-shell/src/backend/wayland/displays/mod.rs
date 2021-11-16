use lazy_static::lazy_static;
use wayland_client::{
    self as wl,
};
use wayland_protocols::unstable::xdg_decoration::v1::client::zxdg_decoration_manager_v1::ZxdgDecorationManagerV1;
use wayland_protocols::unstable::xdg_output::v1::client::zxdg_output_manager_v1::ZxdgOutputManagerV1;
use wayland_protocols::wlr::unstable::layer_shell::v1::client::zwlr_layer_shell_v1::ZwlrLayerShellV1;
use wayland_protocols::xdg_shell::client::xdg_wm_base::XdgWmBase;
use wayland_client::protocol::wl_compositor::WlCompositor;
use wayland_client::protocol::wl_shm::WlShm;

use super::error::Error;
// mod outputs;

lazy_static! {
    pub static ref GLOBAL: Singleton = Singleton::new();
}

pub struct Globals {
    pub display: wl::Display,
    // pub xdgoutput: wl::Main<ZxdgOutputManagerV1>,
    // pub xdgbase: wl::Main<XdgWmBase>,
    // outputs: Vec<outputs::Output>,
}

pub struct Singleton(std::sync::Mutex<std::cell::RefCell<Option<std::sync::Arc<Globals>>>>);

impl Singleton {
    fn new() -> Self {
        Self(std::sync::Mutex::new(std::cell::RefCell::new(None)))
    }
}

// pub fn outputs(instance: &Singleton) -> Vec<outputs::Output> {
//     let guard = instance.0.lock().unwrap();
//     if let Some(m) = guard.borrow().clone() {
//         return m.outputs.clone()
//     }

//     Vec::new()
// }

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
    let globalm = wl::GlobalManager::new(&attached);
    let globals = std::sync::Arc::new(Globals{
        display: server.clone(),
        // outputs: Vec::new(),
    });

    if let Err(cause) = manage_xdg_outputs(&globalm) {
        tracing::info!("unable to connect to xdg output manager v1 protocol");
    }

    // if let Err(cause) = manage_wlr_outputs(&globalm) {
    //     tracing::info!("unable to connect to wlr output manager v1 protocol");
    // }

    let xdgbase = globalm.instantiate_exact::<XdgWmBase>(2).map_err(|e| Error::global("xdg_wm_base", 2, e))?;
    let zxdg_decoration_manager_v1 = globalm.instantiate_exact::<ZxdgDecorationManagerV1>(1).map_err(|e| Error::global("zxdg_decoration_manager_v1", 1, e))?;
    let zwlr_layershell_v1 = globalm.instantiate_exact::<ZwlrLayerShellV1>(1).map_err(|e| Error::global("zwlr_layershell_v1", 1, e))?;
    let wl_compositor = globalm.instantiate_exact::<WlCompositor>(4).map_err(|e| Error::global("wl_compositor", 4, e))?;
    let wl_shm = globalm.instantiate_exact::<WlShm>(1).map_err(|e| Error::global("wl_shm", 1, e))?;
    // let globalm = wl::GlobalManager::new_with_cb(
    //     &attached,
    //     move |event, registry, x| match event {
    //         wl::GlobalEvent::New { id, interface, version } if interface.as_str() == "wl_output" => {
    //             if version < 3 {
    //                 panic!("unsupported wayland output version {:?}", version);
    //             }
    //             tracing::info!("wayland output created detected");
    //         }
    //         wl::GlobalEvent::Removed { id, interface } if interface.as_str() == "wl_output" => {
    //             tracing::info!("wayland output removal detected");
    //         }
    //         _ => (), // ignore other interfaces
    //     },
    // );

    guard.borrow_mut().replace(globals.clone());

    return Ok(globals);
}

fn manage_xdg_outputs<'a>(globalm: &'a wl::GlobalManager) -> Result<(), Error> {
    let xdgoutput = globalm.instantiate_exact::<ZxdgOutputManagerV1>(1).map_err(|e| Error::global("zxdg_output_manager_v1", 1, e))?;

    Ok(())
}

fn manage_wlr_outputs<'a>(globalm: &'a wl::GlobalManager) -> Result<(), Error> {
    Ok(())
}