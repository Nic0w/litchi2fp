use std::collections::HashSet;

use crate::{
    error::Error,
    litchi::csv::de::{Coordinates, MissionRecord},
};

use super::{Action, FlightPlan, PointOfInterest, Waypoint};

use crate::litchi::csv::de::Altitude;

use crate::litchi::Action as LitchiAction;

type PoiKey = (u64, u64);

impl<'a, 'f> TryFrom<&'a [MissionRecord]> for FlightPlan<'f> {
    type Error = Error;

    fn try_from(records: &'a [MissionRecord]) -> Result<Self, Self::Error> {
        let poi: HashSet<_> = records
            .iter()
            .filter_map(Option::<PointOfInterest>::from)
            .collect();

        let color_roll = super::POI_COLORS.iter().cycle();

        let mut poi: Vec<_> = poi
            .into_iter()
            .scan(color_roll, |picker, mut poi| {
                if let Some(color) = picker.next() {
                    poi.color = *color;
                }

                Some(poi)
            })
            .collect();

        let mut waypoints: Vec<Waypoint> = Vec::with_capacity(records.len());

        for r in records {
            let mut wp = Waypoint::from(r);

            if let Some::<PoiKey>(key_b) = Option::from(r) {
                for (i, p) in poi.iter().enumerate() {
                    let key_a = PoiKey::from(p);

                    if key_a.eq(&key_b) {
                        wp.poi = Some(i as u8);
                        wp.follow_poi = true;
                    }
                }
            }
            waypoints.push(wp);
        }

        if let Some(last) = waypoints.last_mut() {
            last.actions = Some(vec![Action::VideoStopCapture]);
        }

        let start = waypoints
            .first()
            .ok_or(Error::MalformedLitchiMission("missing start point"))?;

        let latitude = start.latitude;
        let longitude = start.longitude;

        let mut flightplan = FlightPlan::new("", latitude, longitude);

        flightplan.plan.poi.append(&mut poi);

        flightplan.plan.waypoints.append(&mut waypoints);

        Ok(flightplan)
    }
}

impl<'a> From<&'a MissionRecord> for Option<PoiKey> {
    fn from(record: &'a MissionRecord) -> Self {
        record.poi.as_ref().map(
            |Coordinates {
                 latitude,
                 longitude,
                 ..
             }| (latitude.to_bits(), longitude.to_bits()),
        )
    }
}

impl<'a> From<&'a PointOfInterest> for PoiKey {
    fn from(poi: &'a PointOfInterest) -> Self {
        (poi.latitude.to_bits(), poi.longitude.to_bits())
    }
}

impl<'a> From<&'a MissionRecord> for Option<PointOfInterest> {
    fn from(record: &'a MissionRecord) -> Self {
        use Altitude::*;

        record.poi.as_ref().map(
            |Coordinates {
                 latitude,
                 longitude,
                 altitude,
             }| PointOfInterest {
                latitude: *latitude,
                longitude: *longitude,
                altitude: match altitude {
                    AboveGround(x) | Absolute(x) => *x as i16,
                },
                color: 0x32a852,
            },
        )
    }
}

impl<'a, 'w> From<&'a MissionRecord> for Waypoint {
    fn from(rec: &'a MissionRecord) -> Self {
        use Altitude::*;
        use LitchiAction::*;

        let actions: Vec<Action> = rec.actions.iter().map(Action::from).collect();

        let actions = if actions.is_empty() {
            None
        } else {
            Some(actions)
        };

        let speed = match rec.speed.floor() as u8 {
            0 => super::DEFAULT_SPEED_MS,
            x => x,
        };

        Waypoint {
            latitude: rec.waypoint.latitude,
            longitude: rec.waypoint.longitude,
            altitude: match rec.waypoint.altitude {
                AboveGround(x) | Absolute(x) => x,
            },
            yaw: 360.0 - rec.heading,
            speed,
            poi: None,
            dont_stop: true,
            follow_poi: false,
            follow: 1,
            last_yaw: 0f64,
            actions,
        }
    }
}

mod tests {

    #[test]
    fn lol() {
        let a = std::f64::INFINITY;

        let b = a as u8;

        println!("{}, {}", a, b);
    }
}
