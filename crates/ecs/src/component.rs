use crate::entity::Entity;
use std::any::Any;
use std::collections::HashMap;

pub trait Component: 'static + Send + Sync {}
impl<T: 'static + Send + Sync> Component for T {}

pub trait ComponentStorage {
  fn remove(&mut self, entity: Entity);
  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct DenseComponentStorage<T> {
  data: HashMap<u32, T>, // Entity.id -> Component
}

impl<T> Default for DenseComponentStorage<T> {
  fn default() -> Self {
    Self {
      data: HashMap::new(),
    }
  }
}

impl<T> DenseComponentStorage<T> {
  pub fn insert(&mut self, entity: Entity, component: T) {
    self.data.insert(entity.id, component);
  }

  pub fn get(&self, entity: Entity) -> Option<&T> {
    self.data.get(&entity.id)
  }

  pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
    self.data.get_mut(&entity.id)
  }
}

impl<T: 'static> ComponentStorage for DenseComponentStorage<T> {
  fn remove(&mut self, entity: Entity) {
    self.data.remove(&entity.id);
  }

  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}
