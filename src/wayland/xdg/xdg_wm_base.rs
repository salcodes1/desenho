use wayland_protocols::xdg::shell::server::xdg_wm_base;

use crate::wayland::*;

pub struct XdgWmBaseState;

impl Dispatch<XdgWmBase, XdgWmBaseState> for DisplayState {
    fn request(
        state: &mut Self,
        client: &wayland_server::Client,
        resource: &XdgWmBase,
        request: <XdgWmBase as wayland_server::Resource>::Request,
        data: &XdgWmBaseState,
        dhandle: &wayland_server::DisplayHandle,
        data_init: &mut wayland_server::DataInit<'_, Self>,
    ) {
        log::warn!("Unhandled xdg_wm_base request: {:?}", request);

        match request {
            xdg_wm_base::Request::Destroy => todo!(),
            xdg_wm_base::Request::CreatePositioner {
                id,
            } => todo!(),
            xdg_wm_base::Request::GetXdgSurface {
                id,
                surface,
            } => {
                // 🎯 Here you'd normally initialize the xdg_surface
                data_init.init(id, xdg::xdg_surface::XdgSurfaceState {});
            }
            xdg_wm_base::Request::Pong { serial } => todo!(),
            _ => todo!(),
        }
    }
}