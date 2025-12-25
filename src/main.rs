use std::{os::fd::AsFd, sync::Arc};

use wayland_protocols::xdg::shell::server::xdg_wm_base::XdgWmBase;
use wayland_server::{ListeningSocket, backend::ClientData, protocol::{wl_compositor::WlCompositor, wl_shm::WlShm}};

mod wayland;
mod logging;

static WAYLAND_SOCKET_NAME: &str = "desenho-1";

fn main() {
    logging::init().unwrap();
    log::info!("Desenho initializing...");

    let mut wayland_state = wayland::DisplayState {  };
    let mut server = wayland_server::Display::<wayland::DisplayState>::new().unwrap();

    server.handle().create_global::<wayland::DisplayState, WlCompositor, _>(4, ());
    server.handle().create_global::<wayland::DisplayState, WlShm, _>(2, ());
    server.handle().create_global::<wayland::DisplayState, XdgWmBase, _>(2, ());
    // set env variable to use wayland display
    let socket = ListeningSocket::bind(WAYLAND_SOCKET_NAME).unwrap();


    loop {
        if let Ok(Some(stream)) = socket.accept() {
            server.handle().insert_client(stream, Arc::new(DummyClientData)).unwrap();
        }
        server.dispatch_clients(&mut wayland_state).unwrap();
        server.flush_clients().unwrap();
    }
}

struct DummyClientData;

impl ClientData for DummyClientData {}