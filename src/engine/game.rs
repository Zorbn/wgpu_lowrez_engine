use crate::engine::{engine_handle, input, render_handle};

pub trait Game {
    fn start(&mut self, handle: &mut engine_handle::EngineHandle);
    fn update(
        &mut self,
        input: &input::Input,
        handle: &mut engine_handle::EngineHandle,
        delta_time: f32,
    );
    fn fixed_update(&mut self, input: &input::Input, handle: &mut engine_handle::EngineHandle);
    fn render(&mut self, handle: &mut render_handle::RenderHandle);
    fn get_fixed_update_rate(&self) -> u32 {
        60
    }
    fn get_name(&self) -> String {
        "Game".into()
    }
}
