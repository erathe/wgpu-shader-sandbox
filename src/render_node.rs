use crate::{base_node::BaseNode, texture};

pub trait RenderNode {
    fn process(&self, device: &wgpu::Device, output: &wgpu::TextureView, queue: &wgpu::Queue);
    fn set_bind_group(&mut self, _device: &wgpu::Device, _texture: &texture::Texture) {}
}

pub struct RenderGraph {
    nodes: Vec<Box<dyn RenderNode>>,
    textures: [texture::Texture; 2],
}

impl RenderGraph {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let base_node = BaseNode::new(&device, &config);
        let mut graph = Self {
            textures: [
                texture::Texture::create_2d_texture(
                    &device,
                    config.width,
                    config.height,
                    config.format,
                    wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
                    wgpu::FilterMode::Nearest,
                    Some("ping_texture"),
                ),
                texture::Texture::create_2d_texture(
                    &device,
                    config.width,
                    config.height,
                    config.format,
                    wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
                    wgpu::FilterMode::Nearest,
                    Some("pong_texture"),
                ),
            ],
            nodes: Vec::new(),
        };

        graph.add_node(&device, Box::new(base_node), false);
        graph
    }

    pub fn add_node(&mut self, device: &wgpu::Device, mut node: Box<dyn RenderNode>, set_bg: bool) {
        if set_bg {
            let len = self.nodes.len();
            node.set_bind_group(&device, &self.textures[(len - 1) % 2]);
        }
        self.nodes.push(node);
    }

    pub fn process<'a>(
        &'a self,
        device: &wgpu::Device,
        output: &wgpu::TextureView,
        queue: &wgpu::Queue,
    ) {
        let len = self.nodes.len();
        for (idx, node) in self.nodes.iter().enumerate() {
            let out = {
                if idx == len - 1 {
                    output
                } else {
                    &self.textures[idx % 2].view
                }
            };
            node.process(device, &out, queue);
        }
    }
}
