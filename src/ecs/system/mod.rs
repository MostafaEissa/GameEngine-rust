mod render_system;
mod physics_system;
mod keyboard_system;

pub use render_system::RenderSystem;
pub use render_system::AnimationSystem;
pub use physics_system::PhysicsSystem;
pub use keyboard_system::KeyboardSystem;


use super::entity::World;
use super::component::Component;
use std::collections::HashSet;
use std::any::TypeId;
use std::cell::{RefMut,Ref};
use std::ops::{Deref, DerefMut};

pub trait SystemData<'a> {
    fn fetch(interests: &HashSet<TypeId>, world: &'a World) -> Self;
    fn interests() -> HashSet<TypeId>;
}

pub struct Read<'a, T> {
    pub inner : Ref<'a, T>,
}

impl<'a, T> Deref for Read<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &*self.inner
    }
}

pub struct ReadStorage<'a, T> {
    pub inner : Vec<Read<'a, T>>,
}

impl<'a, T> IntoIterator for ReadStorage<'a, T> {
    type Item = Read<'a, T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a, T: Component> SystemData<'a> for ReadStorage<'a, T> {
    fn fetch(interests: &HashSet<TypeId>, world: &'a World) -> Self {
        world.fetch_all_component::<T>(interests)
    }

    fn interests() -> HashSet<TypeId> {
        let mut set = HashSet::new();
        set.insert(TypeId::of::<T>());
        set
    }
}

pub struct ReadResource<'a, T> {
    pub inner : Read<'a, T>,
}

impl<'a, T: Component> SystemData<'a> for ReadResource<'a, T> {
    fn fetch(_interests: &HashSet<TypeId>, world: &'a World) -> Self {
        world.fetch_resource::<T>()
    }

    fn interests() -> HashSet<TypeId> {
        HashSet::new()
    }
}

impl<'a, T> Deref for ReadResource<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &*self.inner
    }
}

pub struct Write<'a, T> {
    pub inner : RefMut<'a, T>,
}

impl<'a, T> Deref for Write<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &*self.inner
    }
}

impl<'a, T> DerefMut for Write<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.inner
    }
}

pub struct WriteStorage<'a, T> {
    pub inner : Vec<Write<'a, T>>,
}

impl<'a, T> IntoIterator for WriteStorage<'a, T> {
    type Item = Write<'a, T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a, T: Component> SystemData<'a> for WriteStorage<'a, T> {
    fn fetch(interests: &HashSet<TypeId>, world: &'a World) -> Self {
        world.fetch_all_component_mut::<T>(interests)
    }

    fn interests() -> HashSet<TypeId> {
        let mut set = HashSet::new();
        set.insert(TypeId::of::<T>());
        set
    }

}

pub struct WriteResource<'a, T> {
    pub inner : Write<'a, T>,
}

impl<'a, T: Component> SystemData<'a> for WriteResource<'a, T> {
    fn fetch(_interests: &HashSet<TypeId>, world: &'a World) -> Self {
        world.fetch_resource_mut::<T>()
    }

    fn interests() -> HashSet<TypeId> {
        HashSet::new()
    }
}

impl<'a, T> Deref for WriteResource<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &*self.inner
    }
}

impl<'a, T> DerefMut for WriteResource<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.inner
    }
}


impl<'a, A : SystemData<'a>, B : SystemData<'a>> SystemData<'a> for (A, B) {
    fn fetch(interests: &HashSet<TypeId>, world: &'a World) -> Self {
        (<A as SystemData>::fetch(interests, world), <B as SystemData>::fetch(interests, world))
    }

    fn interests() -> HashSet<TypeId> {
        let mut set = HashSet::new();
        let s1 = <A as SystemData>::interests();
        let s2 = <B as SystemData>::interests();
        set.extend(s1);
        set.extend(s2);
        set
    }
}

impl<'a, A : SystemData<'a>, B : SystemData<'a>,  C: SystemData<'a>> SystemData<'a> for (A, B, C) {
    fn fetch(interests: &HashSet<TypeId>, world: &'a World) -> Self {
        (<A as SystemData>::fetch(interests, world), <B as SystemData>::fetch(interests, world), <C as SystemData>::fetch(interests, world))
    }

    fn interests() -> HashSet<TypeId> {
        let mut set = HashSet::new();
        let s1 = <A as SystemData>::interests();
        let s2 = <B as SystemData>::interests();
        let s3 = <C as SystemData>::interests();
        set.extend(s1);
        set.extend(s2);
        set.extend(s3);
        set
    }
}

impl<'a, A : SystemData<'a>, B : SystemData<'a>,  C: SystemData<'a>, D: SystemData<'a>> SystemData<'a> for (A, B, C, D) {
    fn fetch(interests: &HashSet<TypeId>, world: &'a World) -> Self {
        (<A as SystemData>::fetch(interests, world), <B as SystemData>::fetch(interests, world), <C as SystemData>::fetch(interests, world), <D as SystemData>::fetch(interests, world))
    }

    fn interests() -> HashSet<TypeId> {
        let mut set = HashSet::new();
        let s1 = <A as SystemData>::interests();
        let s2 = <B as SystemData>::interests();
        let s3 = <C as SystemData>::interests();
        let s4 = <D as SystemData>::interests();
        set.extend(s1);
        set.extend(s2);
        set.extend(s3);
        set.extend(s4);
        set
    }
}

impl<'a, A : SystemData<'a>, B : SystemData<'a>,  C: SystemData<'a>, D: SystemData<'a>, E: SystemData<'a>> SystemData<'a> for (A, B, C, D, E) {
    fn fetch(interests: &HashSet<TypeId>, world: &'a World) -> Self {
        (<A as SystemData>::fetch(interests, world), <B as SystemData>::fetch(interests, world), 
        <C as SystemData>::fetch(interests, world), <D as SystemData>::fetch(interests, world),
        <E as SystemData>::fetch(interests, world)
    )
    }

    fn interests() -> HashSet<TypeId> {
        let mut set = HashSet::new();
        let s1 = <A as SystemData>::interests();
        let s2 = <B as SystemData>::interests();
        let s3 = <C as SystemData>::interests();
        let s4 = <D as SystemData>::interests();
        let s5 = <E as SystemData>::interests();
        set.extend(s1);
        set.extend(s2);
        set.extend(s3);
        set.extend(s4);
        set.extend(s5);
        set
    }
}

pub trait System<'a> {
    type Item: SystemData<'a>;
    fn run(&mut self, data: Self::Item);
}

pub trait Runnable<'a> {
    fn run(&mut self, world: &'a World);
}

impl<'a, T: System<'a>> Runnable<'a> for T {
    fn run(&mut self, world: &'a World) {
        let data = T::Item::fetch(&T::Item::interests(), world);
        self.run(data);
    }
}

#[macro_export]
macro_rules! zip {
    ($t1:expr,$t2:expr) => {
        $t1.into_iter().zip($t2)
    };
    ($t1:expr,$t2:expr, $t3:expr) => {
        $t1.into_iter().zip($t2).zip($t3).map(|((x, y), z)| (x, y, z))
    };
}