use std::sync::Arc;

use vulkano::{image::{Image, ImageUsage, view::{ImageView, ImageViewCreateInfo}}, swapchain::{self, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo}, sync::GpuFuture};

use crate::vulkan::VulkanRenderer;

pub(crate) struct SurfacePresent {
    renderer: Arc<VulkanRenderer>,

    surface: Arc<Surface>,
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<Image>>,

    swapchain_create_info: SwapchainCreateInfo,
}

impl SurfacePresent {
    pub fn new(
        renderer: Arc<VulkanRenderer>,
        surface: Arc<Surface>,
    ) -> Self {
        let swapchain_create_info = SwapchainCreateInfo {
            min_image_count: 3,
            image_format: vulkano::format::Format::B8G8R8A8_SRGB,
            image_usage: ImageUsage::COLOR_ATTACHMENT,
            present_mode: vulkano::swapchain::PresentMode::Mailbox,
            image_extent: [800, 600], // TODO: change
            ..Default::default()
        };

        let (swapchain, images) = Swapchain::new(
            renderer.device.clone(),
            surface.clone(),
            swapchain_create_info.clone()
        )
        .unwrap();

        SurfacePresent {
            renderer,
            surface,
            swapchain,
            images,
            swapchain_create_info,
        }
    }

    pub fn set_extent(&mut self, width: u32, height: u32) {
        self.swapchain_create_info.image_extent = [width, height];
        let (new_swapchain, new_images) = self.swapchain.recreate(self.swapchain_create_info.clone()).unwrap();
        self.swapchain = new_swapchain;
        self.images = new_images;
    }
}

impl crate::backends::PresentBackend for SurfacePresent {
    fn acquire_frame(&mut self) -> anyhow::Result<crate::backends::AcquiredFrame> {
        // Acquire next image from the swapchain
        let (idx, _suboptimal, acq_future) =
            swapchain::acquire_next_image(self.swapchain.clone(), None)?;

        // Create an ImageView for this image
        let view = ImageView::new(
            self.images[idx as usize].clone(),
            ImageViewCreateInfo {
                format: self.swapchain.image_format(),
                subresource_range: vulkano::image::ImageSubresourceRange {
                    aspects: vulkano::image::ImageAspects::COLOR,
                    mip_levels: 0..1,
                    array_layers: 0..1,
                },
                ..Default::default()
            },
        )?;

        // Use the actual swapchain extent (in case it changed)
        let extent = self.swapchain.image_extent();

        // Ready future: now(device) join acquire_future
        let device = self.renderer.device.clone();
        let ready: Box<dyn GpuFuture> =
            Box::new(vulkano::sync::now(device).join(acq_future));

        // Clone everything we'll need for presentation
        let queue = self.renderer.queue.clone();
        let swapchain = self.swapchain.clone();
        let image_index = idx;

        let present= Box::new(move |fut: Box<dyn GpuFuture>| {
            let fut = fut
                .then_swapchain_present(
                    queue,
                    SwapchainPresentInfo::swapchain_image_index(swapchain, image_index),
                )
                .then_signal_fence_and_flush()
                .expect("failed to present swapchain image");
            Box::new(fut) as Box<dyn GpuFuture>
        });

        Ok(crate::backends::AcquiredFrame {
            view,
            extent,
            index: idx as usize,
            ready,
            present,
        })
    }
}
