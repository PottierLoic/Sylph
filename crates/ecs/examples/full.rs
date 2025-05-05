use ecs::world::World;
use ecs::system::System;

#[derive(Debug)]
struct Position {
  x: f32,
  y: f32,
}

#[derive(Debug, Clone)]
struct Velocity {
  dx: f32,
  dy: f32,
}

#[derive(Debug)]
struct Health(u32);

struct MovementSystem;
impl System for MovementSystem {
  fn run(&mut self, world: &mut World) {
    let entities: Vec<_> = world.entities().alive_iter().collect();
    for entity in entities {
      let vel = world.get_component::<Velocity>(entity).cloned();
      if let (Some(pos), Some(vel)) = (
        world.get_component_mut::<Position>(entity),
        vel,
      ) {
        pos.x += vel.dx;
        pos.y += vel.dy;
      }
    }
  }
}

struct HealthSystem;
impl System for HealthSystem {
  fn run(&mut self, world: &mut World) {
    let entities: Vec<_> = world.entities().alive_iter().collect();
    for entity in entities {
      if let Some(hp) = world.get_component_mut::<Health>(entity) {
        hp.0 -= 10;
        println!("Entity {} now has {} HP", entity.id, hp.0);
        if hp.0 <= 0 {
          println!("Entity {} has died. Despawning...", entity.id);
          world.despawn(entity);
        }
      }
    }
  }
}

fn main() {
  // Creating the world which is the equivalent of the unity scene
  let mut world = World::default();

  // Spawn player
  let player = world.spawn();
  world.label("player", player); // adding the label so we can find it later
  world.insert(player, Position { x: 0.0, y: 0.0 }); // adding components, should probably be renamed to add_component
  world.insert(player, Velocity { dx: 1.0, dy: 1.5 });
  world.insert(player, Health(100));

  // Spawn dummy NPC
  let npc = world.spawn();
  world.label("npc", npc);
  world.insert(npc, Position { x: 10.0, y: -2.0 });
  world.insert(npc, Velocity { dx: -0.5, dy: 0.0 });
  world.insert(npc, Health(20));

  // Run systems
  // The movement system is applying the velocity to the position
  // the health system is decreasing everyone health by 10 and despawn entities with health <= 0
  let mut movement_system = MovementSystem;
  let mut health_system = HealthSystem;
  movement_system.run(&mut world);
  health_system.run(&mut world);
  health_system.run(&mut world);

  // Inspect player state
  let player = world.get_labeled("player").unwrap(); // getting the entity using the label set previously
  let pos = world.get_component::<Position>(player).unwrap();
  let hp = world.get_component::<Health>(player).unwrap();
  println!("Player moved to: ({:.1}, {:.1})", pos.x, pos.y);
  println!("Player HP: {}", hp.0);
}
