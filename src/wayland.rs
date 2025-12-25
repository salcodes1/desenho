use wayland_protocols::xdg::shell::server::{
    xdg_surface::{Request, XdgSurface},
    xdg_toplevel::XdgToplevel,
    xdg_wm_base::XdgWmBase,
};
use wayland_server::{
    Dispatch, GlobalDispatch,
    protocol::{
        wl_buffer::WlBuffer,
        wl_compositor::WlCompositor,
        wl_shm::{Format, WlShm},
        wl_shm_pool::WlShmPool,
        wl_surface::WlSurface,
    },
};

pub struct State {}

pub struct CompositorState {}

pub struct SurfaceState {}

pub struct ShmState {}

pub struct ShmPoolState {}

impl GlobalDispatch<WlCompositor, ()> for State {
    fn bind(
        state: &mut Self,
        handle: &wayland_server::DisplayHandle,
        client: &wayland_server::Client,
        resource: wayland_server::New<WlCompositor>,
        global_data: &(),
        data_init: &mut wayland_server::DataInit<'_, Self>,
    ) {
        data_init.init(resource, CompositorState {});
        log::info!("Bound wl_compositor for client {:?}", client.id());
    }
}

impl GlobalDispatch<WlShm, ()> for State {
    fn bind(
        state: &mut Self,
        handle: &wayland_server::DisplayHandle,
        client: &wayland_server::Client,
        resource: wayland_server::New<WlShm>,
        global_data: &(),
        data_init: &mut wayland_server::DataInit<'_, Self>,
    ) {
        let shm = data_init.init(resource, ShmState {});

        shm.format(Format::Argb8888);
        shm.format(Format::Xrgb8888);

        log::info!("Bound wl_shm for client {:?}", client.id());
    }
}

impl GlobalDispatch<XdgWmBase, ()> for State {
    fn bind(
        state: &mut Self,
        handle: &wayland_server::DisplayHandle,
        client: &wayland_server::Client,
        resource: wayland_server::New<XdgWmBase>,
        global_data: &(),
        data_init: &mut wayland_server::DataInit<'_, Self>,
    ) {
        data_init.init(resource, XdgWmBaseState {});
        log::info!("Bound xdg_wm_base for client {:?}", client.id());
    }
}

struct XdgWmBaseState;
impl Dispatch<XdgWmBase, XdgWmBaseState> for State {
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
            wayland_protocols::xdg::shell::server::xdg_wm_base::Request::Destroy => todo!(),
            wayland_protocols::xdg::shell::server::xdg_wm_base::Request::CreatePositioner {
                id,
            } => todo!(),
            wayland_protocols::xdg::shell::server::xdg_wm_base::Request::GetXdgSurface {
                id,
                surface,
            } => {
                // 🎯 Here you'd normally initialize the xdg_surface
                data_init.init(id, XdgSurfaceState {});
            }
            wayland_protocols::xdg::shell::server::xdg_wm_base::Request::Pong { serial } => todo!(),
            _ => todo!(),
        }
    }
}

struct XdgSurfaceState;
impl Dispatch<XdgSurface, XdgSurfaceState> for State {
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
                let toplevel = data_init.init(id, XdgToplevelState {});

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
            quest::AckConfigure { serial } => {
                log::info!("xdg_surface acked configure with serial {}", serial);
            },
            _ => todo!(),
        }
    }
}

struct XdgToplevelState;
impl Dispatch<XdgToplevel, XdgToplevelState> for State {
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

impl Dispatch<WlCompositor, CompositorState> for State {
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
                data_init.init(id, SurfaceState {});
            }
            wayland_server::protocol::wl_compositor::Request::CreateRegion { id } => {
                // can leave empty for now
            }
            _ => {
                log::warn!("Unhandled wl_compositor request: {:?}", request);
            }
        }
    }
}

impl Dispatch<WlSurface, SurfaceState> for State {
    fn request(
        state: &mut Self,
        client: &wayland_server::Client,
        resource: &WlSurface,
        request: wayland_server::protocol::wl_surface::Request,
        data: &SurfaceState,
        dhandle: &wayland_server::DisplayHandle,
        data_init: &mut wayland_server::DataInit<'_, Self>,
    ) {
        // can leave empty for now

        log::warn!("Unhandled wl_surface request");
    }
}

impl Dispatch<WlShm, ShmState> for State {
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

impl Dispatch<WlShmPool, ShmPoolState> for State {
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

struct BufferState;
impl Dispatch<WlBuffer, BufferState> for State {
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
