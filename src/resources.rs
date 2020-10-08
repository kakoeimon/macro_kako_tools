use std::collections::HashMap;
use macroquad::{Texture2D, load_texture, set_texture_filter, FilterMode};

use quad_snd::{
    decoder::{read_ogg},
    mixer::Sound
};


pub struct Resources {
    texture_filter_mode: FilterMode,
    textures: HashMap<String, Texture2D>,
    sounds: HashMap<String, Sound>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            texture_filter_mode: FilterMode::Linear,
            textures: HashMap::with_capacity(100),
            sounds: HashMap::with_capacity(100),
            
        }
    }


    pub fn set_texture_filter_mode(&mut self, filter: FilterMode) {
        self.texture_filter_mode = filter;
    }
    
    pub async fn load_texture(&mut self, path: &str) {
        let texture = load_texture(&path).await;
        set_texture_filter(texture, self.texture_filter_mode);
        self.textures.insert(path.to_owned(), texture);
    }
    /* This requires a modification in the source of macroquad to work.
    pub fn load_texture_from_bytes(&mut self, name: &str, bytes: &[u8]) {
        use macroquad::load_texture_from_bytes;
        let texture = load_texture_from_bytes(bytes);
        set_texture_filter(texture, self.texture_filter_mode);
        self.textures.insert(name.to_owned(), texture);
    }
    */

    pub fn get_texture(&self, path: &str)-> Option<&Texture2D> {
        self.textures.get(path)
    }

    
    pub fn load_ogg(&mut self, name: &str, data: &[u8]) {
        let sound = read_ogg(data).unwrap();
        self.sounds.insert(name.to_owned(), sound);
    }

    pub fn get_sound(&self, name: &str) -> Option<&Sound> {
        self.sounds.get(name)
    }
    
}