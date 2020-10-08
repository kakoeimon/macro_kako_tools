use macroquad::{Vec2, Camera2D, set_camera};


pub struct KaCamera {
    width: f32,
    height: f32,
    scale: f32,
    pub camera: Camera2D,
}

impl KaCamera {
    pub fn new(width: f32, height: f32) -> Self {
        let scale = 2.0 / width;
        Self {
            width,
            height,
            scale,
            camera: Camera2D 
                {
                    offset: Vec2::zero(),
                    zoom: Vec2::new(scale, -2.0 / height),
                    ..Default::default()
                }
        }
    }

    pub fn screen_to_world(&self, pos: (f32, f32)) -> Vec2 {
        use macroquad::{screen_width, screen_height, vec2};
        let screen_size = vec2(screen_width(), screen_height());
        let mut pos = Vec2::new(pos.0, pos.1);
        pos -= screen_size / 2.0;
        pos /= screen_size * self.scale;
        pos   
    }

    pub fn set(&self) {
        set_camera(self.camera);
    }

    pub fn get_scale(&self) -> f32 {
        self.scale
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn get_pos(&self) -> Vec2 {
        -self.camera.offset / self.scale
    }

    pub fn set_pos_v(&mut self, pos: Vec2) {
        self.camera.offset = -pos * self.scale;
    }

    pub fn set_pos_xy(&mut self, x: f32, y: f32) {
        self.camera.offset.set_x(-x * self.scale);
        self.camera.offset.set_y(-y * self.scale);
    }

    pub fn move_v(&mut self, movement: Vec2) {
        *self.camera.offset.x_mut() -= movement.x() * self.scale;
        *self.camera.offset.y_mut() -= movement.y() * self.scale;
    }

    pub fn move_xy(&mut self, x: f32, y: f32) {
        *self.camera.offset.x_mut() -= x * self.scale;
        *self.camera.offset.y_mut() -= y * self.scale;
    }

    pub fn get_camera(&self) -> Camera2D {
        self.camera
    }

    pub fn get_aabb_data(&self) -> (Vec2, Vec2) {
        let pos = self.get_pos();
        let half_e = Vec2::new(self.width / 2.0, self.height / 2.0);
        (pos, half_e)
    }

}