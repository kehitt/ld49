use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::renderer::{
    background_pipeline::{BackgroundBinds, BackgroundPipeline, BackgroundPipelineGlobals},
    sprite_pipeline::{SpriteBinds, SpritePipelineGlobals},
    texture::Texture,
};

use super::{
    background_pipeline::BackgroundBindGroup,
    sprite_pipeline::{Instance, SpriteBindGroup, SpritePipeline, Vertex},
    RenderDevice,
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

pub struct Renderer {
    renderer: RenderDevice,

    sprite_pipeline: SpritePipeline,
    sprite_bind_group: SpriteBindGroup,

    background_pipeline: BackgroundPipeline,
    background_bind_group: BackgroundBindGroup,

    vertex_buffer: wgpu::Buffer,

    sprite_globals: SpritePipelineGlobals,
    #[allow(dead_code)]
    sprite_globals_buffer: wgpu::Buffer,

    background_globals: BackgroundPipelineGlobals,
    #[allow(dead_code)]
    background_globals_buffer: wgpu::Buffer,

    instances: Vec<Instance>,
}

impl Renderer {
    pub async fn new(
        window: &Window,
        sprite_atlas_data: &[u8],
        sprite_size: (u32, u32),
        background_atlas_data: &[u8],
        background_size: (u32, u32),
    ) -> Self {
        let renderer = RenderDevice::new(window).await;

        let sprite_shader = renderer
            .device
            .create_shader_module(&wgpu::include_wgsl!("shader/instanced_sprite.wgsl"));
        let sprite_binds = SpriteBinds::new(&renderer.device);
        let sprite_pipeline = SpritePipeline::new(
            &renderer.device,
            &renderer.config,
            &sprite_shader,
            &sprite_binds,
        );

        let background_shader = renderer
            .device
            .create_shader_module(&wgpu::include_wgsl!("shader/background.wgsl"));
        let background_binds = BackgroundBinds::new(&renderer.device);
        let background_pipeline = BackgroundPipeline::new(
            &renderer.device,
            &renderer.config,
            &background_shader,
            &background_binds,
        );

        let sprite_atlas = Texture::from_bytes(
            &renderer.device,
            &renderer.queue,
            sprite_atlas_data,
            "spritesheet.png",
        )
        .unwrap();

        let background_atlas = Texture::from_bytes(
            &renderer.device,
            &renderer.queue,
            background_atlas_data,
            "background_spritesheet.png",
        )
        .unwrap();

        let mat = Self::calc_ortho_matrix(renderer.size);
        let (sprite_width, sprite_height) = sprite_size;
        let (atlas_width, atlas_height) = sprite_atlas.get_dimensions();
        let sprite_globals = SpritePipelineGlobals {
            view_proj_matrix: mat.to_cols_array_2d(),
            sprite_size: [sprite_width, sprite_height],
            sprite_sheet_size: [atlas_width, atlas_height],
        };

        let sprite_globals_buffer =
            renderer
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Sprite Globals Buffer"),
                    contents: bytemuck::cast_slice(&[sprite_globals]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

        let sprite_bind_group =
            sprite_binds.bind_data(&renderer.device, &sprite_atlas, &sprite_globals_buffer);

        let (sprite_width, sprite_height) = background_size;
        let (atlas_width, atlas_height) = background_atlas.get_dimensions();
        let background_globals = BackgroundPipelineGlobals {
            view_proj_matrix: mat.to_cols_array_2d(),
            sprite_idx: 0,
            sprite_size: [sprite_width, sprite_height],
            sprite_sheet_size: [atlas_width, atlas_height],
            player_health: 1.0,
        };

        let background_globals_buffer =
            renderer
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Background Globals Buffer"),
                    contents: bytemuck::cast_slice(&[background_globals]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

        let background_bind_group = background_binds.bind_data(
            &renderer.device,
            &background_atlas,
            &background_globals_buffer,
        );

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
            background_pipeline,
            background_bind_group,
            vertex_buffer,
            sprite_globals,
            sprite_globals_buffer,
            background_globals,
            background_globals_buffer,
            instances,
        }
    }

    pub fn on_surface_lost(&mut self) {
        self.renderer.on_surface_lost();
    }

    pub fn on_resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.on_resize(new_size);

        let mat = Self::calc_ortho_matrix(new_size);
        self.sprite_globals.view_proj_matrix = mat.to_cols_array_2d();

        self.renderer.queue.write_buffer(
            &self.sprite_globals_buffer,
            0,
            bytemuck::cast_slice(&[self.sprite_globals]),
        )
    }

    pub fn set_background_state(&mut self, sprite_idx: u32, player_health: f32) {
        self.background_globals.player_health = player_health;
        self.background_globals.sprite_idx = sprite_idx;
        self.renderer.queue.write_buffer(
            &self.background_globals_buffer,
            0,
            bytemuck::cast_slice(&[self.background_globals]),
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
                    label: Some("Renderer command encoder"),
                });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Background render pass"),
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

            render_pass.set_pipeline(&self.background_pipeline.pipeline);
            render_pass.set_bind_group(0, &self.background_bind_group.0, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..6, 0..1);
        }

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Instanced sprite render pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
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
        // Screen coordinates are pixel coordinates, recentered to 0,0 being the center of the window
        let half_width = window_size.width / 2;
        let half_height = window_size.height / 2;
        OPENGL_TO_WGPU_MATRIX
            * glam::Mat4::orthographic_lh(
                -(half_width as f32),
                half_width as f32,
                -(half_height as f32),
                half_height as f32,
                0.1,
                10.0,
            )
    }
}
