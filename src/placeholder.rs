use macroquad::Vec2;

use crate::*;

use hecs::{World, EntityBuilder};

pub struct Placeholder {
    pub pos: Vec2,
    pub half_e: Vec2,
    pub call_fn: fn(&mut KaGame, x: f32, y: f32),
}


pub fn add(game: &mut KaGame, x: f32, y: f32, w: f32, h: f32, call_fn: fn(&mut KaGame, x: f32, y: f32)) {
    let mut e = EntityBuilder::new();
    e.add(
        Placeholder {
            pos: Vec2::new(x, y),
            half_e: Vec2::new(w / 2.0, h / 2.0),
            call_fn,
        }
    );
    
    game.to_spawn(e);
}

pub fn system(world: &mut World, game: &mut KaGame, camera: &KaCamera) {
    
    for (e, placeholder) in world.query::<&Placeholder>().iter() {
        if KaAABB::overlaps(&placeholder.pos, &placeholder.half_e, &camera.get_pos(), &Vec2::new(camera.get_width() / 2.0, camera.get_height() / 2.0)) {
            (placeholder.call_fn)(game, placeholder.pos.x(), placeholder.pos.y());
            game.to_despawn(e);
        }
    }
}

