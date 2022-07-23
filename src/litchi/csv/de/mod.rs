use crate::litchi::Action;

mod deserializer;
mod helpers;

#[derive(Debug)]
pub struct MissionRecord {
    pub waypoint: Coordinates,

    pub heading: f64,

    pub curvesize: f64,

    pub rotationdir: u8,
    pub gimbal: Option<GimbalSettings>,

    pub speed: f32,

    pub poi: Option<Coordinates>,

    pub photo_timeinterval: i8,
    pub photo_distinterval: i8,

    pub actions: Vec<Action>,
}

#[derive(Debug)]
pub enum GimbalSettings {
    FocusPoi(f64),
    Interpolate(f64),
}

#[derive(Debug)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Altitude,
}

#[derive(Debug)]
pub enum Altitude {
    AboveGround(u16),
    Absolute(u16),
}
