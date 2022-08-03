use crate::engine::{input, texture, vertex};
use cgmath::Matrix4;
use wgpu::util::DeviceExt;
use winit::event::VirtualKeyCode;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct Camera {
    pub viewpoint: ViewPoint,
    buffer: wgpu::Buffer,
    // bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    depth_texture: texture::Texture,
    // pipeline_layout: wgpu::PipelineLayout,
    pipeline: wgpu::RenderPipeline,
    texture_bind_group: wgpu::BindGroup,
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
        // TODO: Make the following into a separate function that adds a pipeline
        // TODO: to the list of pipelines in this camera.
        shader_res_path: &str,
        format: wgpu::TextureFormat,
        texture: &texture::Texture,
        // END TODO
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

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(
                std::fs::read_to_string(format!("res/{}", shader_res_path))
                    .unwrap()
                    .into(),
            ),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[texture.bind_group_layout().unwrap(), &bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[vertex::Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            viewpoint,
            buffer,
            bind_group,
            depth_texture,
            pipeline,
            texture_bind_group: texture.create_bind_group(device),
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

    pub fn pan(&mut self, input: &input::Input, speed: f32) {
        let mut dir_x = 0;

        if input.is_key_held(VirtualKeyCode::Left) {
            dir_x = -1;
        }

        if input.is_key_held(VirtualKeyCode::Right) {
            dir_x = 1;
        }

        self.viewpoint.pos.x += dir_x as f32 * speed;
        self.viewpoint.target.x += dir_x as f32 * speed;
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    // pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
    //     &self.bind_group_layout
    // }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn texture_bind_group(&self) -> &wgpu::BindGroup {
        &self.texture_bind_group
    }

    pub fn depth_texture(&self) -> &texture::Texture {
        &self.depth_texture
    }

    pub fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }

    // pub fn pipeline_layout(&self) -> &wgpu::PipelineLayout {
    //     &self.pipeline_layout
    // }
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
