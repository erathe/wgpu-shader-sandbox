use crate::{
    render_node::RenderNode,
    render_pipeline::{
        create_basic_sampler_bind_group, create_basic_sampler_bind_group_layout,
        create_render_pipeline,
    },
    texture,
};

pub struct FractNode {
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: Option<wgpu::BindGroup>,
}

impl FractNode {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let bind_group_layout =
            create_basic_sampler_bind_group_layout(device, Some("Fract::bg_layout"));

        let shader = wgpu::include_wgsl!("fract_shader.wgsl");
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = create_render_pipeline(
            device,
            &pipeline_layout,
            config.format,
            &[],
            wgpu::PrimitiveTopology::TriangleList,
            shader,
            Some("Fract::render_pipeline"),
        );
        Self {
            pipeline,
            bind_group_layout,
            bind_group: None,
        }
    }
}

impl RenderNode for FractNode {
    fn process(&self, device: &wgpu::Device, output: &wgpu::TextureView, queue: &wgpu::Queue) {
        if let Some(bind_group) = &self.bind_group {
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Fract Node Encoder"),
            });
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Fract::pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &output,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.draw(0..3, 0..1);

            drop(pass);

            queue.submit(Some(encoder.finish()));
        }
    }

    fn set_bind_group(&mut self, device: &wgpu::Device, texture: &texture::Texture) {
        self.bind_group = Some(create_basic_sampler_bind_group(
            device,
            &self.bind_group_layout,
            &texture,
            Some("Fract::bg"),
        ));
    }
}
