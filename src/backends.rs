pub(crate) mod surface;
pub(crate) mod winit;

use std::sync::Arc;
use vulkano::{image::view::ImageView, sync::GpuFuture};

pub struct AcquiredFrame {
    pub view: Arc<ImageView>,
    pub index: usize,

    /// Must be waited on / chained before touching `view`.
    pub ready: Box<dyn GpuFuture>,

    /// Given a future that has finished rendering into `view`,
    /// schedule presentation and return a future that completes
    /// when the frame is fully done.
    pub present: Box<dyn FnOnce(Box<dyn GpuFuture>) -> Box<dyn GpuFuture> + Send>,
    pub extent: [u32; 2],
}

pub trait PresentBackend {
    fn acquire_frame(&mut self) -> anyhow::Result<AcquiredFrame>;
}