use std::path::Path;

use serde::{Deserialize, de::{Visitor, self}};
use serde_repr::{Serialize_repr, Deserialize_repr};

use deserializer::Deserializer;

pub use self::waypoint::Waypoint;

mod deserializer;
mod waypoint;

const MAGIC: u32 = 0x6C_63_68_6D; //b"lchm"

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u32)]
pub enum FinishAction {
    None = 0,
    ReturnToHome = 1,
    Land = 2,
    BackToFirst = 3,
    Reverse = 4
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u32)]
pub enum PathMode {
    StraigtLines = 0,
    CurvedTurns = 1
}

#[derive(Deserialize, Debug)]
pub struct PointOfInterest {
    pub latitude: f64,
    pub longitude: f64,

    pub altitude: f32,
}

#[derive(Deserialize, Debug)]
pub struct LitchiMission {

    _unknown: u32,
    pub finish_action: FinishAction,
    pub path_mode: PathMode,
    pub cruising_speed: f32,
    pub max_speed: f32,

    _b : [u32; 4], 
    
    pub waypoints: Vec<Waypoint>,
    pub poi: Vec<PointOfInterest>
}

pub fn from_slice(bytes: &[u8]) -> LitchiMission {

    let mut deserializer = Deserializer::from_slice(&bytes);

    let maybe_magic = deserializer.parse_u32();

    if MAGIC != maybe_magic {
        //return Err(BadMagic)
    }

    println!("{} {}", MAGIC, maybe_magic);

    let mission = LitchiMission::deserialize(&mut deserializer);

    mission.unwrap()
}

pub fn from_path<P: AsRef<Path>>(path: P) {

    
}
