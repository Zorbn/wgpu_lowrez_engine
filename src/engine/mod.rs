pub mod camera;
pub mod engine_handle;
pub mod game;
pub mod input;
pub mod model;
mod pipeline;
pub mod render_handle;
mod state;
pub mod texture;
pub mod texture_array;
pub mod vertex;

use std::time::Instant;
use winit::{
    dpi::LogicalPosition,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub fn start_game(game: Box<dyn game::Game>) {
    pollster::block_on(run(game));
}

async fn run(game: Box<dyn game::Game>) {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title(game.get_name()).build(&event_loop).unwrap();
    window.set_outer_position(get_new_window_position(&window));

    let fixed_update_rate = game.get_fixed_update_rate();
    let fixed_update_delta = 1.0 / fixed_update_rate as f32;

    let mut state = state::State::new(&window, game).await;

    let mut last_frame_time = Instant::now();
    let mut fixed_frame_accumulator = 0.0;
    let mut delta_time = 0.0;

    state.start();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            state.input(event);

            match event {
                WindowEvent::Resized(physical_size) => {
                    state.resize(*physical_size);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    state.resize(**new_inner_size);
                }
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => {}
            }
        }
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            let current_time = Instant::now();
            delta_time = (current_time - last_frame_time).as_secs_f32();
            last_frame_time = current_time;
            fixed_frame_accumulator += delta_time;

            while fixed_frame_accumulator >= fixed_update_delta {
                fixed_frame_accumulator -= fixed_update_delta;
                state.fixed_update();
            }

            state.update(delta_time);
            match state.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => state.reconfigure_surface(),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(wgpu::SurfaceError::Outdated) => {}
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}

fn get_new_window_position(window: &Window) -> LogicalPosition<u32> {
    let monitor_size = window.current_monitor().unwrap().size();
    let window_size = window.inner_size();
    let monitor_center = LogicalPosition::new(monitor_size.width / 2, monitor_size.height / 2);
    let window_center = LogicalPosition::new(window_size.width / 2, window_size.height / 2);

    let center_x = if window_center.x < monitor_center.x {
        monitor_center.x - window_center.x
    } else {
        0
    };
    let center_y = if window_center.y < monitor_center.y {
        monitor_center.y - window_center.y
    } else {
        0
    };

    LogicalPosition::new(center_x, center_y)
}
