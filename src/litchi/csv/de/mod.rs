
mod helpers;
mod deserializer;

#[derive(Debug)]
pub struct MissionRecord {

    pub waypoint: Coordinates,
    //pub latitude: f64,
    //pub longitude: f64,

    //#[serde(rename = "altitude(m)")]
    //pub altitude: u16,

    //#[serde(rename = "heading(deg)")]
    pub heading: f64,

    //#[serde(rename = "curvesize(m)")]
    pub curvesize: f64,

    pub rotationdir: u8, 
    pub gimbal: Option<GimbalSettings>,
    
    //pub altitudemode: u8,

    //#[serde(rename = "speed(m/s)")]
    pub speed: f32,

    pub poi: Option<Coordinates>,
    //pub poi_latitude: f64,
    //pub poi_longitude: f64,

    //#[serde(rename = "poi_altitude(m)")]
    //pub poi_altitude: u16,
    //pub poi_altitudemode: u8,
    pub photo_timeinterval: i8,
    pub photo_distinterval: i8,

    pub actions: Vec<Action>
}

#[derive(Debug)]
pub enum  GimbalSettings {
    FocusPoi(f64),
    Interpolate(f64)
}

#[derive(Debug)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Altitude
}

#[derive(Debug)]
pub enum Altitude {
    AboveGround(u16),
    Absolute(u16)
}

#[derive(Debug)]
pub enum Action {
    StayFor { ms: usize },
    TakePhoto,
    StartRecording,
    StopRecording,
    RotateAircraft { angle: u16 },
    TiltCamera { angle: i16 },
}

