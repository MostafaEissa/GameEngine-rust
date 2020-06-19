use super::component::Component;
use std::collections::{HashMap, HashSet};
use std::any::{Any, TypeId};
use super::system::{Read, Write, ReadStorage, WriteStorage, ReadResource, WriteResource};
use std::cell::{RefCell, RefMut, Ref};

pub type EntityId = u64;

struct Entity {
    active: bool,
    components: HashMap<TypeId, Box<RefCell<dyn Any>>>,
}

impl Entity {

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn destroy(&mut self) {
        self.active = false;
    }

    pub fn add_component<'a, T: 'a + Component + Default>(&mut self) -> RefMut<T> {
        self.components.insert(TypeId::of::<T>(), Box::new(RefCell::new(T::default())));
        self.get_component_mut::<T>()
    }

    pub fn get_component<T: Component>(&self) -> Ref<T> {
        let t :Ref<_> = self.components.get(&TypeId::of::<T>()).unwrap().borrow();
        let tx : Ref<_> = Ref::map(t, |e| e.downcast_ref().unwrap());
        tx//.downcast_mut().unwrap()
    }

    pub fn get_component_mut<T: Component>(&self) -> RefMut<T>{
        let t :RefMut<_> = self.components.get(&TypeId::of::<T>()).unwrap().borrow_mut();
        let tx : RefMut<_> = RefMut::map(t, |e| e.downcast_mut().unwrap());
        tx//.downcast_mut().unwrap()
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


pub struct World {
    id: EntityId,
    entities: HashMap<EntityId, Entity>
}

impl World {

    pub fn new() -> Self {

        let mut w = World { id: 0, entities: HashMap::new()};
        w.create_entity();
        w
    }

    pub fn create_entity(&mut self) -> EntityId {
        let entity_id = self.id;
        self.id += 1;
        self.entities.insert(entity_id, Entity::default());
        entity_id
    }

    pub fn add_resource<T: Component + Default>(&mut self) -> RefMut<T> {
        self.add_component(0)
    }

    pub fn add_component<T: Component + Default>(&mut self, entity_id: EntityId) -> RefMut<T> {
        let entity = self.entities.get_mut(&entity_id).unwrap();
        entity.add_component::<T>()
    }

    pub fn fetch_component<'a, T: Component>(&'a self, entity_id: EntityId) -> Read<'a, T> {
        self.entities.get(&entity_id).map(|e| Read {
            inner: e.get_component::<T>(),
        }).unwrap()
    }

    pub fn fetch_resource<'a, T: Component>(&'a self) -> ReadResource<'a, T> {
        ReadResource {
            inner: self.fetch_component(0),
        }
    }

    pub fn fetch_resource_mut<'a, T: Component>(&'a self) -> WriteResource<'a, T> {
        WriteResource {
            inner: self.fetch_component_mut(0),
        }
    }

    pub fn fetch_all_component<'a, T: Component>(&'a self, interests: &HashSet<TypeId>) -> ReadStorage<'a, T> {
        ReadStorage {
            inner:  self.filter(interests).into_iter().map(|k| self.fetch_component(k)).collect()
        }
    }

    pub fn fetch_component_mut<'a, T: Component>(&'a self, entity_id: EntityId) -> Write<'a, T> {
        self.entities.get(&entity_id).map(|e| Write {
            inner: e.get_component_mut::<T>(),
        }).unwrap()
    }

    pub fn fetch_all_component_mut<'a, T: Component>(&'a self, interests: &HashSet<TypeId>) -> WriteStorage<'a, T> {
        WriteStorage {
            inner:  self.filter(interests).into_iter().map(|k| self.fetch_component_mut(k)).collect()
        }
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