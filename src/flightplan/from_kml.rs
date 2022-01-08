use kml::types::Coord;

use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

use crate::{flightplan::model::Waypoint, litchi::kml::Mission};

pub use super::model::FlightPlan;
use super::Action;

impl<'a, 'm, 'f> TryFrom<&'f Mission<'m>> for FlightPlan<'f> {
    type Error = crate::error::Error;

    fn try_from(mission: &'f Mission) -> std::result::Result<Self, Self::Error> {
        let mut poi = vec![];

        let waypoints: Result<Vec<_>> =
            mission.path.coords.iter().map(Waypoint::try_from).collect();

        let mut waypoints = waypoints?;

        if let Some(last) = waypoints.last_mut() {
            last.actions = Some(vec![Action::VideoStopCapture]);
        }

        let start = waypoints.first().ok_or(Error::MalformedLitchiMission("missing start point"))?;
        
        let latitude = start.latitude;
        let longitude = start.longitude;

        let mut flightplan = FlightPlan::new(mission.name, latitude, longitude);

        flightplan.plan.poi.append(&mut poi);

        flightplan.plan.waypoints.append(&mut waypoints);

        Ok(flightplan)
    }
}

impl<'a, 'w> TryFrom<&'a Coord> for Waypoint {
    type Error = crate::error::Error;

    fn try_from(coord: &'a Coord) -> std::result::Result<Self, Self::Error> {
        let latitude = coord.x;
        let longitude = coord.y;
        let altitude = altitude_checked(coord.z)?;

        let yaw = 0f64;
        let last_yaw = yaw;

        let speed = 5; //Default speed of 5m/s

        let poi = None;
        let follow = 0;

        let follow_poi = false;
        let dont_stop = true;

        let actions = None;

        let waypoint = Waypoint {
            latitude,
            longitude,
            altitude,
            yaw,
            last_yaw,
            speed,
            poi,
            follow_poi,
            dont_stop,
            follow,
            actions,
        };

        Ok(waypoint)
    }
}

fn altitude_checked(altitude: Option<f64>) -> Result<u16> {
    use std::num::IntErrorKind::*;
    use Error::AltitudeOverflow;

    if let Some(alt) = altitude {
        match (alt.is_nan(), alt.is_infinite(), alt.is_sign_negative()) {
            (true, ..) => Err(AltitudeOverflow(InvalidDigit)),

            (_, true, true) => Err(AltitudeOverflow(NegOverflow)),

            (_, true, false) => Err(AltitudeOverflow(PosOverflow)),

            (false, false, ..) => match (1.0f64 > alt, (u16::MAX as f64) < alt) {
                (true, ..) => Err(AltitudeOverflow(Zero)),
                (_, true) => Err(AltitudeOverflow(PosOverflow)),

                (_, _) => Ok(alt as u16),
            },
        }
    } else {
        Ok(super::DEFAULT_WAYPOINT_ALTITUDE_M)
    }
}
