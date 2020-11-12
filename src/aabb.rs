use std::sync::{Mutex};

use macroquad::prelude::{Vec2};
use hecs::{World, Entity};

use crate::KaSprite;


const MARGIN: f32 = 0.01;



pub struct KaSensor{
    pub mask: i32,
    pub active: bool,
    pub overlapping: Vec<Entity>,
}

impl KaSensor {
    pub fn new() -> Self {
        Self {
            mask: 0,
            active: true,
            overlapping: Vec::new(),
        }
    }

    pub fn new_with_mask(mask: i32) -> Self {
        Self {
            mask,
            active: true,
            overlapping: Vec::new(),
        }
    }
}

pub struct KaMoveable {
    pub velocity: Vec2,
    pub on_wall: Option<Entity>,
    pub on_floor: Option<Entity>,
    pub on_ceilling: Option<Entity>,
    pub pushable: bool,
    pub external_forces: Vec2,
    //external_friction is used to lerp the external_forces to zero (it is multiplied with delta)
    pub external_friction: f32,
    pub slide: bool,
}

impl Default for KaMoveable {
    fn default() -> Self {
        Self {
            velocity: Vec2::zero(),
            on_wall: None,
            on_floor: None,
            on_ceilling: None,
            pushable: true,
            external_forces: Vec2::zero(),
            external_friction: 10.0,
            slide: true,
        }
    }
}


impl KaMoveable {
    pub fn new(velocity_x: f32, velocity_y: f32, pushable: bool) -> Self {
        Self {
            velocity: Vec2::new(velocity_x, velocity_y),
            pushable,
            ..Default::default()
        }
    }

    pub fn set_velocity(&mut self, x: f32, y: f32) {
        *self.velocity.x_mut() = x;
        *self.velocity.y_mut() = y;
    }

    pub fn get_collisions(&self) -> Vec<Entity> {
        let mut entities: Vec<Entity> = Vec::new();
        match self.on_floor {
            None => (),
            Some(e) => {entities.push(e.clone());}
        };

        match self.on_wall {
            None => (),
            Some(e) => {entities.push(e.clone());}
        };

        match self.on_ceilling {
            None => (),
            Some(e) => {entities.push(e.clone());}
        };

        entities
    }
    
}

pub struct KaAABB{
    pub pos: Mutex<Vec2>,
    pub half_e: Vec2,
    pub collision_layer: i32,
    pub collision_mask: i32,
    pub solid: bool,
    pub one_way: bool,
    pub collision_exceptions: Vec<u32>,
}

impl Default for KaAABB {
    fn default() -> Self {
        Self {
            pos: Mutex::new(Vec2::zero()),
            half_e: Vec2::zero(),
            collision_layer: 0,
            collision_mask: 0,
            solid: true,
            one_way: false,
            collision_exceptions: Vec::new(),

        }
    }
}


impl KaAABB {
    pub fn new(x: f32, y: f32, w: f32, h: f32, 
        collision_layer: i32, collision_mask: i32, solid: bool, one_way: bool
    ) -> Self 
    {
        Self {
            pos: Mutex::new(Vec2::new(x, y)),
            half_e: Vec2::new(w / 2.0, h / 2.0),
            collision_layer,
            collision_mask,
            solid,
            one_way,
            collision_exceptions: Vec::new(),
        }
    }

    pub fn overlaps(pos1: &Vec2, half_e1: &Vec2, pos2: &Vec2, half_e2: &Vec2) -> bool {
        if (pos1.x() - pos2.x()).abs() > half_e1.x() + half_e2.x() {return false;};
        if (pos1.y() - pos2.y()).abs() > half_e1.y() + half_e2.y() {return false;};
        true
    }

    pub fn overlap_min_max(min1: &Vec2, max1: &Vec2, min2: &Vec2, max2:&Vec2) -> bool {
        if max1.x() < min2.x() || min1.x() > max2.x() {return false;};
        if max1.y() < min2.y() || min1.y() > max2.y() {return false;};
        true
    }

    pub fn draw(pos: &Vec2, half_e: &Vec2) {
        macroquad::prelude::draw_rectangle(pos.x() - half_e.x(), pos.y() - half_e.y(), half_e.x() * 2.0, half_e.y() * 2.0, macroquad::prelude::WHITE);
    }

