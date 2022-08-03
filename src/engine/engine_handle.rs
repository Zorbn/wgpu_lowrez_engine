use crate::engine::{camera, model, resource_handles, texture, vertex};

pub struct EngineHandle<'a> {
    device: &'a mut wgpu::Device,
    queue: &'a mut wgpu::Queue,
    config: &'a mut wgpu::SurfaceConfiguration,
    cameras: &'a mut Vec<camera::Camera>,
    models: &'a mut Vec<model::Model>,
    textures: &'a mut Vec<texture::Texture>,
}

impl<'a> EngineHandle<'a> {
    pub fn new(
        device: &'a mut wgpu::Device,
        queue: &'a mut wgpu::Queue,
        config: &'a mut wgpu::SurfaceConfiguration,
        cameras: &'a mut Vec<camera::Camera>,
        models: &'a mut Vec<model::Model>,
        textures: &'a mut Vec<texture::Texture>,
    ) -> Self {
        Self {
            device,
            queue,
            config,
            cameras,
            models,
            textures,
        }
    }

    pub fn create_camera(
        &mut self,
        pos: cgmath::Point3<f32>,
        target: cgmath::Point3<f32>,
        up: cgmath::Vector3<f32>,
        projection: Box<dyn camera::Projection>,
        texture: resource_handles::TextureHandle,
        set_width: Option<u32>,
        set_height: Option<u32>,
    ) -> resource_handles::CameraHandle {
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
            "shader.wgsl",
            self.config.format,
            &self.textures[texture.0],
            width,
            height,
            fixed_size,
        );

        self.cameras.push(camera);

        resource_handles::CameraHandle(new_cam_index)
    }

    pub fn create_model(
        &mut self,
        vertices: &[vertex::Vertex],
        indices: &[u16],
    ) -> resource_handles::ModelHandle {
        let new_model_index = self.models.len();

        let model = model::Model::new(self.device, vertices, indices);
        self.models.push(model);

        resource_handles::ModelHandle(new_model_index)
    }

    pub fn create_texture(
        &mut self,
        width: u32,
        height: u32,
        format: wgpu::TextureFormat,
        extra_usages: wgpu::TextureUsages,
        label: Option<&str>,
    ) -> resource_handles::TextureHandle {
        let new_tex_index = self.textures.len();

        let texture = texture::Texture::from_dimensions(
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
        ));

        self.textures.push(texture);

        resource_handles::TextureHandle(new_tex_index)
    }

    pub fn load_texture(&mut self, res_path: &str) -> resource_handles::TextureHandle {
        let new_tex_index = self.textures.len();

        let texture = texture::Texture::from_path(self.device, self.queue, res_path)
            .expect(&format!("Failed to load texture from path: {}", res_path));
        self.textures.push(texture);

        resource_handles::TextureHandle(new_tex_index)
    }

    pub fn get_camera(&mut self, handle: resource_handles::CameraHandle) -> &mut camera::Camera {
        &mut self.cameras[handle.0]
    }
}
