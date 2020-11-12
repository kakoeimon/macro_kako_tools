use std::collections::HashMap;
use macroquad::prelude::{Texture2D, load_texture, set_texture_filter, FilterMode};

use quad_snd::{
    decoder::{read_ogg},
    mixer::{Sound, PlaybackStyle}
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
    
    pub async fn load_texture(&mut self, path: &str) -> Texture2D {
        let texture = load_texture(&path).await;
        set_texture_filter(texture, self.texture_filter_mode);
        self.textures.insert(path.to_owned(), texture);
        texture
    }

    /*
    pub fn load_texture_from_bytes(&mut self, name: &str, bytes: &[u8]) -> Texture2D{
        use macroquad::prelude::load_texture_from_bytes;
        let texture = load_texture_from_bytes(bytes);
        set_texture_filter(texture, self.texture_filter_mode);
        self.textures.insert(name.to_owned(), texture);
        texture
    }
    */


    pub fn get_texture(&self, path: &str)-> Option<&Texture2D> {
        self.textures.get(path)
    }

    
    pub fn load_ogg(&mut self, name: &str, data: &[u8], looped: bool) {
        let mut sound = read_ogg(data).unwrap();
        if looped {
            sound.playback_style = PlaybackStyle::Looped;
        }
        
        self.sounds.insert(name.to_owned(), sound);
    }

    pub fn get_sound(&self, name: &str) -> Option<&Sound> {
        self.sounds.get(name)
    }
    
}