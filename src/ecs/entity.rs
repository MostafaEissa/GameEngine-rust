use super::component::Component;
use std::collections::{HashMap, HashSet};
use std::any::{Any, TypeId};

pub type EntityId = u64;

struct Entity {
    active: bool,
    components: HashMap<TypeId, Box<dyn Any>>,
}

impl Entity {

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn destroy(&mut self) {
        self.active = false;
    }

    pub fn add_component<T: 'static + Component + Default>(&mut self) -> &mut T {
        self.components.insert(TypeId::of::<T>(), Box::new(T::default()));
        self.get_component_mut::<T>()
    }

    pub fn get_component<T: Component>(&self) -> &T{
        let t = self.components.get(&TypeId::of::<T>()).unwrap();
        t.downcast_ref().unwrap()
    }

    pub fn get_component_mut<T: Component>(&mut self) -> &mut T{
        let t = self.components.get_mut(&TypeId::of::<T>()).unwrap();
        t.downcast_mut().unwrap()
    }

    pub fn has_all(&self, types: &HashSet<TypeId>) -> bool {
        for tp in types {
            if !self.components.contains_key(tp) {
                return false;
            }
        }
        true
    }

}

impl Default for Entity {
    fn default() -> Self {
        Entity {active: true, components: HashMap::new()}
    }
}


pub struct EntityManager {
    id: EntityId,
    entities: HashMap<EntityId, Entity>
}

impl EntityManager {

    pub fn create_entity(&mut self) -> EntityId {
        let entity_id = self.id;
        self.id += 1;
        self.entities.insert(entity_id, Entity::default());
        entity_id
    }

    pub fn add_component<T: Component + Default>(&mut self, entity_id: EntityId) -> &mut T {
        let entity = self.entities.get_mut(&entity_id).unwrap();
        entity.add_component::<T>()
    }

    pub fn get_component<T: Component>(&self, entity_id: EntityId) -> &T {
        let entity = self.entities.get(&entity_id).unwrap();
        entity.get_component::<T>()
    }

    pub fn get_component_mut<T: Component>(&mut self, entity_id: EntityId) -> &mut T {
        let entity = self.entities.get_mut(&entity_id).unwrap();
        entity.get_component_mut::<T>()
    }

    pub fn filter(&self,  types: &HashSet<TypeId>) ->  Vec<EntityId> {
        self.entities.iter().filter(|(_,v)| v.has_all(types)).map(|(&k, _)| k).collect::<Vec<_>>()
    }

    pub fn destroy(&mut self, entity_id: EntityId) {
        let entity = self.entities.get_mut(&entity_id).unwrap();
        entity.destroy();
    }

    pub fn refresh(&mut self) {
        self.entities.retain(|_, v| v.is_active());
    }
}

impl Default for EntityManager {
    fn default() -> Self {
        EntityManager { id: 0, entities: HashMap::new()}
    }
}