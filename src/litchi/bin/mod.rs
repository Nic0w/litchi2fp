use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

use deserializer::Deserializer;

pub use self::mission::LitchiMission;
pub use self::waypoint::WaypointPartial;

pub use error::Error;

mod deserializer;
mod error;
mod mission;
mod waypoint;

const MAGIC: u32 = 0x6C_63_68_6D; //b"lchm"

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FinishAction {
    None = 0,
    ReturnToHome = 1,
    Land = 2,
    BackToFirst = 3,
    Reverse = 4,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PathMode {
    StraigtLines = 0,
    CurvedTurns = 1,
}

#[derive(Deserialize, Debug)]
pub struct PointOfInterest {
    pub latitude: f64,
    pub longitude: f64,

    pub altitude: f32,
}

#[derive(Debug)]
pub enum PhotoInterval {
    Time { seconds: f32 },
    Distance { meters: f32 },
}

pub fn from_slice(bytes: &[u8]) -> Result<LitchiMission, Error> {
    let mut deserializer = Deserializer::from_slice(bytes);

    LitchiMission::deserialize(&mut deserializer)
}
