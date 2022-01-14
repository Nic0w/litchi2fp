use std::hash::Hash;

use chrono::prelude::*;

mod color;
mod from_csv;
mod from_kml;
mod model;

pub use model::*;
pub use color::POI_COLORS;

use crate::{litchi::{csv::de::MissionRecord, kml::Mission}, error::Error};

const DEFAULT_SPEED_MS: u8 = 5;
const DEFAULT_WAYPOINT_ALTITUDE_M: u16 = 3;

pub fn from_csv<'t, 'f>(title: &str, records: &'t [MissionRecord]) -> Result<FlightPlan<'f>, Error> {

    let mut res: Result<FlightPlan, Error> = records.try_into();

    if let Ok(flightplan) = res.as_mut() {
        flightplan.title = title.to_owned();
        flightplan.uuid = title.to_owned();
    }

    res
}

pub fn from_kml<'m, 'f>(mission: &'m Mission) -> Result<FlightPlan<'f>, Error> {
    mission.try_into()
}

impl<'f> From<&'_ FlightPlan<'f>> for String {

    fn from(flightplan: &FlightPlan<'_>) -> Self {

        if let Ok(res) = serde_json::to_string_pretty(flightplan) {
            res
        }
        else {
            unreachable!()
        }
    }
}

impl<'f> From<&'_ FlightPlan<'f>> for Vec<u8> {

    fn from(flightplan: &FlightPlan<'_>) -> Self {
        String::from(flightplan).into_bytes()
    }
}


pub mod defaults {

    use super::Action;

    pub const _4K_30FPS_RECORDING: Action = Action::VideoStartCapture {
        camera_id: 0,
        resolution: 2073600,
        fps: 30,
    };
}

impl<'f> FlightPlan<'f> {
    fn new(title: &str, latitude: f64, longitude: f64) -> Self {
        let now = Utc::now();

        let title = title.to_owned();
        let uuid = title.to_owned();
        let date = now.timestamp_millis() as u64;

        let takeoff = vec![defaults::_4K_30FPS_RECORDING];

        FlightPlan {
            version: 1,
            product: "ANAFI_4K",
            product_id: 2324,

            title,
            uuid,

            date,

            progressive_course_activated: true,
            dirty: false,

            longitude,
            latitude,

            longitude_delta: 0.0,
            latitude_delta: 0.0,

            zoom_level: 17.0,

            rotation: 0,
            tilt: 0,
            map_type: 4,

            plan: model::Plan {
                takeoff,
                poi: vec![],
                waypoints: vec![],
            },
        }
    }
}

impl PartialEq for PointOfInterest {
    fn eq(&self, other: &Self) -> bool {
        let same_latitude = (other.latitude - self.latitude).abs() < 0.0001;
        let same_longitude = (other.longitude - self.longitude).abs() < 0.0001;
        let same_altitude = (other.altitude - self.altitude) == 0;

        same_latitude && same_longitude && same_altitude
    }
}

impl Eq for PointOfInterest {}

impl Hash for PointOfInterest {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let tuple = (self.latitude.to_bits(), self.longitude.to_bits());

        tuple.hash(state);

        self.altitude.hash(state);
        self.color.hash(state);
    }
}
