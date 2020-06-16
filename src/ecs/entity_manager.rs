use super::entity::Entity;


pub struct EntityManager {
    entities: Vec<Entity>
}

impl EntityManager {

    pub fn new() -> Self {
        EntityManager {entities: vec![]}
    }
    pub fn update(&mut self) {
        for entity in &mut self.entities {
            entity.update();
        }
    }

    pub fn draw(&self) {
        for entity in &self.entities {
            entity.draw();
        }
    }

    pub fn refresh(&mut self) {
        self.entities.retain(|e| e.is_active());
    }

    pub fn add_entity(&mut self) -> &mut Entity {
        self.entities.push(Entity::new());
        self.entities.last_mut().unwrap()
    }

    pub fn get_entity(&mut self, entity_id: u32) -> &mut Entity {
        self.entities.get_mut(entity_id as usize).unwrap()
    }
}
