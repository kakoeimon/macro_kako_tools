use hecs::{World, EntityBuilder};

pub struct Spawner {
    builders: Vec<EntityBuilder>,
}

impl Spawner {
    pub fn new() -> Self {
        Self {
            builders: Vec::new(),
        }
    }

    pub fn add(&mut self, builder: EntityBuilder) {
        self.builders.push(builder);
    }

    pub fn spawn(&mut self, world: &mut World) {
        while let Some(mut builder) = self.builders.pop() {
            world.spawn(builder.build());
        }
    }
}