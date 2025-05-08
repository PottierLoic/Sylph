use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Entity {
  pub id: u32,
  pub generation: u32,
}

#[derive(Default)]
pub struct EntityManager {
  generations: Vec<u32>,
  free: VecDeque<u32>,
}

impl EntityManager {
  pub fn create(&mut self) -> Entity {
    if let Some(id) = self.free.pop_front() {
      let generations = self.generations[id as usize];
      Entity {
        id,
        generation: generations,
      }
    } else {
      let id = self.generations.len() as u32;
      self.generations.push(0);
      Entity { id, generation: 0 }
    }
  }

  pub fn destroy(&mut self, entity: Entity) {
    let id = entity.id as usize;
    if self.is_alive(entity) {
      self.generations[id] += 1;
      self.free.push_back(entity.id);
    }
  }

  pub fn is_alive(&self, entity: Entity) -> bool {
    self
      .generations
      .get(entity.id as usize)
      .is_some_and(|&generation| generation == entity.generation)
  }

  pub fn alive_iter(&self) -> impl Iterator<Item = Entity> + '_ {
    self
      .generations
      .iter()
      .enumerate()
      .filter_map(|(id, &generation)| {
        if !self.free.contains(&(id as u32)) {
          Some(Entity {
            id: id as u32,
            generation,
          })
        } else {
          None
        }
      })
  }
}
