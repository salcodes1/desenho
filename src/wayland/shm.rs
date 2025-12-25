use crate::wayland::*;

pub struct ShmState;
pub struct ShmPoolState;
struct BufferState;

impl Dispatch<WlShm, ShmState> for DisplayState {
    fn request(
        state: &mut Self,
        client: &wayland_server::Client,
        resource: &WlShm,
        request: wayland_server::protocol::wl_shm::Request,
        data: &ShmState,
        dhandle: &wayland_server::DisplayHandle,
        data_init: &mut wayland_server::DataInit<'_, Self>,
    ) {
        match request {
            wayland_server::protocol::wl_shm::Request::CreatePool { id, fd, size } => {
                // 💡 You may want to store fd + size in your state
                data_init.init(id, ShmPoolState {});
            }
            _ => {}
        }
    }
}

impl Dispatch<WlShmPool, ShmPoolState> for DisplayState {
    fn request(
        state: &mut Self,
        client: &wayland_server::Client,
        resource: &WlShmPool,
        request: <WlShmPool as wayland_server::Resource>::Request,
        data: &ShmPoolState,
        dhandle: &wayland_server::DisplayHandle,
        data_init: &mut wayland_server::DataInit<'_, Self>,
    ) {
        match request {
            wayland_server::protocol::wl_shm_pool::Request::CreateBuffer {
                id,
                offset,
                width,
                height,
                stride,
                format,
            } => {
                // 🎯 Here you'd normally validate bounds & format
                data_init.init(id, BufferState);
            }

            wayland_server::protocol::wl_shm_pool::Request::Resize { .. } => {
                // optional to support properly for now
            }
            _ => {}
        }
    }
}

impl Dispatch<WlBuffer, BufferState> for DisplayState {
    fn request(
        state: &mut Self,
        client: &wayland_server::Client,
        resource: &WlBuffer,
        request: <WlBuffer as wayland_server::Resource>::Request,
        data: &BufferState,
        dhandle: &wayland_server::DisplayHandle,
        data_init: &mut wayland_server::DataInit<'_, Self>,
    ) {
        log::warn!("Unhandled wl_buffer request: {:?}", request);
    }
}
