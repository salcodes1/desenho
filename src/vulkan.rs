pub(crate) mod buffer;
pub(crate) mod content;

use std::{collections::BTreeMap, sync::Arc, vec};

use vulkano::{
    VulkanLibrary, buffer::Subbuffer, command_buffer::{
        AutoCommandBufferBuilder, CopyBufferToImageInfo, PrimaryAutoCommandBuffer, RenderPassBeginInfo, SubpassBeginInfo, SubpassEndInfo, allocator::StandardCommandBufferAllocator
    }, descriptor_set::{
        allocator::{StandardDescriptorSetAllocator, StandardDescriptorSetAllocatorCreateInfo},
        layout::{
            DescriptorSetLayout, DescriptorSetLayoutBinding, DescriptorSetLayoutCreateInfo,
            DescriptorType,
        },
    }, device::{Device, DeviceCreateInfo, DeviceExtensions, DeviceFeatures, Queue, QueueCreateInfo}, instance::{Instance, InstanceCreateInfo, InstanceExtensions}, memory::allocator::StandardMemoryAllocator, pipeline::{
        GraphicsPipeline, Pipeline, PipelineLayout, PipelineShaderStageCreateInfo,
        graphics::{
            GraphicsPipelineCreateInfo,
            color_blend::{AttachmentBlend, ColorBlendAttachmentState, ColorBlendState},
            input_assembly::InputAssemblyState,
            multisample::MultisampleState,
            rasterization::RasterizationState,
            subpass::PipelineSubpassType,
            vertex_input::VertexInputState,
            viewport::{Scissor, Viewport, ViewportState},
        },
        layout::PipelineLayoutCreateInfo,
    }, render_pass::{
        AttachmentDescription, AttachmentReference, Framebuffer, FramebufferCreateInfo, RenderPass,
        RenderPassCreateInfo, Subpass, SubpassDescription,
    }, swapchain::Surface
};
use wayland_server::Resource;

use crate::{vulkan::content::SurfaceContent, wayland::{DisplayState, shm, surface::SurfaceState}};

pub(crate) struct VulkanRenderer {
    pub(crate) instance: Arc<Instance>,
    pub(crate) device: Arc<Device>,
    pub(crate) queue: Arc<Queue>,
    pub(crate) cmd_allocator: Arc<StandardCommandBufferAllocator>,
    pub(crate) descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>,
    pub(crate) general_memory_allocator: Arc<StandardMemoryAllocator>,
}

impl VulkanRenderer {
    pub fn new() -> Arc<Self> {
        let library = VulkanLibrary::new().unwrap();
        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                enabled_extensions: InstanceExtensions {
                    khr_wayland_surface: true,
                    khr_surface: true,
                    // khr_display: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .unwrap();

        let mut physical_devices = instance.enumerate_physical_devices().unwrap();
        let physical_device = physical_devices.next().expect("No device available");

        log::info!(
            "Using device: {} (type: {:?})",
            physical_device.properties().device_name,
            physical_device.properties().device_type,
        );
        let (device, queue) = match Device::new(
            physical_device,
            DeviceCreateInfo {
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index: 0,
                    ..Default::default()
                }],
                enabled_extensions: DeviceExtensions {
                    khr_swapchain: true,
                    // khr_display_swapchain: true,
                    ..Default::default()
                },
                enabled_features: DeviceFeatures {
                    ..Default::default()
                },
                ..Default::default()
            },
        ) {
            Ok((dev, mut q)) => (dev, q.next().unwrap()),
            Err(e) => panic!("Failed to create device: {}", e),
        };

        let cmd_allocator = Arc::new(StandardCommandBufferAllocator::new(
            device.clone(),
            Default::default(),
        ));

