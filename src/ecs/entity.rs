use super::component::Component;

pub struct Entity {
    active: bool,
    components: Vec<Box<dyn Component>>,
}

impl Entity {

    pub fn new() -> Self {
        Entity {active: true, components: vec![]}
    }
    
    pub fn update(&mut self) {
        for c in &mut self.components {
            c.update();
        }
        for c in &self.components {
            c.draw();
        }
    }
    
    pub fn draw(&self) {}

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn destroy(&mut self) {
        self.active = false;
    }

    pub fn add_component<T: 'static + Component + Default>(&mut self) {
        self.components.push(Box::new(T::default()));
    }

    pub fn get_component<T: Component>(&mut self) -> &mut T{
        let t = self.components.get_mut(0).unwrap();
        t.as_any().downcast_mut().unwrap()
    }
}