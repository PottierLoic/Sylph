use std::any::TypeId;
use std::collections::HashMap;

use crate::component::{Component, ComponentStorage, DenseComponentStorage};
use crate::entity::{Entity, EntityManager};

#[derive(Default)]
pub struct World {
  entities: EntityManager,
  storages: HashMap<TypeId, Box<dyn ComponentStorage>>,
  labels: HashMap<String, Entity>,
}

impl World {
  pub fn spawn(&mut self) -> Entity {
    self.entities.create()
  }

  pub fn despawn(&mut self, entity: Entity) -> bool {
    if self.entities.is_alive(entity) {
      self.entities.destroy(entity);

      for storage in self.storages.values_mut() {
        storage.remove(entity);
      }

      true
    } else {
      false
    }
  }

  pub fn is_alive(&self, entity: Entity) -> bool {
    self.entities.is_alive(entity)
  }

  pub fn insert<T: Component>(&mut self, entity: Entity, component: T) {
    let type_id = TypeId::of::<T>();

    let storage = self.storages
      .entry(type_id)
      .or_insert_with(|| Box::new(DenseComponentStorage::<T>::default()))
      .as_any_mut()
      .downcast_mut::<DenseComponentStorage<T>>()
      .unwrap();

    storage.insert(entity, component);
  }

  pub fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
    let type_id = TypeId::of::<T>();
    let storage = self.storages.get(&type_id)?;
    let storage = storage.as_any().downcast_ref::<DenseComponentStorage<T>>()?;
    storage.get(entity)
  }

  pub fn get_component_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
    let type_id = TypeId::of::<T>();
    let storage = self.storages.get_mut(&type_id)?;
    let storage = storage.as_any_mut().downcast_mut::<DenseComponentStorage<T>>()?;
    storage.get_mut(entity)
  }

  pub fn label(&mut self, name: impl Into<String>, entity: Entity) {
    self.labels.insert(name.into(), entity);
  }

  pub fn get_labeled(&self, name: &str) -> Option<Entity> {
    self.labels.get(name).cloned()
  }

  pub fn entities(&self) -> &EntityManager {
    &self.entities
  }
}
