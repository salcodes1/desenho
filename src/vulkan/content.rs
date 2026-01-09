use std::sync::Arc;

use vulkano::{
    buffer::{Buffer, Subbuffer},
    descriptor_set::{
        DescriptorSet,
        layout::{
            DescriptorSetLayout, DescriptorSetLayoutBinding, DescriptorSetLayoutCreateInfo,
            DescriptorType,
        },
    },
    device::Device,
    format::Format,
    image::{
        Image, ImageCreateInfo,
        sampler::{Sampler, SamplerCreateInfo},
        view::ImageView,
    },
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter}, pipeline::{GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo, graphics::{GraphicsPipelineCreateInfo, color_blend::{AttachmentBlend, ColorBlendAttachmentState, ColorBlendState}, input_assembly::InputAssemblyState, multisample::MultisampleState, rasterization::RasterizationState, subpass::PipelineSubpassType, vertex_input::VertexInputState, viewport::{Scissor, Viewport, ViewportState}}, layout::PipelineLayoutCreateInfo}, render_pass::Subpass,
};

use crate::vulkan::{VulkanRenderer, shaders};

pub(crate) struct SurfaceContent {
    pub renderer: Arc<VulkanRenderer>,
    pub image: Arc<Image>,
    pub view: Arc<ImageView>,
    pub sampler: Arc<Sampler>,
    pub descriptor_set: Arc<DescriptorSet>,
    pub descriptor_set_layout: Arc<DescriptorSetLayout>,
    pub staging_buffer: Subbuffer<[u8]>,
    pub pipeline: Arc<GraphicsPipeline>,
}

impl SurfaceContent {
    pub fn new_from_wl_surface(
        renderer: Arc<VulkanRenderer>,
        extent: [u32; 2],
        wl_format: wayland_server::protocol::wl_shm::Format,
    ) -> Arc<Self> {
        let format = match wl_format {
            wayland_server::protocol::wl_shm::Format::Argb8888 => Format::B8G8R8A8_SRGB,
            wayland_server::protocol::wl_shm::Format::Xrgb8888 => Format::B8G8R8A8_SRGB,
            _ => Format::B8G8R8A8_SRGB,
        };

        let image = Image::new(
            renderer.general_memory_allocator.clone(),
            ImageCreateInfo {
                image_type: vulkano::image::ImageType::Dim2d,
                format: format,
                extent: [extent[0], extent[1], 1],
                usage: vulkano::image::ImageUsage::SAMPLED | vulkano::image::ImageUsage::TRANSFER_DST,
                ..Default::default()
            },
            AllocationCreateInfo {
                ..Default::default()
            },
        )
        .unwrap();

        let view = ImageView::new_default(image.clone()).unwrap();

        let sampler = Sampler::new(
            renderer.device.clone(),
            SamplerCreateInfo {
                mag_filter: vulkano::image::sampler::Filter::Linear,
                min_filter: vulkano::image::sampler::Filter::Linear,
                mipmap_mode: vulkano::image::sampler::SamplerMipmapMode::Linear,
                address_mode: [vulkano::image::sampler::SamplerAddressMode::ClampToEdge; 3],
                ..Default::default()
            },
        )
        .unwrap();

        let descriptor_set_layout = DescriptorSetLayout::new(
            renderer.device.clone(),
            DescriptorSetLayoutCreateInfo {
                bindings: [(
                    0,
                    DescriptorSetLayoutBinding {
                        stages: vulkano::shader::ShaderStages::FRAGMENT,
                        ..DescriptorSetLayoutBinding::descriptor_type(
                            DescriptorType::CombinedImageSampler,
                        )
                    },
                )]
                .into_iter()
                .collect(),
                ..Default::default()
            },
        )
        .unwrap();

        let descriptor_set = DescriptorSet::new(
            renderer.descriptor_set_allocator.clone(),
            descriptor_set_layout.clone(),
            [
                vulkano::descriptor_set::WriteDescriptorSet::image_view_sampler(
                    0,
                    view.clone(),
                    sampler.clone(),
                ),
            ]
            .into_iter(),
            [],
        )
        .unwrap();

        let staging_buffer = Buffer::new_slice(
            renderer.general_memory_allocator.clone(),
            vulkano::buffer::BufferCreateInfo {
                size: 0,
                usage: vulkano::buffer::BufferUsage::TRANSFER_SRC,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            (extent[0] * extent[1] * 4) as u64,
        )
        .unwrap();

        let vs = shaders::vertex::load(renderer.device.clone()).unwrap();
        let fs = shaders::fragment::load(renderer.device.clone()).unwrap();

        let layout = PipelineLayout::new(
            renderer.device.clone(),
            PipelineLayoutCreateInfo {
                set_layouts: vec![descriptor_set_layout.clone()],
                push_constant_ranges: vec![],
                ..Default::default()
            },
        )
        .unwrap();

        let pipeline = GraphicsPipeline::new(
            renderer.device.clone(),
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
                        extent: [extent[0] as f32, extent[1] as f32],
                        depth_range: 0.0..=1.0,
                    }]
                    .into(),
                    scissors: vec![Scissor {
                        offset: [0, 0],
                        extent: [extent[0], extent[1]],
                    }]
                    .into(),
                    ..Default::default()
                }),
                rasterization_state: Some(RasterizationState::default()),
                multisample_state: Some(MultisampleState::default()),
                color_blend_state: Some(ColorBlendState {
                    attachments: vec![ColorBlendAttachmentState {
                        blend: Some(AttachmentBlend {
                            src_color_blend_factor: vulkano::pipeline::graphics::color_blend::BlendFactor::One,
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
                    Subpass::from(renderer.render_pass.clone(), 0).unwrap(),
                )),
                ..GraphicsPipelineCreateInfo::layout(layout)
            },
        )
        .unwrap();

        Arc::new(Self {
            renderer,
            image,
            view,
            sampler,
            descriptor_set,
            descriptor_set_layout,
            staging_buffer,
            pipeline,
        })
    }
}
