use crate::wayland::*;

pub struct XdgSurfaceState;

impl Dispatch<XdgSurface, XdgSurfaceState> for DisplayState {
    fn request(
        state: &mut Self,
        client: &wayland_server::Client,
        resource: &XdgSurface,
        request: <XdgSurface as wayland_server::Resource>::Request,
        data: &XdgSurfaceState,
        dhandle: &wayland_server::DisplayHandle,
        data_init: &mut wayland_server::DataInit<'_, Self>,
    ) {
        log::warn!("Unhandled xdg_surface request: {:?}", request);
        match request {
            Request::Destroy => todo!(),
            Request::GetToplevel { id } => {
                let toplevel = data_init.init(id, xdg::xdg_toplevel::XdgToplevelState {});

                toplevel.configure(800, 600, vec![]);
                resource.configure(1);
            }
            Request::GetPopup {
                id,
                parent,
                positioner,
            } => todo!(),
            Request::SetWindowGeometry {
                x,
                y,
                width,
                height,
            } => {
                log::info!(
                    "xdg_surface set window geometry to ({}, {}, {}, {})",
                    x,
                    y,
                    width,
                    height
                );
            }
            Request::AckConfigure { serial } => {
                log::info!("xdg_surface acked configure with serial {}", serial);
            },
            _ => todo!(),
        }
    }
}