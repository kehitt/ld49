use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::renderer::{
    sprite_pipeline::{SpriteBinds, SpritePipelineGlobals},
    texture::Texture,
};

use super::{
    sprite_pipeline::{Instance, SpriteBindGroup, SpritePipeline, Vertex},
    Renderer,
};

const SPRITE_VERTICES: &[Vertex] = &[
    Vertex {
        position: [1.0, 1.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [-1.0, 1.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [-1.0, -1.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [-1.0, -1.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
];

const OPENGL_TO_WGPU_MATRIX: glam::Mat4 = glam::const_mat4!(
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 0.5, 0.0],
    [0.0, 0.0, 0.5, 1.0]
);

pub struct SpriteRenderer {
    renderer: Renderer,

    sprite_pipeline: SpritePipeline,
    sprite_bind_group: SpriteBindGroup,

    vertex_buffer: wgpu::Buffer,

    globals: SpritePipelineGlobals,
    #[allow(dead_code)]
    globals_buffer: wgpu::Buffer,

    instances: Vec<Instance>,
}

impl SpriteRenderer {
    pub async fn new(window: &Window, sprite_atlas_data: &[u8], sprite_size: (u32, u32)) -> Self {
        let renderer = Renderer::new(window).await;

        let shader_module = renderer
            .device
            .create_shader_module(&wgpu::include_wgsl!("shader/instanced_sprite.wgsl"));
        let sprite_binds = SpriteBinds::new(&renderer.device);
        let sprite_pipeline = SpritePipeline::new(
            &renderer.device,
            &renderer.config,
            &shader_module,
            &sprite_binds,
        );

        let sprite_atlas = Texture::from_bytes(
            &renderer.device,
            &renderer.queue,
            sprite_atlas_data,
            "spritesheet.png",
        )
        .unwrap();

        let mat = Self::calc_ortho_matrix(renderer.size);
        let (sprite_width, sprite_height) = sprite_size;
        let (atlas_width, atlas_height) = sprite_atlas.get_dimensions();
        let globals = SpritePipelineGlobals {
            view_proj_matrix: mat.to_cols_array_2d(),
            sprite_size: [sprite_width, sprite_height],
            sprite_sheet_size: [atlas_width, atlas_height],
        };

        let globals_buffer =
            renderer
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Globals Buffer"),
                    contents: bytemuck::cast_slice(&[globals]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

        let sprite_bind_group =
            sprite_binds.bind_data(&renderer.device, &sprite_atlas, &globals_buffer);

        let vertex_buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(SPRITE_VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let instances = Vec::<Instance>::new();

        Self {
            renderer,
            sprite_pipeline,
            sprite_bind_group,
            vertex_buffer,
            globals,
            globals_buffer,
            instances,
        }
    }

    pub fn on_surface_lost(&mut self) {
        self.renderer.on_surface_lost();
    }

    pub fn on_resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.on_resize(new_size);

        let mat = Self::calc_ortho_matrix(new_size);
        self.globals.view_proj_matrix = mat.to_cols_array_2d();

        self.renderer.queue.write_buffer(
            &self.globals_buffer,
            0,
            bytemuck::cast_slice(&[self.globals]),
        )
    }

    pub fn add_sprite_instance(&mut self, sprite_idx: u32, model_matrix: [[f32; 4]; 4]) {
        self.instances.push(Instance {
            sprite_idx,
            model_matrix,
        })
    }

    pub fn draw_instances_or_panic(&mut self) {
        match self.draw_instances() {
            Ok(_) => (),
            Err(wgpu::SurfaceError::Lost) => self.on_surface_lost(),
            Err(e) => panic!("{:?}", e),
        }
    }

    fn draw_instances(&mut self) -> Result<(), wgpu::SurfaceError> {
        // @OPTIMIZATION: Recreating a buffer every frame is not the best idea
        let instance_buffer =
            self.renderer
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Instance Buffer"),
                    contents: bytemuck::cast_slice(&self.instances),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        let output = self.renderer.surface.get_current_frame()?.output;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder =
            self.renderer
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("SpriteRenderer command encoder"),
                });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Instanced sprite render pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.sprite_pipeline.pipeline);
            render_pass.set_bind_group(0, &self.sprite_bind_group.0, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
            render_pass.draw(0..6, 0..self.instances.len() as _);
        }
        self.renderer
            .queue
            .submit(std::iter::once(encoder.finish()));

        self.instances.clear();
        Ok(())
    }

    fn calc_ortho_matrix(window_size: winit::dpi::PhysicalSize<u32>) -> glam::Mat4 {
        let ratio = f64::from(window_size.width) / f64::from(window_size.height);
        OPENGL_TO_WGPU_MATRIX
            * glam::Mat4::orthographic_lh(
                -10.0 * ratio as f32,
                10.0 * ratio as f32,
                -10.0,
                10.0,
                0.1,
                1000.0,
            )
    }
}
