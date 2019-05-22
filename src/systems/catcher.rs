//! Is responsible for the "outline" arrows at the bottom of the screen used to indicate 
//! user input.

use amethyst::core::Transform;
use amethyst::input::{InputHandler, InputEvent};
use amethyst::shrev::EventChannel;
use amethyst::ecs::{SystemData, Component, Join, Read, ReadStorage, System, WriteStorage, Write, Resources, ReaderId, DenseVecStorage};
use amethyst::core::Time;
use std::time::Duration;
use std::collections::HashMap;
use amethyst::input::InputBundle;
use amethyst::core::EventReader;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};
use crate::node::NodeDirection;

/// How much before values are not ignored
static DEADZONE: f64 = 0.2f64;

#[derive(Hash, PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
pub enum ActionKey {
    NotSupported,
    Up,
    Down,
    Left,
    Right,
}

impl From<&NodeDirection> for ActionKey {
    fn from(direction: &NodeDirection) -> Self {
        match direction {
            NodeDirection::Left => ActionKey::Left,
            NodeDirection::Up => ActionKey::Up,
            NodeDirection::Down => ActionKey::Down,
            NodeDirection::Right => ActionKey::Right,
        }
    }
}

impl Display for ActionKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

type KeyEvent = InputEvent<ActionKey>;

#[derive(Debug)]
struct ActionKeyTiming {
    on: Duration,
    off: Option<Duration>,
}

pub struct CatcherSystem {
    /// Key data contains a hash for each wanted "key" (this is helpful if we want to)
    /// add additional keys in the future. Each time a key is pressed, a record will
    /// be created or updated with the key pressed along with its timestamp as a `Duration`
    key_data: HashMap<ActionKey, ActionKeyTiming>,
    reader: Option<ReaderId<KeyEvent>>,
}

impl Default for CatcherSystem {
    fn default() -> Self {
        Self {
            key_data: HashMap::new(),
            reader: None,
        }
    }
}

impl<'s> System<'s> for CatcherSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    Write<'s, EventChannel<KeyEvent>>,
    Read<'s, Time>,
    ReadStorage<'s, Catcher>,
  );

  fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader = Some(res.fetch_mut::<EventChannel<KeyEvent>>().register_reader());
  }

  fn run(&mut self, (transforms, event_channel, time, catchers): Self::SystemData) {

    // Match events and write to our state data
    for event in event_channel.read(self.reader.as_mut().unwrap()) {
        match event {

            // On key down...
            InputEvent::ActionPressed(action) => {
                if let ActionKey::NotSupported = action { continue }
                self.key_data.insert(action.clone(), ActionKeyTiming { on: time.absolute_time(), off: None } );
            }

            // On key up...
            InputEvent::ActionReleased(action) => {
                if let ActionKey::NotSupported = action { continue }
                if let Some(entry) = self.key_data.get(action) {
                    self.key_data.insert(action.clone(), ActionKeyTiming { on: entry.on, off: Some(time.absolute_time()) } );
                }
            }

            // Else, do nothing
            _ => { continue }
        }
    }

    // Apply data to our catcher entities
    for catcher in catchers.join() {
        // see if we have a record in our HashMap for this catcher's direction
        let data = self.key_data.get(&ActionKey::from(&catcher.direction));
        if let Some(d) = data {
            // set the render index of the catcher depending on our timing data
            if time.absolute_time() > d.on {
                // set to active sprite
                
            }
            if d.off.is_some() {
                if time.absolute_time() > d.off.unwrap() {
                    // set to inactive sprite

                }
            }
            if d.off.is_none() {
                // set to inactive sprite

            }
        }
    }
  }
}

// Catcher component
#[derive(Default)]
pub struct Catcher {
    pub direction: NodeDirection,
}

impl Catcher {
    pub fn new(direction: NodeDirection) -> Self {
        Self {
            direction,
        }
    }

    /// Get the sprite index depending on the direction and active state
    /// 
    /// * `active` - whether or not the key that should activate this sprite is pressed
    pub fn sprite_index(&self, active: bool) -> usize {
        let index = self.direction.to_usize();
        // if not active, we don't need to do any multiplcation stuff.
        if !active { 
            index 
        } else {
            // operate as a 1 index so we can calculate the new sprite easier (since 0 * 2 == 0)
            let index_1 = index + 1;
            (index_1 * 2) - 1
        }
    }
}

impl Component for Catcher {
    type Storage = DenseVecStorage<Self>;
}
