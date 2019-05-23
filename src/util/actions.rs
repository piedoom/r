//! This contains an enumeration of possible actions which are further defined in
//! our `bindings_config.ron`.
use crate::util::Direction;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};
use std::time::Duration;

/// The `ActionKey` represents all possible actions and is further defined in the `bindings_config.ron`.
/// 
/// * `NotSupported` - catches all keys not defined in the actions. This is useful when destructuring keypress events
/// in a system.
#[derive(Hash, PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
pub enum ActionKey {
    NotSupported,
    Up,
    Down,
    Left,
    Right,
}

/// Structure allows us to measure the timing of when keys were pressed.
/// 
/// * `on` - `Duration` that is reset whenever the key is pressed
/// * `off` - `Option<Duration>` that is set to `None` when the key is currently down, and a value when key has come back up.
#[derive(Debug)]
pub struct ActionKeyTiming {
    pub on: Duration,
    pub off: Option<Duration>,
}

impl From<&Direction> for ActionKey {
    fn from(direction: &Direction) -> Self {
        match direction {
            Direction::Left => ActionKey::Left,
            Direction::Up => ActionKey::Up,
            Direction::Down => ActionKey::Down,
            Direction::Right => ActionKey::Right,
        }
    }
}

impl Display for ActionKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}
