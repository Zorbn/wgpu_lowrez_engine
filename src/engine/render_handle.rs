use crate::engine::{camera, resource_handles, texture};

pub struct RenderHandle<'a> {
    cameras: &'a mut Vec<camera::Camera>,
    view: &'a wgpu::TextureView,
    encoder: &'a mut wgpu::CommandEncoder,
}

impl<'a> RenderHandle<'a> {
    pub fn new(
        cameras: &'a mut Vec<camera::Camera>,
        view: &'a wgpu::TextureView,
        encoder: &'a mut wgpu::CommandEncoder,
    ) -> Self {
        Self {
            cameras,
            view,
            encoder,
        }
    }

    pub fn begin_render_pass<'b>(
        &'b mut self,
        camera_handle: resource_handles::CameraHandle,
        clear_color: wgpu::Color,
        set_target: Option<&'b texture::Texture>,
    ) -> wgpu::RenderPass {
        let view = match set_target {
            Some(t) => t.view(),
            _ => self.view,
        };

        let mut render_pass = self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(clear_color),
                    store: true,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.cameras[camera_handle.0].depth_texture().view(),
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });

        render_pass.set_pipeline(&self.cameras[camera_handle.0].pipeline());
        render_pass.set_bind_group(0, self.cameras[camera_handle.0].texture_bind_group(), &[]);
        render_pass.set_bind_group(1, self.cameras[camera_handle.0].bind_group(), &[]);

        render_pass
    }
}
