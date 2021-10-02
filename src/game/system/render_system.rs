use specs::prelude::*;

use crate::{game::component::Transform, renderer::RendererState};

#[derive(Default)]
pub struct RenderSystem {
    pub renderer_state: Option<RendererState>,
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = ReadStorage<'a, Transform>;

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
    }

    fn run(&mut self, position: Self::SystemData) {
        if let Some(renderer_state) = &mut self.renderer_state {
            let data = position.join().collect::<Vec<_>>();
            match draw_frame(renderer_state, data) {
                Ok(_) => (),
                Err(wgpu::SurfaceError::Lost) => renderer_state.on_resize(renderer_state.size),
                Err(wgpu::SurfaceError::OutOfMemory) => panic!("Out of memory"),
                Err(e) => eprintln!("{:?}", e),
            }
        } else {
            panic!("No renderer state was created!")
        }
    }
}

// this is bad on many levels
fn draw_frame(
    renderer_state: &mut RendererState,
    data: Vec<&Transform>,
) -> Result<(), wgpu::SurfaceError> {
    let output = renderer_state.surface.get_current_frame()?.output;
    let view = output
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder =
        renderer_state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

    {
        let mut _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
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
    }

    {
        for tf in data {
            let staging_buffer = wgpu::util::DeviceExt::create_buffer_init(
                &renderer_state.device,
                &wgpu::util::BufferInitDescriptor {
                    label: Some("NEW ModelMat Buffer"),
                    contents: bytemuck::cast_slice(&tf.to_model_mat()),
                    usage: wgpu::BufferUsages::COPY_SRC,
                },
            );

            encoder.copy_buffer_to_buffer(
                &staging_buffer,
                0,
                &renderer_state.model_mat_buffer,
                0,
                std::mem::size_of::<glam::Mat4>() as wgpu::BufferAddress,
            );

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
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

            render_pass.set_pipeline(&renderer_state.render_pipeline);
            render_pass.set_vertex_buffer(0, renderer_state.vertex_buffer.slice(..));
            render_pass.set_bind_group(0, &renderer_state.model_mat_bind_group, &[]);
            render_pass.set_bind_group(1, &renderer_state.view_proj_bind_group, &[]);
            // I'm not going to worry about instancing for now
            render_pass.draw(0..6, 0..1);
        }
    }
    renderer_state
        .queue
        .submit(std::iter::once(encoder.finish()));
    Ok(())
}