    pub fn set_collision_layer(&mut self, bits: &[i32]) {
        for bit in bits.iter() {
            self.collision_layer |= 1<< bit;
        }
    }

    pub fn create_collision_layer(bits: &[i32]) -> i32{
        let mut layer: i32 = 0;
        for bit in bits.iter() {
            layer |= 1<< bit;
        }
        layer
    }

    pub fn set_collision_bit(layer: &mut i32, bit: i32, value: bool) {
        if value {
            *layer |= 1 << bit;
        } else {
            *layer &= !(1 << bit)
        }
    }

    pub fn get_collision_bit(layer: i32, bit: i32) -> bool {
        if layer & (1 << bit) == 0 {
            return false;
        }
        true
    }

    pub fn set_collision_layer_bit(&mut self, bit: i32, value: bool) {
        KaAABB::set_collision_bit(&mut self.collision_layer, bit, value);
    }

    pub fn get_collision_layer_bit(&self, bit: i32) -> bool {
        KaAABB::get_collision_bit(self.collision_layer, bit)
    }

    pub fn set_collision_mask_bit(&mut self, bit: i32, value: bool) {
        KaAABB::set_collision_bit(&mut self.collision_mask, bit, value);
    }
    
    pub fn get_collision_mask_bit(&self, bit: i32) -> bool {
        KaAABB::get_collision_bit(self.collision_mask, bit)
    }

    pub fn have_exception(&self, id: u32) -> bool {
        self.collision_exceptions.iter().any(|&i| i == id)
    }

    pub fn add_exception(&mut self, id: u32) {
        if !self.have_exception(id) {
            self.collision_exceptions.push(id);
        }
        
    }

    pub fn can_collide(&self, other: &KaAABB) -> bool {
        if other.collision_layer & self.collision_mask == 0 {
            return false;
        }
        true
    }


    pub fn swept_aabb(pos1: &Vec2, half_e1: &Vec2, pos2: &Vec2, half_e2: &Vec2, vel: &Vec2) -> (Vec2,f32) {
        let x1 = pos1.x();
        let y1 = pos1.y();
        let w1 = half_e1.x();
        let h1 = half_e1.y();

        let x2 = pos2.x();
        let y2 = pos2.y();
        let w2 = half_e2.x();
        let h2 = half_e2.y();

        let vx = vel.x();
        let vy = vel.y();

        let x_inv_entry: f32;
        let y_inv_entry: f32;
        let x_inv_exit: f32;
        let y_inv_exit: f32;
    
        if vx > 0.0 {
            x_inv_entry = (x2 - w2) - (x1 + w1); // if half_extent and x,y at center (b2.x - b2.w) - (b1.x + b1.w)
            x_inv_exit = (x2 + w2) - (x1 - w1); //(b2.x + b2.w) - (b1.x - b1.w)
        } else {
            x_inv_entry = (x2 + w2) - (x1 - w1);
            x_inv_exit = (x2 - w2) - (x1 + w1);
        }
    
        if vy > 0.0 {
            y_inv_entry = (y2 - h2) - (y1 + h1);
            y_inv_exit = (y2 + h2) - (y1 - h1);
        } else {
            y_inv_entry = (y2 + h2) - (y1 - h1);
            y_inv_exit = (y2 - h2) - (y1 + h1);
        }
    
        let x_entry: f32;
        let y_entry: f32;
        let x_exit: f32;
        let y_exit: f32;
    
        if vx == 0.0 {
            if x1 - w1 < x2 + w2 && x2 - w2 < x1 + w1 {
                x_entry = std::f32::NEG_INFINITY;
                x_exit = std::f32::INFINITY;
            } else {
                return (Vec2::new(0.0, 0.0), 1.0);
            }
            
        } else {
            x_entry = x_inv_entry / vx;
            x_exit = x_inv_exit / vx;
        }
    
        if vy == 0.0 {
            if y1 - h1 < y2 + h2 && y2 - h2 < y1 + h1 {
                y_entry = std::f32::NEG_INFINITY;
                y_exit = std::f32::INFINITY;
            } else {
                return (Vec2::new(0.0, 0.0), 1.0);
            }
            
        } else {
            y_entry = y_inv_entry / vy;
            y_exit = y_inv_exit / vy;
        }
    
        let entry_time = x_entry.max(y_entry);
        let exit_time = x_exit.min(y_exit);
    
        if entry_time > exit_time || x_entry < 0.0 && y_entry < 0.0 || x_entry > 1.0 || y_entry > 1.0 {
            return (Vec2::new(0.0, 0.0), 1.0);
        } else {
            if x_entry > y_entry {
                if x_inv_entry < 0.0 {
                    return (Vec2::new(1.0, 0.0), entry_time);
                } else {
                    return (Vec2::new(-1.0, 0.0), entry_time);
                }
            } else {
                if y_inv_entry < 0.0 {
                    return (Vec2::new(0.0, 1.0), entry_time);
                } else {
                    return (Vec2::new(0.0, -1.0), entry_time);
                }
            }
        }
    }