        let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
            device.clone(),
            StandardDescriptorSetAllocatorCreateInfo::default(),
        ));

        let general_memory_allocator =
            Arc::new(StandardMemoryAllocator::new_default(device.clone()));
        Arc::new(VulkanRenderer {
            instance,
            device,
            queue,
            cmd_allocator,
            descriptor_set_allocator,
            general_memory_allocator,
        })
    }

    fn make_pipeline(
        &self,
        render_pass: Arc<RenderPass>,
        content: Arc<SurfaceContent>,
    ) -> Arc<GraphicsPipeline> {
        let vs = shaders::vertex::load(self.device.clone()).unwrap();
        let fs = shaders::fragment::load(self.device.clone()).unwrap();

        let layout = PipelineLayout::new(
            self.device.clone(),
            PipelineLayoutCreateInfo {
                set_layouts: vec![content.descriptor_set_layout.clone()],
                push_constant_ranges: vec![],
                ..Default::default()
            },
        )
        .unwrap();

        let pipeline = GraphicsPipeline::new(
            self.device.clone(),
            None,
            GraphicsPipelineCreateInfo {
                stages: vec![
                    PipelineShaderStageCreateInfo::new(vs.entry_point("main").unwrap()),
                    PipelineShaderStageCreateInfo::new(fs.entry_point("main").unwrap()),
                ]
                .into(),
                vertex_input_state: Some(VertexInputState::new()),
                input_assembly_state: Some(InputAssemblyState::default()),
                viewport_state: Some(ViewportState {
                    viewports: vec![Viewport {
                        offset: [0.0, 0.0],
                        extent: [800.0, 600.0],
                        depth_range: 0.0..=1.0,
                    }]
                    .into(),
                    scissors: vec![Scissor {
                        offset: [0, 0],
                        extent: [800, 600],
                    }]
                    .into(),
                    ..Default::default()
                }),
                rasterization_state: Some(RasterizationState::default()),
                multisample_state: Some(MultisampleState::default()),
                color_blend_state: Some(ColorBlendState {
                    attachments: vec![ColorBlendAttachmentState {
                        blend: Some(AttachmentBlend {
                            src_color_blend_factor: vulkano::pipeline::graphics::color_blend::BlendFactor::SrcAlpha,
                            dst_color_blend_factor: vulkano::pipeline::graphics::color_blend::BlendFactor::OneMinusSrcAlpha,
                            color_blend_op: vulkano::pipeline::graphics::color_blend::BlendOp::Add,
                            src_alpha_blend_factor: vulkano::pipeline::graphics::color_blend::BlendFactor::One,
                            dst_alpha_blend_factor: vulkano::pipeline::graphics::color_blend::BlendFactor::Zero,
                            alpha_blend_op: vulkano::pipeline::graphics::color_blend::BlendOp::Add,
                        }),
                        ..Default::default()
                    }],
                    ..Default::default()
                }),
                depth_stencil_state: None,
                subpass: Some(PipelineSubpassType::BeginRenderPass(
                    Subpass::from(render_pass, 0).unwrap(),
                )),
                ..GraphicsPipelineCreateInfo::layout(layout)
            },
        )
        .unwrap();

        pipeline
    }

    pub fn render_output_surfaces_commands<'a>(
        self: &Arc<Self>,
        target_view: Arc<vulkano::image::view::ImageView>,
        surfaces: impl IntoIterator<Item = &'a SurfaceState>,
        state: &DisplayState,
    ) -> Arc<PrimaryAutoCommandBuffer> {
        let mut builder = AutoCommandBufferBuilder::primary(
            self.cmd_allocator.clone(),
            self.queue.queue_family_index(),
            vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        let render_pass = RenderPass::new(
            self.device.clone(),
            RenderPassCreateInfo {
                attachments: vec![AttachmentDescription {
                    format: vulkano::format::Format::B8G8R8A8_SRGB,
                    samples: vulkano::image::SampleCount::Sample1,
                    load_op: vulkano::render_pass::AttachmentLoadOp::Clear,
                    store_op: vulkano::render_pass::AttachmentStoreOp::Store,
                    initial_layout: vulkano::image::ImageLayout::ColorAttachmentOptimal,
                    final_layout: vulkano::image::ImageLayout::PresentSrc,
                    ..Default::default()
                }],
                subpasses: vec![SubpassDescription {
                    color_attachments: vec![Some(AttachmentReference {
                        attachment: 0,
                        layout: vulkano::image::ImageLayout::General,
                        ..Default::default()
                    })],
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .unwrap();

        let fb = Framebuffer::new(
            render_pass.clone(),
            FramebufferCreateInfo {
                attachments: vec![target_view],
                ..Default::default()
            },
        )
        .unwrap();

        let mut contents = vec![];
        for surface in surfaces {
            if surface.current_buffer.is_none() {
                continue;
            }
            let buffer = surface.current_buffer.as_ref().unwrap().data::<shm::BufferState>().unwrap();
            let pool = state.shm.pools.get(&buffer.pool).unwrap();
            let offset = buffer.offset as isize;
            let width = buffer.width as isize;
            let height = buffer.height as isize;
            let stride = buffer.stride as isize;
            let data_ptr = unsafe { pool.loc.offset(offset) as *const u8 };
            let format = buffer.format;
            let content = SurfaceContent::new_from_wl_surface(
                self.clone(),
                [width as u32, height as u32],
                wayland_server::protocol::wl_shm::Format::Argb8888,
            );
            // Copy buffer data into staging buffer
            let staging_buffer = &content.staging_buffer;
            {
                let mut staging_content = staging_buffer.write().unwrap();
                for y in 0..height {
                    for x in 0..width {
                        let pixel_offset = y * stride + x * 4;
                        let pixel_ptr = unsafe { data_ptr.offset(pixel_offset) };
                        let pixel = unsafe {std::slice::from_raw_parts(pixel_ptr, 4)};
                        let (r, g, b, a) = match format {
                            wayland_server::WEnum::Value(wayland_server::protocol::wl_shm::Format::Argb8888) => (pixel[0], pixel[1], pixel[2], pixel[3]),
                            wayland_server::WEnum::Value(wayland_server::protocol::wl_shm::Format::Xrgb8888) => (pixel[1], pixel[2], pixel[3], 255),
                            _ => (0, 0, 0, 0), // Unsupported format
                        };
                        let dst_offset = (y * width + x) * 4;
                        staging_content[dst_offset as usize] = r;
                        staging_content[dst_offset as usize + 1] = g;
                        staging_content[dst_offset as usize + 2] = b;
                        staging_content[dst_offset as usize + 3] = a;
                    }
                }
            }
            surface.current_buffer.as_ref().unwrap().release();

            // Copy from staging buffer to image
            builder
                .copy_buffer_to_image(CopyBufferToImageInfo {
                    ..CopyBufferToImageInfo::buffer_image(staging_buffer.clone(), content.image.clone())
                }).unwrap();

            contents.push(content);
            if let Some(callback) = &surface.frame_callback {
                callback.done(0);
            }
        }


        builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![Some([0.9, 0.9, 0.9, 1.0].into())],
                    ..RenderPassBeginInfo::framebuffer(fb)
                },
                SubpassBeginInfo {
                    contents: vulkano::command_buffer::SubpassContents::Inline,
                    ..Default::default()
                },
            )
            .unwrap();

        for content in contents {
            let pipeline = self.make_pipeline(render_pass.clone(), content.clone());

            unsafe {
                builder
                    .bind_pipeline_graphics(pipeline.clone())
                    .unwrap()
                    .bind_descriptor_sets(
                        vulkano::pipeline::PipelineBindPoint::Graphics,
                        pipeline.layout().clone(),
                        0,
                        content.descriptor_set.clone(),
                    )
                    .unwrap()
                    .draw(6, 1, 0, 0)
                    .unwrap()
            };
        }

        builder.end_render_pass(SubpassEndInfo::default()).unwrap();

        builder.build().unwrap()
    }
}

mod shaders {
    pub(crate) mod vertex {
        vulkano_shaders::shader! {
                ty: "vertex",
                src: r#"
                #version 450

                layout(location = 0) out vec2 v_uv;

                void main() {
                    // Fullscreen triangle positions in NDC
                    const vec2 positions[3] = vec2[](
                        vec2(-1.0, -1.0),
                        vec2( 3.0, -1.0),
                        vec2(-1.0,  3.0)
                    );

                    vec2 pos = positions[gl_VertexIndex];
                    gl_Position = vec4(pos, 0.0, 1.0);

                    // Map from NDC (-1..1) to UV (0..1)
                    v_uv = pos * 0.5 + 0.5;
                }
                "#,
        }
    }

    pub(crate) mod fragment {
        vulkano_shaders::shader! {
                ty: "fragment",
                src: r#"
                #version 450

                layout(set = 0, binding = 0) uniform sampler2D surface_tex;

                layout(location = 0) in vec2 v_uv;
                layout(location = 0) out vec4 out_color;

                void main() {
                    out_color = texture(surface_tex, v_uv);
                }
                "#,
        }
    }
}
