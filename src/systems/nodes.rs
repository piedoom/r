use crate::components::Node;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

pub struct NodesSystem;

/// Controls behavior of individual nodes (i.e. the things that show what button to press and when)
impl<'s> System<'s> for NodesSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    ReadStorage<'s, Node>,
  );

  fn run(&mut self, (mut transforms, nodes): Self::SystemData) {
    // Move each node downwards at a constant speed
    for (_node, transform) in (&nodes, &mut transforms).join() {
      transform.prepend_translation_y(-1.0);
    }
  }
}