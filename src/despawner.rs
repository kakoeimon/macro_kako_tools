use hecs::{World, Entity};

pub struct Despawner {
    entities: Vec<Entity>,
}

impl Despawner {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    pub fn add(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn despawn(&mut self, world: &mut World) {
        for entity in self.entities.iter() {
            match world.despawn(*entity) {
                Ok(()) => (),
                Err(_) => (),
            }
        }
        self.entities.clear();
    }
}