use wayland_server::protocol::wl_surface;

use crate::wayland::*;

pub struct SurfaceState {}

impl Dispatch<WlSurface, SurfaceState> for DisplayState {
    fn request(
        state: &mut Self,
        client: &wayland_server::Client,
        resource: &WlSurface,
        request: wayland_server::protocol::wl_surface::Request,
        data: &SurfaceState,
        dhandle: &wayland_server::DisplayHandle,
        data_init: &mut wayland_server::DataInit<'_, Self>,
    ) {
        match request {
            wl_surface::Request::Destroy => {
                log::warn!("wl_surface destroyed");
            },
            wl_surface::Request::Attach { buffer, x, y } => {
                
                log::warn!("wl_surface attach buffer: {:?} at ({}, {})", buffer, x, y);
            },
            wl_surface::Request::Damage { x, y, width, height } => {
                log::warn!("wl_surface damage at ({}, {}) size ({}, {})", x, y, width, height);
            },
            wl_surface::Request::Frame { callback } => {
                log::warn!("wl_surface frame callback: {:?}", callback);
                data_init.init(callback, callback::CallbackState {});
            },
            wl_surface::Request::SetOpaqueRegion { region } => {
                log::warn!("wl_surface set opaque region: {:?}", region);
            },
            wl_surface::Request::SetInputRegion { region } => {
                log::warn!("wl_surface set input region: {:?}", region);
            },
            wl_surface::Request::Commit => {
                log::warn!("wl_surface commit");
            },
            wl_surface::Request::SetBufferTransform { transform } => {
                log::warn!("wl_surface set buffer transform: {:?}", transform);
            },
            wl_surface::Request::SetBufferScale { scale } => {
                log::warn!("wl_surface set buffer scale: {}", scale);
            },
            wl_surface::Request::DamageBuffer { x, y, width, height } => {
                log::warn!("wl_surface damage buffer at ({}, {}) size ({}, {})", x, y, width, height);
            },
            wl_surface::Request::Offset { x, y } => {
                log::warn!("wl_surface offset by ({}, {})", x, y);
            },
            _ => todo!(),
        }
    }
}