use wayland_server::{WEnum, protocol::{wl_callback::WlCallback, wl_surface}};

use crate::{vulkan::content::SurfaceContent, wayland::*};

pub(crate) struct SurfaceState {
    pub current_buffer: Option<WlBuffer>,
    pub pending_buffer: Option<WlBuffer>,
    pub frame_callback: Option<WlCallback>,
}

impl SurfaceState {
    pub fn new() -> Self {
        SurfaceState {
            current_buffer: None,
            pending_buffer: None,
            frame_callback: None,
        }
    }

    pub fn frame(&mut self, callback: WlCallback) {
        self.frame_callback = Some(callback);
    }

    pub fn attach_buffer(&mut self, buffer: Option<WlBuffer>) {
        self.pending_buffer = buffer;
    }

    pub fn commit(&mut self) {
        self.current_buffer = self.pending_buffer.take();
    }

}

impl Dispatch<WlSurface, ()> for DisplayState {
    fn request(
        state: &mut Self,
        client: &wayland_server::Client,
        resource: &WlSurface,
        request: wayland_server::protocol::wl_surface::Request,
        data: &(),
        dhandle: &wayland_server::DisplayHandle,
        data_init: &mut wayland_server::DataInit<'_, Self>,
    ) {
        match request {
            wl_surface::Request::Destroy => {
                log::warn!("wl_surface destroyed");
            },
            wl_surface::Request::Attach { buffer, x, y } => {
                log::warn!("wl_surface attach buffer: {:?} at ({}, {})", buffer, x, y);

                state.get_surface_state_mut(resource.id()).attach_buffer(buffer);
            },
            wl_surface::Request::Damage { x, y, width, height } => {
                log::warn!("wl_surface damage at ({}, {}) size ({}, {})", x, y, width, height);
            },
            wl_surface::Request::Frame { callback } => {
                log::warn!("wl_surface frame callback: {:?}", callback);
                let callback = data_init.init(callback, callback::CallbackState {});
                state.get_surface_state_mut(resource.id()).frame(callback);
            },
            wl_surface::Request::SetOpaqueRegion { region } => {
                log::warn!("wl_surface set opaque region: {:?}", region);
            },
            wl_surface::Request::SetInputRegion { region } => {
                log::warn!("wl_surface set input region: {:?}", region);
            },
            wl_surface::Request::Commit => {
                log::warn!("wl_surface commit");
                state.get_surface_state_mut(resource.id()).commit();
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