pub mod bin;
pub mod csv;
pub mod kml;

#[derive(Debug)]
pub enum Action {
    StayFor { ms: usize },
    TakePhoto,
    StartRecording,
    StopRecording,
    RotateAircraft { angle: u16 },
    TiltCamera { angle: i16 },
}