use crate::wayland::*;

pub struct XdgToplevelState;

impl Dispatch<XdgToplevel, XdgToplevelState> for DisplayState {
    fn request(
        state: &mut Self,
        client: &wayland_server::Client,
        resource: &XdgToplevel,
        request: <XdgToplevel as wayland_server::Resource>::Request,
        data: &XdgToplevelState,
        dhandle: &wayland_server::DisplayHandle,
        data_init: &mut wayland_server::DataInit<'_, Self>,
    ) {
        match request {
            wayland_protocols::xdg::shell::server::xdg_toplevel::Request::Destroy => {
                log::warn!("xdg_toplevel destroyed");
            }
            wayland_protocols::xdg::shell::server::xdg_toplevel::Request::SetParent { parent } => {
                log::warn!("xdg_toplevel set parent: {:?}", parent);
            }
            wayland_protocols::xdg::shell::server::xdg_toplevel::Request::SetTitle { title } => {
                log::info!("xdg_toplevel set title: {}", title);
            }
            wayland_protocols::xdg::shell::server::xdg_toplevel::Request::SetAppId { app_id } => {
                log::info!("xdg_toplevel set app_id: {}", app_id);
            }
            wayland_protocols::xdg::shell::server::xdg_toplevel::Request::ShowWindowMenu {
                seat,
                serial,
                x,
                y,
            } => {
                log::info!("xdg_toplevel show window menu at ({}, {})", x, y);
            }
            wayland_protocols::xdg::shell::server::xdg_toplevel::Request::Move { seat, serial } => {
                log::info!("xdg_toplevel move requested");
            }
            wayland_protocols::xdg::shell::server::xdg_toplevel::Request::Resize {
                seat,
                serial,
                edges,
            } => {
                log::info!("xdg_toplevel resize requested on edges: {:?}", edges);
            }
            wayland_protocols::xdg::shell::server::xdg_toplevel::Request::SetMaxSize {
                width,
                height,
            } => {
                log::info!("xdg_toplevel set max size to ({}, {})", width, height);
            }
            wayland_protocols::xdg::shell::server::xdg_toplevel::Request::SetMinSize {
                width,
                height,
            } => {
                log::info!("xdg_toplevel set min size to ({}, {})", width, height);
            }
            wayland_protocols::xdg::shell::server::xdg_toplevel::Request::SetMaximized => {
                log::info!("xdg_toplevel maximized");
            }
            wayland_protocols::xdg::shell::server::xdg_toplevel::Request::UnsetMaximized => {
                log::info!("xdg_toplevel unmaximized");
            }
            wayland_protocols::xdg::shell::server::xdg_toplevel::Request::SetFullscreen {
                output,
            } => {
                log::info!("xdg_toplevel set fullscreen on output: {:?}", output);
            }
            wayland_protocols::xdg::shell::server::xdg_toplevel::Request::UnsetFullscreen => {
                log::info!("xdg_toplevel unset fullscreen");
            }
            wayland_protocols::xdg::shell::server::xdg_toplevel::Request::SetMinimized => {
                log::info!("xdg_toplevel minimized");
            }
            _ => todo!(),
        }
    }
}