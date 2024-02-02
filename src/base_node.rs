use crate::{render_node::RenderNode, render_pipeline::create_render_pipeline};

pub struct BaseNode {
    pipeline: wgpu::RenderPipeline,
}

impl BaseNode {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let shader = wgpu::include_wgsl!("base_shader.wgsl");
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = create_render_pipeline(
            device,
            &pipeline_layout,
            config.format,
            &[],
            wgpu::PrimitiveTopology::TriangleList,
            shader,
            Some("Base::render_pipeline"),
        );
        Self { pipeline }
    }
}

impl RenderNode for BaseNode {
    fn process(&self, device: &wgpu::Device, output: &wgpu::TextureView, queue: &wgpu::Queue) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Base::pass"),
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
        pass.draw(0..3, 0..1);

        drop(pass);
        queue.submit(Some(encoder.finish()));
    }
}
