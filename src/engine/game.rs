use crate::engine::{asset_handle, engine_handle, input, render_handle};

pub trait Game {
    fn start(&mut self, handle: &mut engine_handle::EngineHandle);
    fn update(
        &mut self,
        input: &input::Input,
        handle: &mut engine_handle::EngineHandle,
        delta_time: f32,
    );
    fn fixed_update(&mut self, input: &input::Input, handle: &mut engine_handle::EngineHandle);
    fn render<'a>(
        &mut self,
        render_handle: &'a mut render_handle::RenderHandle,
        asset_handle: &'a asset_handle::AssetHandle,
    );
    fn get_fixed_update_rate(&self) -> u32;
}