    //To get overlapping KaAABB without having an actual KaAABB with Sensor
    pub fn get_overlapping(world: &World, pos: &Vec2, half_e: &Vec2, mask: i32) -> Vec<Entity>{
        let mut overlapping: Vec<Entity> = Vec::new();
        for (e, aabb) in world.query::<&KaAABB>().iter() {
            if aabb.collision_layer & mask != 0 {
                let pos2 = aabb.pos.lock().unwrap();
                if KaAABB::overlaps(pos, half_e, &pos2, &aabb.half_e) {
                    overlapping.push(e);
                }
            }
        }
        overlapping
    }

    //To get overlapping KaAABB with a specific Component
    pub fn get_overlapping_with<T: hecs::Component>(world: &World, pos: &Vec2, half_e: &Vec2, mask: i32) -> Vec<Entity>{
        let mut overlapping: Vec<Entity> = Vec::new();
        for (e, (aabb, _)) in world.query::<(&KaAABB, &T)>().iter() {
            if aabb.collision_layer & mask != 0 {
                let pos2 = aabb.pos.lock().unwrap();
                if KaAABB::overlaps(pos, half_e, &pos2, &aabb.half_e) {
                    overlapping.push(e);
                }
            }
        }
        overlapping
    }
    
}

pub fn ka_aabb_move( world: &mut World, delta: f32)
{   
    //Moveable
    for (e1, (aabb1, mut moveable)) in world.query::<(&KaAABB, &mut KaMoveable)>().iter() {
        moveable.on_floor = None;
        moveable.on_ceilling = None;
        moveable.on_wall = None;
        let mut vel = moveable.velocity + moveable.external_forces;
        moveable.external_forces = moveable.external_forces.lerp(Vec2::zero(), moveable.external_friction  * delta);
        if moveable.external_forces.length_squared() < 1.0 {
            moveable.external_forces = Vec2::zero();
        }
        if let Some(floor_enity) = moveable.on_floor {
            if let Ok(mut result) = world.query_one::<&KaMoveable>(floor_enity) {
                if let Some(_floor) = result.get() {

                }
            }
        }
        if vel.length_squared() < 1.0 {
            vel = Vec2::zero();
        }
        vel *= delta;

        let mut pos1 = aabb1.pos.lock().unwrap();

        let mut closest_entity: Option<Entity> = None;
        let mut closest_normal = Vec2::zero();
        let mut max_t = 1.0f32;

        for (e2, aabb2) in world.query::<&KaAABB>().iter() {
            if e1.id() != e2.id() && aabb2.solid  && aabb1.can_collide(&aabb2) 
                && !aabb1.have_exception(e2.id()) && !aabb2.have_exception(e1.id())
            {
                let pos2 = aabb2.pos.lock().unwrap();
                let min2 = Vec2::new(pos2.x() - aabb2.half_e.x(), pos2.y() - aabb2.half_e.y());
                if aabb2.one_way && pos1.y() + aabb1.half_e.y() > min2.y() { continue; };
                let (normal, t) = KaAABB::swept_aabb(&pos1, &aabb1.half_e, &pos2, &aabb2.half_e, &vel);
                if t < max_t {
                    max_t = t;
                    closest_entity = Some(e2);
                    closest_normal = normal;
                }
            }
        }

        *pos1.x_mut() += vel.x() * max_t + closest_normal.x() * MARGIN;
        *pos1.y_mut() += vel.y() * max_t + closest_normal.y() * MARGIN;


        if closest_normal.x() != 0.0 {
            moveable.on_wall = closest_entity;
        } else if closest_normal.y() < 0.0 {
            moveable.on_floor = closest_entity;
        } else {
            moveable.on_ceilling = closest_entity;
        }
        if max_t < 1.0 {

            if moveable.slide {
                let dotporod = (vel.x() * closest_normal.y() + vel.y() * closest_normal.x()) * (1.0 - max_t);
                let vel = Vec2::new(dotporod * closest_normal.y(), dotporod * closest_normal.x());
                max_t = 1.0;
                for (e2, aabb2) in world.query::<&KaAABB>().iter() {
                    if e1.id() != e2.id() && aabb2.solid  && aabb1.can_collide(&aabb2) 
                        && !aabb1.have_exception(e2.id()) && !aabb2.have_exception(e1.id())
                    {
                        let pos2 = aabb2.pos.lock().unwrap();
                        let min2 = Vec2::new(pos2.x() - aabb2.half_e.x(), pos2.y() - aabb2.half_e.y());
                        if aabb2.one_way && pos1.y() + aabb1.half_e.y() > min2.y() { continue; };
                        let (normal, t) = KaAABB::swept_aabb(&pos1, &aabb1.half_e, &pos2, &aabb2.half_e, &vel);
                        if t < max_t {
                            max_t = t;
                            closest_entity = Some(e2);
                            closest_normal = normal;
                        }
                    }
                }
        
                *pos1.x_mut() += vel.x() * max_t + closest_normal.x() * MARGIN;
                *pos1.y_mut() += vel.y() * max_t + closest_normal.y() * MARGIN;
        
        
                if closest_normal.x() != 0.0 {
                    moveable.on_wall = closest_entity;
                } else if closest_normal.y() < 0.0 {
                    moveable.on_floor = closest_entity;
                } else {
                    moveable.on_ceilling = closest_entity;
                }
            }
        }
        
    }

    for (_, (aabb, _movable, sprite)) in world.query::<(&KaAABB, &KaMoveable, &mut KaSprite)>().iter() {
        let pos = aabb.pos.lock().unwrap();
        *sprite.pos.x_mut() = pos.x();
        *sprite.pos.y_mut() = pos.y();
    }
}

