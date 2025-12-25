use std::{
    os::{fd::OwnedFd, raw::c_void},
    ptr::null,
    sync::Arc,
};

use wayland_server::WEnum;

use crate::wayland::*;

pub struct ShmState {
    pub(crate) pools: HashMap<ObjectId, ShmPoolState>,
}

impl ShmState {
    pub fn new() -> Self {
        ShmState {
            pools: HashMap::new(),
        }
    }
}

pub struct ShmPoolState {
    fd: OwnedFd,
    pub(crate) loc: *mut c_void,
    size: i32,
}

unsafe impl Send for ShmPoolState {}
unsafe impl Sync for ShmPoolState {}

impl ShmPoolState {
    pub fn new(fd: OwnedFd, size: i32) -> Self {
        unsafe {
            let loc = rustix::mm::mmap(
                core::ptr::null_mut(),
                size as usize,
                rustix::mm::ProtFlags::READ | rustix::mm::ProtFlags::WRITE,
                rustix::mm::MapFlags::SHARED,
                &fd,
                0,
            )
            .unwrap();
            ShmPoolState { fd, size, loc }
        }
    }

    pub fn resize(&mut self, size: i32) {
        unsafe {
            self.loc = rustix::mm::mremap(
                self.loc,
                self.size as usize,
                size as usize,
                rustix::mm::MremapFlags::MAYMOVE,
            )
            .unwrap();
            self.size = size;
        }
    }
}

pub(crate) struct BufferState {
    pub(crate) offset: i32,
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) stride: i32,
    pub(crate) format: WEnum<Format>,
    pub(crate) pool: ObjectId,
}

impl Dispatch<WlShm, ()> for DisplayState {
    fn request(
        state: &mut Self,
        client: &wayland_server::Client,
        resource: &WlShm,
        request: wayland_server::protocol::wl_shm::Request,
        data: &(),
        dhandle: &wayland_server::DisplayHandle,
        data_init: &mut wayland_server::DataInit<'_, Self>,
    ) {
        match request {
            wayland_server::protocol::wl_shm::Request::CreatePool { id, fd, size } => {
                let shm_pool_resource = data_init.init(id, ());
                state.shm.pools.insert(
                    shm_pool_resource.id(),
                    ShmPoolState::new(fd, size),
                );
            }
            _ => {}
        }
    }
}

impl Dispatch<WlShmPool, ()> for DisplayState {
    fn request(
        state: &mut Self,
        client: &wayland_server::Client,
        resource: &WlShmPool,
        request: <WlShmPool as wayland_server::Resource>::Request,
        data: &(),
        dhandle: &wayland_server::DisplayHandle,
        data_init: &mut wayland_server::DataInit<'_, Self>,
    ) {
        let mut pool_state = state.shm.pools.get_mut(&resource.id()).unwrap();
        match request {
            wayland_server::protocol::wl_shm_pool::Request::CreateBuffer {
                id,
                offset,
                width,
                height,
                stride,
                format,
            } => {
                data_init.init(
                    id,
                    BufferState {
                        offset,
                        width,
                        height,
                        stride,
                        format,
                        pool: resource.id(),
                    },
                );
            }

            wayland_server::protocol::wl_shm_pool::Request::Resize { size } => {
                pool_state.resize(size);
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
