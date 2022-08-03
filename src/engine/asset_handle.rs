use crate::engine::{model, resource_handles, state, texture};

pub struct AssetHandle<'a> {
    models: &'a mut Vec<model::Model>,
    textures: &'a mut Vec<texture::Texture>,
}

impl<'a> AssetHandle<'a> {
    pub fn new(models: &'a mut Vec<model::Model>, textures: &'a mut Vec<texture::Texture>) -> Self {
        Self { models, textures }
    }

    pub fn get_model(&self, handle: resource_handles::ModelHandle) -> &model::Model {
        &self.models[handle.0]
    }

    pub fn get_texture(&self, handle: resource_handles::TextureHandle) -> &texture::Texture {
        &self.textures[handle.0]
    }
}
