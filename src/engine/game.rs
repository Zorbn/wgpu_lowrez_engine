use crate::engine::{engine_handle, input, render_handle};

pub trait Game {
    fn start(&mut self, _handle: &mut engine_handle::EngineHandle) {}
    fn update(
        &mut self,
        _input: &input::Input,
        _handle: &mut engine_handle::EngineHandle,
        _delta_time: f32,
    ) {
    }
    fn fixed_update(&mut self, _input: &input::Input, _handle: &mut engine_handle::EngineHandle) {}
    fn render(&mut self, _handle: &mut render_handle::RenderHandle) {}
    fn get_fixed_update_rate(&self) -> u32 {
        60
    }
    fn get_name(&self) -> String {
        "Game".into()
    }
}
