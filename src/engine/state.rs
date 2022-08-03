use crate::engine::{
    camera, engine_handle, game, input, model, render_handle, texture,
};

macro_rules! engine_handle {
    ($sel:ident) => {{
        let State {
            device,
            queue,
            config,
            cameras,
            ..
        } = $sel;

        let handle = engine_handle::EngineHandle::new(
            device, queue, config, cameras,
        );
        handle
    }};
}

pub struct State {
    game: Box<dyn game::Game>,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    input_handler: input::Input,
    fixed_input_handler: input::Input,
    cameras: Vec<camera::Camera>,
    models: Vec<model::Model>,
    textures: Vec<texture::Texture>,
    pipelines: Vec<wgpu::RenderPipeline>,
}

impl State {
    pub async fn new(window: &winit::window::Window, game: Box<dyn game::Game>) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Adapter request was not successful!");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .expect("Device request was not successful!");

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        Self {
            game,
            surface,
            device,
            queue,
            config,
            size,
            input_handler: input::Input::new(),
            fixed_input_handler: input::Input::new(),
            cameras: Vec::new(),
            models: Vec::new(),
            textures: Vec::new(),
            pipelines: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        let mut handle = engine_handle!(self);

        self.game.start(&mut handle);
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;

            for camera in &mut self.cameras {
                camera.resize(&self.device, self.config.width, self.config.height);
            }

            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn reconfigure_surface(&mut self) {
        self.resize(self.size);
    }

    pub fn input(&mut self, event: &winit::event::WindowEvent) {
        match event {
            winit::event::WindowEvent::KeyboardInput {
                input:
                    winit::event::KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                self.input_handler.key_state_changed(*keycode, *state);
                self.fixed_input_handler.key_state_changed(*keycode, *state);
            }
            _ => {}
        }
    }

    pub fn fixed_update(&mut self) {
        let mut handle = engine_handle!(self);

        self.game
            .fixed_update(&self.fixed_input_handler, &mut handle);
        self.fixed_input_handler.update();
    }

    pub fn update(&mut self, delta_time: f32) {
        let mut handle = engine_handle!(self);

        self.game
            .update(&self.input_handler, &mut handle, delta_time);

        let State { cameras, .. } = self;

        for camera in cameras {
            camera.update(&mut self.queue, self.config.width, self.config.height);
        }

        self.input_handler.update();
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut render_handle = render_handle::RenderHandle::new(&mut self.cameras, &view, &mut encoder);

        self.game.render(&mut render_handle);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
