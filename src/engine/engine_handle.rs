use crate::engine::{camera, model, pipeline, texture, vertex};

pub struct EngineHandle<'a> {
    device: &'a mut wgpu::Device,
    queue: &'a mut wgpu::Queue,
    config: &'a mut wgpu::SurfaceConfiguration,
    cameras: &'a mut Vec<camera::Camera>,
}

impl<'a> EngineHandle<'a> {
    pub fn new(
        device: &'a mut wgpu::Device,
        queue: &'a mut wgpu::Queue,
        config: &'a mut wgpu::SurfaceConfiguration,
        cameras: &'a mut Vec<camera::Camera>,
    ) -> Self {
        Self {
            device,
            queue,
            config,
            cameras,
        }
    }

    pub fn create_camera(
        &mut self,
        pos: cgmath::Point3<f32>,
        target: cgmath::Point3<f32>,
        up: cgmath::Vector3<f32>,
        projection: Box<dyn camera::Projection>,
        set_width: Option<u32>,
        set_height: Option<u32>,
    ) -> camera::CameraHandle {
        let new_cam_index = self.cameras.len();

        let width = match set_width {
            Some(w) => w,
            _ => self.config.width,
        };
        let height = match set_height {
            Some(h) => h,
            _ => self.config.height,
        };

        let fixed_size = set_width.is_some() || set_height.is_some();

        let camera = camera::Camera::new(
            &self.device,
            pos,
            target,
            up,
            projection,
            width,
            height,
            fixed_size,
        );

        self.cameras.push(camera);

        camera::CameraHandle(new_cam_index)
    }

    pub fn create_model(
        &mut self,
        vertices: &[vertex::Vertex],
        indices: &[u16],
    ) -> model::Model {
        model::Model::new(self.device, vertices, indices)
    }

    pub fn create_texture(
        &mut self,
        width: u32,
        height: u32,
        format: wgpu::TextureFormat,
        extra_usages: wgpu::TextureUsages,
        label: Option<&str>,
    ) -> texture::Texture {
        texture::Texture::from_dimensions(
            self.device,
            width,
            height,
            format,
            extra_usages,
            label,
        )
        .expect(&format!(
            "Failed to create texture with label: {}",
            match label {
                Some(l) => l,
                _ => "[no label]",
            }
        ))
    }

    pub fn create_pipeline(
        &mut self,
        shader_res_path: &str,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
        camera_handle: Option<camera::CameraHandle>,
    ) -> wgpu::RenderPipeline {
        let mut layouts = bind_group_layouts.to_vec();

        if let Some(handle) = camera_handle {
            layouts.push(self.cameras[handle.0].bind_group_layout());
        }

        pipeline::create_pipeline(
            self.device,
            self.config.format,
            shader_res_path,
            layouts.as_slice(),
        )
    }

    pub fn load_texture(&mut self, res_path: &str) -> texture::Texture {
        texture::Texture::from_path(self.device, self.queue, res_path)
            .expect(&format!("Failed to load texture from path: {}", res_path))
    }

    pub fn get_camera(&mut self, handle: camera::CameraHandle) -> &mut camera::Camera {
        &mut self.cameras[handle.0]
    }
}
