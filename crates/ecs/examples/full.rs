use ecs::world::World;
use ecs::system::System;
use ecs::components::Transform;

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
      if let (Some(transform), Some(vel)) = (
        world.get_component_mut::<Transform>(entity),
        vel,
      ) {
        transform.position[0] += vel.dx;
        transform.position[1] += vel.dy;
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
  let mut world = World::default();

  // Spawn player
  let player = world.spawn("player");
  world.label("player", player);
  world.insert(player, Velocity { dx: 1.0, dy: 1.5 });
  world.insert(player, Health(100));

  // Spawn dummy NPC
  let npc = world.spawn("enemy");
  world.label("npc", npc);
  world.insert(npc, Velocity { dx: -0.5, dy: 0.0 });
  world.insert(npc, Health(20));

  let mut movement_system = MovementSystem;
  let mut health_system = HealthSystem;

  movement_system.run(&mut world);
  health_system.run(&mut world);
  health_system.run(&mut world);

  let player = world.get_labeled("player").unwrap();
  let transform = world.get_component::<Transform>(player).unwrap();
  let hp = world.get_component::<Health>(player).unwrap();

  println!(
    "Player moved to: ({:.1}, {:.1})",
    transform.position[0], transform.position[1]
  );
  println!("Player HP: {}", hp.0);
}
