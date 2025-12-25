use crate::wayland::*;

use wayland_server::{Dispatch, protocol::wl_compositor::WlCompositor};

pub struct CompositorState {}

impl CompositorState {
    pub fn new() -> Self {
        CompositorState {}
    }
}

impl Dispatch<WlCompositor, CompositorState> for DisplayState {
    fn request(
        state: &mut Self,
        client: &wayland_server::Client,
        resource: &WlCompositor,
        request: <WlCompositor as wayland_server::Resource>::Request,
        data: &CompositorState,
        dhandle: &wayland_server::DisplayHandle,
        data_init: &mut wayland_server::DataInit<'_, Self>,
    ) {
        match request {
            wayland_server::protocol::wl_compositor::Request::CreateSurface { id } => {
                data_init.init(id, surface::SurfaceState {});
            }
            wayland_server::protocol::wl_compositor::Request::CreateRegion { id } => {
                data_init.init(id, region::RegionState {});
            }
            _ => {
                log::warn!("Unhandled wl_compositor request: {:?}", request);
            }
        }
    }
}
