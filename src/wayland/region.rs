use wayland_server::protocol::wl_region::{self, WlRegion};

use crate::wayland::*;

pub struct RegionState {}

impl Dispatch<WlRegion, RegionState> for DisplayState {
    fn request(
        state: &mut Self,
        client: &wayland_server::Client,
        resource: &WlRegion,
        request: wl_region::Request,
        data: &RegionState,
        dhandle: &wayland_server::DisplayHandle,
        data_init: &mut wayland_server::DataInit<'_, Self>,
    ) {
        match request {
            wl_region::Request::Destroy => {
                // Handle destroy if needed
            }
            wl_region::Request::Add {
                x,
                y,
                width,
                height,
            } => {
                // Handle adding a rectangle to the region
            }
            wl_region::Request::Subtract {
                x,
                y,
                width,
                height,
            } => {
                // Handle subtracting a rectangle from the region
            }
            _ => {
                log::warn!("Unhandled wl_region request: {:?}", request);
            }
        }
    }
}