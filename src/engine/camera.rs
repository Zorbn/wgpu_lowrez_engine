use crate::engine::texture;
use cgmath::Matrix4;
use wgpu::util::DeviceExt;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

#[derive(Copy, Clone)]
pub struct CameraHandle(pub usize);

pub struct Camera {
    pub viewpoint: ViewPoint,
    buffer: wgpu::Buffer,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    depth_texture: texture::Texture,
    width: u32,
    height: u32,
    fixed_size: bool,
}

impl Camera {
    pub fn new(
        device: &wgpu::Device,
        pos: cgmath::Point3<f32>,
        target: cgmath::Point3<f32>,
        up: cgmath::Vector3<f32>,
        projection: Box<dyn Projection>,
        screen_width: u32,
        screen_height: u32,
        fixed_size: bool,
    ) -> Self {
        let viewpoint = ViewPoint {
            pos,
            target,
            up,
            projection,
        };

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[get_uniform(&viewpoint, screen_width, screen_height)]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("camera_bind_group_layout"),
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let depth_texture = texture::Texture::create_depth_texture(
            &device,
            screen_width,
            screen_height,
            "depth_texture",
        );

        Self {
            viewpoint,
            buffer,
            bind_group,
            depth_texture,
            bind_group_layout,
            width: screen_width,
            height: screen_height,
            fixed_size,
        }
    }

    pub fn update(&mut self, queue: &mut wgpu::Queue, screen_width: u32, screen_height: u32) {
        let (new_width, new_height) = self.get_new_size(screen_width, screen_height);

        let camera_uniform = get_uniform(&self.viewpoint, new_width, new_height);
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[camera_uniform]));
    }

    pub fn resize(&mut self, device: &wgpu::Device, screen_width: u32, screen_height: u32) {
        let (new_width, new_height) = self.get_new_size(screen_width, screen_height);

        self.depth_texture =
            texture::Texture::create_depth_texture(&device, new_width, new_height, "depth_texture");
    }

    fn get_new_size(&self, screen_width: u32, screen_height: u32) -> (u32, u32) {
        let new_width = if self.fixed_size {
            self.width
        } else {
            screen_width
        };

        let new_height = if self.fixed_size {
            self.height
        } else {
            screen_height
        };

        (new_width, new_height)
    }

    pub fn pan(&mut self, x_dist: f32, y_dist: f32, z_dist: f32) {
        self.viewpoint.pos.x += x_dist;
        self.viewpoint.pos.y += y_dist;
        self.viewpoint.pos.z += z_dist;
        self.viewpoint.target.x += x_dist;
        self.viewpoint.target.y += y_dist;
        self.viewpoint.target.z += z_dist;
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn depth_texture(&self) -> &texture::Texture {
        &self.depth_texture
    }
}

pub struct ViewPoint {
    pub pos: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub projection: Box<dyn Projection>,
}

impl ViewPoint {
    fn build_view_projection_matrix(&self, aspect: f32) -> Matrix4<f32> {
        let view = Matrix4::look_at_rh(self.pos, self.target, self.up);

        OPENGL_TO_WGPU_MATRIX * self.projection.to_matrix(aspect) * view
    }
}

pub trait Projection {
    fn to_matrix(&self, aspect: f32) -> Matrix4<f32>;
}

pub struct PerspectiveProjection {
    pub fov_y: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Projection for PerspectiveProjection {
    fn to_matrix(&self, aspect: f32) -> Matrix4<f32> {
        cgmath::perspective(cgmath::Deg(self.fov_y), aspect, self.z_near, self.z_far)
    }
}

pub struct OrthographicProjection {
    pub width: f32,
    pub height: f32,
    pub fixed_aspect_ratio: bool,
    pub z_near: f32,
    pub z_far: f32,
}

impl Projection for OrthographicProjection {
    fn to_matrix(&self, aspect: f32) -> Matrix4<f32> {
        let width_multiplier = if self.fixed_aspect_ratio || aspect < 1.0 {
            1.0
        } else {
            aspect
        };
        let height_multiplier = if self.fixed_aspect_ratio || aspect > 1.0 {
            1.0
        } else {
            1.0 / aspect
        };

        cgmath::ortho(
            -self.width * 0.5 * width_multiplier,
            self.width * 0.5 * width_multiplier,
            -self.height * 0.5 * height_multiplier,
            self.height * 0.5 * height_multiplier,
            self.z_near,
            self.z_far,
        )
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniform {
    view_proj: [[f32; 4]; 4],
}

impl Uniform {
    fn from_viewpoint(viewpoint: &ViewPoint, aspect: f32) -> Uniform {
        Self {
            view_proj: viewpoint.build_view_projection_matrix(aspect).into(),
        }
    }
}

fn get_aspect(screen_width: u32, screen_height: u32) -> f32 {
    screen_width as f32 / screen_height as f32
}

fn get_uniform(viewpoint: &ViewPoint, screen_width: u32, screen_height: u32) -> Uniform {
    let aspect = get_aspect(screen_width, screen_height);
    Uniform::from_viewpoint(viewpoint, aspect)
}
