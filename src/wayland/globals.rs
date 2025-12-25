use crate::wayland::*;

impl GlobalDispatch<WlCompositor, ()> for DisplayState {
    fn bind(
        state: &mut Self,
        handle: &wayland_server::DisplayHandle,
        client: &wayland_server::Client,
        resource: wayland_server::New<WlCompositor>,
        global_data: &(),
        data_init: &mut wayland_server::DataInit<'_, Self>,
    ) {
        data_init.init(resource, compositor::CompositorState {});
        log::info!("Bound wl_compositor for client {:?}", client.id());
    }
}

impl GlobalDispatch<WlShm, ()> for DisplayState {
    fn bind(
        state: &mut Self,
        handle: &wayland_server::DisplayHandle,
        client: &wayland_server::Client,
        resource: wayland_server::New<WlShm>,
        global_data: &(),
        data_init: &mut wayland_server::DataInit<'_, Self>,
    ) {
        let shm = data_init.init(resource, shm::ShmState {});

        shm.format(Format::Argb8888);
        shm.format(Format::Xrgb8888);

        log::info!("Bound wl_shm for client {:?}", client.id());
    }
}

impl GlobalDispatch<XdgWmBase, ()> for DisplayState {
    fn bind(
        state: &mut Self,
        handle: &wayland_server::DisplayHandle,
        client: &wayland_server::Client,
        resource: wayland_server::New<XdgWmBase>,
        global_data: &(),
        data_init: &mut wayland_server::DataInit<'_, Self>,
    ) {
        data_init.init(resource, xdg::xdg_wm_base::XdgWmBaseState {});
        log::info!("Bound xdg_wm_base for client {:?}", client.id());
    }
}