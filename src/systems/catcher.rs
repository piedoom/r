//! Is responsible for the "outline" arrows at the bottom of the screen used to indicate 
//! user input.

use amethyst::core::Transform;
use amethyst::input::{InputHandler, InputEvent};
use amethyst::shrev::EventChannel;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage, Write, Resources, ReaderId};
use amethyst::ecs::SystemData;
use amethyst::core::Time;
use std::time::Duration;
use std::collections::HashMap;
use amethyst::input::InputBundle;
use amethyst::core::EventReader;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};


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
  );

  fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader = Some(res.fetch_mut::<EventChannel<KeyEvent>>().register_reader());
  }

  fn run(&mut self, (transforms, event_channel, time): Self::SystemData) {
    for event in event_channel.read(self.reader.as_mut().unwrap()) {
        match event {
            InputEvent::ActionPressed(action) => {
                if let ActionKey::NotSupported = action { continue }
                self.key_data.insert(action.clone(), ActionKeyTiming { on: time.absolute_time(), off: None } );
                dbg!(&self.key_data);
            }
            InputEvent::ActionReleased(action) => {
                if let ActionKey::NotSupported = action { continue }
                if let Some(entry) = self.key_data.get(action) {
                    self.key_data.insert(action.clone(), ActionKeyTiming { on: entry.on, off: Some(time.absolute_time()) } );
                }
                dbg!(&self.key_data);
            }
            _ => { continue }
        }
    }
    // let (x_opt, y_opt) = (input.axis_value("x"), input.axis_value("y"));

    // If not setup correctly, return early.
    // if x_opt.is_none() || y_opt.is_none() { return }

    // // else, assign those unwrapped values to a vector and continue operation
    // let (x, y) = (x_opt.unwrap(), y_opt.unwrap());
    // if x.abs() > DEADZONE {
    //     // x event
    // }
    // if y.abs() > DEADZONE {
    //     // y event
    // }
  }
}