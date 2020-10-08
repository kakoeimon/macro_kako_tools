use macroquad::{Vec2, draw_texture_ex, Color, Texture2D, DrawTextureParams, Rect};

use hecs::{World, Entity};

pub struct KaSprite {
    pub pos: Vec2,
    pub offset: Vec2,
    pub scale: Vec2,
    pub texture: Texture2D,
    pub color: Color,
    pub z: u32,
    pub rotation: f32,
    pub frame: Option<Rect>,
    
}

pub fn ka_draw_sprites(world: &mut World) {
    let mut q = world.query::<&KaSprite>();
    let mut query: Vec<(Entity, &KaSprite)> = q.iter().collect();

    query.sort_by(|a, b| a.1.z.partial_cmp(&b.1.z).unwrap() );
    for (_, sprite) in query.iter() {
        match sprite.frame {
            Some(frame) => {
                let (x, y) = (sprite.pos.x() + (sprite.offset.x() - frame.w) * sprite.scale.x() / 2.0 , sprite.pos.y() + (sprite.offset.y() - frame.h) * sprite.scale.y() / 2.0);
                let (x,y) = (x.floor(), y.floor());
                let params = DrawTextureParams {
                    dest_size: Some(Vec2::new(frame.w * sprite.scale.x(), frame.h * sprite.scale.y())),
                    source: sprite.frame,
                    rotation: sprite.rotation,
                    //pivot: None,
                };
                draw_texture_ex(sprite.texture, x, y, sprite.color, params);

            },
            None => {
                
                let (x, y) = (sprite.pos.x() + sprite.offset.x() - sprite.texture.width() * sprite.scale.x() / 2.0 , sprite.pos.y() + sprite.offset.y() - sprite.texture.height() * sprite.scale.y() / 2.0);
                let params = DrawTextureParams {
                    dest_size: Some(Vec2::new(sprite.texture.width() * sprite.scale.x(), sprite.texture.height() * sprite.scale.y())),
                    source: sprite.frame,
                    rotation: sprite.rotation,
                    //pivot: None,
                };
                draw_texture_ex(sprite.texture, x, y, sprite.color, params);
            }
        }
        
    }
}

