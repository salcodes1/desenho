use wayland_server::{WEnum, protocol::{wl_callback::WlCallback, wl_surface}};

use crate::wayland::*;

pub struct SurfaceState {
    pub current_buffer: Option<WlBuffer>,
    pub pending_buffer: Option<WlBuffer>,
    pub frame_callback: Option<WlCallback>,
}

impl SurfaceState {
    pub fn new() -> Self {
        SurfaceState {
            current_buffer: None,
            pending_buffer: None,
            frame_callback: None,
        }
    }

    pub fn frame(&mut self, callback: WlCallback) {
        self.frame_callback = Some(callback);
    }

    pub fn attach_buffer(&mut self, buffer: Option<WlBuffer>) {
        self.pending_buffer = buffer;
    }

    pub fn commit(&mut self) {
        self.current_buffer = self.pending_buffer.take();
    }

    pub fn save_buffer_png(&self, state: &DisplayState) {
        if let Some(buffer) = &self.current_buffer {
            let buffer = buffer.data::<shm::BufferState>().unwrap();
            let pool = state.shm.pools.get(&buffer.pool).unwrap();
            let offset = buffer.offset as isize;
            let width = buffer.width as isize;
            let height = buffer.height as isize;
            let stride = buffer.stride as isize;
            let data_ptr = unsafe { pool.loc.offset(offset) as *const u8 };
            let format = buffer.format;
            // Use the image crate to save the buffer as a PNG
            let mut img_buf = image::ImageBuffer::<image::Rgba<u8>, Vec<u8>>::new(buffer.width as u32, buffer.height as u32);
            for y in 0..height {
                for x in 0..width {
                    let pixel_offset = y * stride + x * 4;
                    let pixel_ptr = unsafe { data_ptr.offset(pixel_offset) };
                    let pixel = unsafe {std::slice::from_raw_parts(pixel_ptr, 4)};
                    let (r, g, b, a) = match format {
                        WEnum::Value(Format::Argb8888) => (pixel[0], pixel[1], pixel[2], pixel[3]),
                        WEnum::Value(Format::Xrgb8888) => (pixel[1], pixel[2], pixel[3], 255),
                        _ => (0, 0, 0, 0), // Unsupported format
                    };
                    img_buf.put_pixel(x as u32, y as u32, image::Rgba([r, g, b, a]));
                }
            }
            img_buf.save("output.png").unwrap();
            self.frame_callback.as_ref().map(|cb| cb.done(0));
        }
    }
}

impl Dispatch<WlSurface, ()> for DisplayState {
    fn request(
        state: &mut Self,
        client: &wayland_server::Client,
        resource: &WlSurface,
        request: wayland_server::protocol::wl_surface::Request,
        data: &(),
        dhandle: &wayland_server::DisplayHandle,
        data_init: &mut wayland_server::DataInit<'_, Self>,
    ) {
        match request {
            wl_surface::Request::Destroy => {
                log::warn!("wl_surface destroyed");
            },
            wl_surface::Request::Attach { buffer, x, y } => {
                log::warn!("wl_surface attach buffer: {:?} at ({}, {})", buffer, x, y);

                state.get_surface_state_mut(resource.id()).attach_buffer(buffer);
            },
            wl_surface::Request::Damage { x, y, width, height } => {
                log::warn!("wl_surface damage at ({}, {}) size ({}, {})", x, y, width, height);
            },
            wl_surface::Request::Frame { callback } => {
                log::warn!("wl_surface frame callback: {:?}", callback);
                let callback = data_init.init(callback, callback::CallbackState {});
                state.get_surface_state_mut(resource.id()).frame(callback);
            },
            wl_surface::Request::SetOpaqueRegion { region } => {
                log::warn!("wl_surface set opaque region: {:?}", region);
            },
            wl_surface::Request::SetInputRegion { region } => {
                log::warn!("wl_surface set input region: {:?}", region);
            },
            wl_surface::Request::Commit => {
                log::warn!("wl_surface commit");
                state.get_surface_state_mut(resource.id()).commit();
                state.get_surface_state(resource.id()).save_buffer_png(state);
            },
            wl_surface::Request::SetBufferTransform { transform } => {
                log::warn!("wl_surface set buffer transform: {:?}", transform);
            },
            wl_surface::Request::SetBufferScale { scale } => {
                log::warn!("wl_surface set buffer scale: {}", scale);
            },
            wl_surface::Request::DamageBuffer { x, y, width, height } => {
                log::warn!("wl_surface damage buffer at ({}, {}) size ({}, {})", x, y, width, height);
            },
            wl_surface::Request::Offset { x, y } => {
                log::warn!("wl_surface offset by ({}, {})", x, y);
            },
            _ => todo!(),
        }
    }
}