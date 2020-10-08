use macroquad::{Texture2D, FilterMode};
use hecs::{World, EntityBuilder, Entity};
use quad_snd::{mixer::SoundMixer};
use crate::{Resources, Spawner, Despawner, Actions, ActionInput, ka_aabb_update};

use quad_snd::{mixer::SoundId};

#[derive(Debug, Clone)]
pub struct NoSoundError;


pub struct KaGame {
    resources: Resources,
    spawner: Spawner,
    despawner: Despawner,
    actions: Actions,
    pub mixer: SoundMixer,

}

impl KaGame {
    pub fn new() -> Self {
        Self {
            resources: Resources::new(),
            spawner: Spawner::new(),
            despawner: Despawner::new(),
            actions: Actions::new(),
            mixer: SoundMixer::new(),
        }
    }

    pub fn update(&mut self, world: &mut World) {
        ka_aabb_update(world);
    }

    pub fn update_actions(&mut self) {
        self.actions.update();
    }

    pub fn set_texture_filter_mode(&mut self, filter: FilterMode) {
        self.resources.set_texture_filter_mode(filter);
    }

    pub async fn load_texture(&mut self, path: &str) {
        self.resources.load_texture(path).await;
    }

    /*
    pub fn load_texture_from_bytes(&mut self, name: &str, bytes: &[u8]) {
        self.resources.load_texture_from_bytes(name, bytes);
    }
    */

    pub fn get_texture(&self, path: &str) -> Option<&Texture2D> {
        self.resources.get_texture(path)
    }

    
    pub fn load_ogg(&mut self, name: &str, data: &[u8]) {
        self.resources.load_ogg(name, data);
    }

    pub fn play_sound(&mut self, name: &str) -> Result<SoundId, NoSoundError>{
        match self.resources.get_sound(name) {
            Some(sound) => {
                Ok(self.mixer.play(sound.clone()))
            },
            None => Err(NoSoundError),
        }
    }
    

    pub fn to_spawn(&mut self, builder: EntityBuilder) {
        self.spawner.add(builder);
    }

    pub fn to_despawn(&mut self, entity: Entity) {
        self.despawner.add(entity);
    }

    pub fn spawn(&mut self, world: &mut World) {
        self.spawner.spawn(world);
    }

    pub fn despawn(&mut self, world: &mut World) {
        self.despawner.despawn(world);
    }

    pub fn add_action(&mut self, name: &str, input: ActionInput) {
        self.actions.add_action(name, input);
    }

    pub fn is_action_down(&self, name: &str) -> bool {
        self.actions.is_action_down(name)
    }

    pub fn is_action_just_pressed(&self, name: &str) -> bool {
        self.actions.is_action_just_pressed(name)
    }

    pub fn is_action_released(&self, name: &str) -> bool {
        self.actions.is_action_released(name)
    }

    pub fn is_action_just_released(&self, name: &str) -> bool {
        self.actions.is_action_just_released(name)
    }

}