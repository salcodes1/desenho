use std::{sync::Arc, vec};

use vulkano::{
    VulkanLibrary,
    command_buffer::{
        AutoCommandBufferBuilder, PrimaryAutoCommandBuffer, RenderPassBeginInfo, SubpassBeginInfo,
        allocator::StandardCommandBufferAllocator,
    },
    device::{Device, DeviceCreateInfo, DeviceExtensions, DeviceFeatures, Queue, QueueCreateInfo},
    instance::{Instance, InstanceCreateInfo, InstanceExtensions},
    memory::allocator::StandardMemoryAllocator,
    pipeline::{
        GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo,
        graphics::GraphicsPipelineCreateInfo, layout::PipelineLayoutCreateInfo,
    },
    render_pass::{
        AttachmentDescription, AttachmentReference, Framebuffer, FramebufferCreateInfo, RenderPass,
        RenderPassCreateInfo, Subpass, SubpassDescription,
    },
};

pub(crate) struct VulkanRenderer {
    pub(crate) instance: Arc<Instance>,
    pub(crate) device: Arc<Device>,
    pub(crate) queue: Arc<Queue>,
    pub(crate) cmd_allocator: Arc<StandardCommandBufferAllocator>,
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
        Arc::new(VulkanRenderer {
            instance,
            device,
            queue,
            cmd_allocator,
        })
    }

    fn make_pipeline(&self) {
        let vs = shaders::vertex::load(self.device.clone()).unwrap();
        let fs = shaders::fragment::load(self.device.clone()).unwrap();

        let descriptor_set = PersistentDescriptorSet;

        let layout = PipelineLayout::new(
            self.device.clone(),
            PipelineLayoutCreateInfo {
                set_layouts: todo!(),
                push_constant_ranges: todo!(),
                ..Default::default()
            },
        )
        .unwrap();


        let pipeline = GraphicsPipeline::new(
            self.device.clone(),
            None,
            GraphicsPipelineCreateInfo::layout(layout),
        )
        .unwrap();
    }

    fn render_output_surfaces_commands(
        &self,
        target_view: Arc<vulkano::image::view::ImageView>,
        target_extent: [u32; 2],
    ) -> Arc<PrimaryAutoCommandBuffer> {
        let mut builder = AutoCommandBufferBuilder::primary(
            self.cmd_allocator.clone(),
            self.queue.queue_family_index(),
            vulkano::command_buffer::CommandBufferUsage::MultipleSubmit,
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

        builder
            .begin_render_pass(
                RenderPassBeginInfo::framebuffer(fb),
                SubpassBeginInfo {
                    contents: vulkano::command_buffer::SubpassContents::Inline,
                    ..Default::default()
                },
            )
            .unwrap()
            .builder
            .build()
            .unwrap()
    }
}

mod shaders {
    pub(crate) mod vertex {
        vulkano_shaders::shader! {
                ty: "vertex",
                src: r#"
                #version 450

                layout(location = 0) in vec2 in_pos;    // [-1, 1] NDC or screen-space converted
                layout(location = 1) in vec2 in_uv;

                layout(push_constant) uniform Push {
                    mat3 surface_to_ndc;   // or something simpler like rect + scale
                } pc;

                layout(location = 0) out vec2 v_uv;

                void main() {
                    vec3 p = pc.surface_to_ndc * vec3(in_pos, 1.0);
                    gl_Position = vec4(p.xy, 0.0, 1.0);
                    v_uv = in_uv;
                }
                "#,
        }
    }

    pub(crate) mod fragment {
        vulkano_shaders::shader! {
                ty: "fragment",
                src: r#"
                #version 450

                layout(set = 0, binding = 0) uniform sampler2D tex;
                layout(push_constant) uniform Push {
                    mat3 surface_to_ndc;
                    float opacity;
                } pc;

                layout(location = 0) in vec2 v_uv;
                layout(location = 0) out vec4 out_color;

                void main() {
                    vec4 c = texture(tex, v_uv);
                    out_color = vec4(c.rgb, c.a * pc.opacity);
                }
                "#,
        }
    }
}
