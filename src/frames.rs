use std::ops::Range;
use std::collections::HashMap;
use macroquad::{Vec2, Texture2D, get_frame_time, Rect};
use hecs::{World, Entity};
use crate::KaGame;
use crate::KaSprite;



pub struct KaFrames {
    pub range: Range<usize>,
    pub frame_size: Vec2,
    pub number_of_frames: usize,
    pub frame_time: f32,
    pub timer: f32,
    pub current_frame: usize,
    pub replay: bool,
    pub end_function: fn(&mut World, &mut KaGame, Entity),
    pub frame_functions: HashMap<usize, fn(&mut World, &mut KaGame, Entity)>
}

impl KaFrames {
    pub fn new(texture: &Texture2D, frames: usize, range: Range<usize>, frame_time: f32, replay: bool, end_function: fn(&mut World, &mut KaGame, Entity)) -> Self {
        let frame_size = Vec2::new(texture.width() / frames as f32, texture.height());
        Self {
            
            frame_size,
            number_of_frames: frames,
            frame_time,
            timer: 0.0,
            current_frame: range.start,
            range,
            replay,
            end_function,
            frame_functions: HashMap::new(),
        }
    }

    pub fn get_frame_rect(&self, frame_number: usize) -> Option<Rect> {
        Some( Rect::new(frame_number as f32 * self.frame_size.x(), 0.0, self.frame_size.x(), self.frame_size.y()) )
    }

    pub fn get_current_frame_rect(&self) -> Option<Rect> {
        self.get_frame_rect(self.current_frame)
    }

    pub fn reset_end_fn(&mut self) {
        self.end_function = end_frame_null;
    }

    pub fn set_end_fn(&mut self, end_fn: fn(&mut World, &mut KaGame, Entity)) {
        self.end_function = end_fn;
    }

    pub fn set_range(&mut self, range: Range<usize>) {
        self.current_frame = range.start;
        self.range = range;
        self.reset_end_fn();
        self.timer = 0.0;
    }

    pub fn set_range_and_end_fn(&mut self, range: Range<usize>, end_fn: fn(&mut World, &mut KaGame, Entity)) {
        self.set_range(range);
        self.set_end_fn(end_fn);
    }

    pub fn is_playing_frames(&self, range: Range<usize>) -> bool {
        self.range == range
    }

}

pub fn end_frame_null(_world: &mut World, _game: &mut KaGame, _entity: Entity) {

}

pub fn ka_sprite_frames(world: &mut World, game: &mut KaGame) {
    let mut fn_to_call: Vec<(fn(&mut World, &mut KaGame, Entity), Entity)> = Vec::new();
    for (e, (mut sprite, mut frames)) in world.query::<(&mut KaSprite, &mut KaFrames)>().iter() {
        frames.timer += get_frame_time();
        if frames.timer >= frames.frame_time {
            frames.timer -= frames.frame_time;
            frames.current_frame += 1;
            //One game frame damage.
            if let Some(result) = frames.frame_functions.get(&frames.current_frame) {
                fn_to_call.push((*result, e.clone()));
            }
            
            
            if frames.current_frame > frames.range.end {
                if frames.replay{
                    frames.current_frame = frames.range.start;
                } else {
                    frames.current_frame = frames.range.end;
                }
                fn_to_call.push((frames.end_function, e.clone()));
            }
        }

        sprite.frame = frames.get_current_frame_rect();
    }
    
    for (func, e) in fn_to_call.iter() {
        (func)(world, game, e.clone());
    }
    
}
