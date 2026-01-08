use std::{cell::OnceCell, collections::HashMap, sync::OnceLock};

use wayland_protocols::xdg::shell::server::{
    xdg_surface::{Request, XdgSurface},
    xdg_toplevel::XdgToplevel,
    xdg_wm_base::XdgWmBase,
};
use wayland_server::{
    Dispatch, GlobalDispatch, New, Resource,
    backend::ObjectId,
    protocol::{
        wl_buffer::WlBuffer,
        wl_compositor::WlCompositor,
        wl_shm::{Format, WlShm},
        wl_shm_pool::WlShmPool,
        wl_surface::WlSurface,
    },
};

use crate::wayland::shm::ShmState;

pub(crate) mod callback;
pub(crate) mod compositor;
pub(crate) mod globals;
pub(crate) mod region;
pub(crate) mod shm;
pub(crate) mod surface;
pub(crate) mod xdg;

pub struct DisplayState {
    pub surfaces: HashMap<ObjectId, surface::SurfaceState>,
    pub(crate) shm: ShmState,
}

// pub struct ObjRef<R: Resource + 'static> {
//     pub id: OnceLock<ObjectId>,
//     _marker: std::marker::PhantomData<R>,
// }

// impl<R: Resource + 'static + Send + Sync> ObjRef<R> {
//     pub fn create<D>(new: New<R>, data_init: &mut wayland_server::DataInit<'_, D>)
//     where
//         D: Dispatch<R, Self>,
//     {
//         let resource = data_init
//             .init(
//                 new,
//                 Self {
//                     id: OnceLock::new(),
//                     _marker: std::marker::PhantomData,
//                 },
//             );
//         let rf = resource.data::<Self>().unwrap();
//         rf.id.set(resource.id()).unwrap();
//     }

//     pub fn id(&self) -> &ObjectId {
//         &self.id.get().unwrap()
//     }
// }

impl DisplayState {
    pub fn new() -> Self {
        DisplayState {
            surfaces: HashMap::new(),
            shm: ShmState::new(),
        }
    }

    pub fn get_surface_state_mut(
        &mut self,
        surface_id: ObjectId,
    ) -> &mut surface::SurfaceState {
        self.surfaces
            .entry(surface_id.clone())
            .or_insert_with(|| surface::SurfaceState::new())
    }

    pub fn get_surface_state(
        &self,
        surface_id: ObjectId,
    ) -> &surface::SurfaceState {
        self.surfaces
            .get(&surface_id)
            .expect("SurfaceState not found")
    }
}