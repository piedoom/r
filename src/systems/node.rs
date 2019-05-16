use crate::node::*;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

pub struct NodeSystem;

impl<'s> System<'s> for NodeSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    ReadStorage<'s, Node>,
    Read<'s, InputHandler<String, String>>,
  );

  fn run(&mut self, (mut transforms, nodes, input): Self::SystemData) {
    for (_node, mut transform) in (&nodes, &mut transforms).join() {
      transform.prepend_translation_y(1.0);
    }
  }
}