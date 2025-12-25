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


mod globals;
mod compositor;
mod region;
mod shm;
mod surface;
mod callback;
mod xdg;

pub struct DisplayState {}




