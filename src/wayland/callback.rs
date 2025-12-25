use wayland_server::protocol::wl_callback::WlCallback;
use wayland_server::protocol::wl_callback;

use crate::wayland::*;

pub struct CallbackState;

impl Dispatch<WlCallback, CallbackState> for DisplayState {
    fn request(
        state: &mut Self,
        client: &wayland_server::Client,
        resource: &WlCallback,
        request: wayland_server::protocol::wl_callback::Request,
        data: &CallbackState,
        dhandle: &wayland_server::DisplayHandle,
        data_init: &mut wayland_server::DataInit<'_, Self>,
    ) {
        log::warn!("Unhandled wl_callback request: {:?}", request);
    }
}