pub fn ka_aabb_sense( world: &mut World) {
    for (e1, (aabb1, sensor)) in world.query::<(&KaAABB, &mut KaSensor)>().iter() {
        sensor.overlapping.clear();
        if !sensor.active {continue};
        let mask = if sensor.mask != 0 {sensor.mask} else {aabb1.collision_mask};
        let pos1 = aabb1.pos.lock().unwrap();
        for (e2, aabb2) in world.query::<&KaAABB>().iter() {
            if e1.id() != e2.id() && aabb2.collision_layer & mask != 0 
                && !aabb1.have_exception(e2.id()) && !aabb2.have_exception(e1.id())
            {
                let pos2 = aabb2.pos.lock().unwrap();
                if KaAABB::overlaps(&pos1, &aabb1.half_e, &pos2, &aabb2.half_e) {
                    sensor.overlapping.push(e2);
                }
            }
        }
    }
}

pub fn ka_aabb_update( world: &mut World, delta: f32) {
    ka_aabb_move(world, delta);
    ka_aabb_sense(world);
}


pub fn ka_draw_collisions(world: &mut World) {
    use macroquad::prelude::{draw_rectangle, BLUE};
    let mut color = BLUE;
    color.0[3] = 128;
    for (_, aabb) in world.query::<&KaAABB>().iter() {
        let pos = aabb.pos.lock().unwrap();
        let (x, y) = (pos.x() - aabb.half_e.x(), pos.y() - aabb.half_e.y());
        draw_rectangle(x, y, aabb.half_e.x() * 2.0, aabb.half_e.y() * 2.0, color);
    }
}


