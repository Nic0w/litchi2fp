use crate::litchi::bin::{LitchiMission, PointOfInterest, WaypointPartial};

use super::{Action, FlightPlan};

impl<'m, 'f> TryFrom<&'_ LitchiMission> for FlightPlan<'f> {
    type Error = crate::error::Error;

    fn try_from(mission: &'_ LitchiMission) -> Result<Self, Self::Error> {
        let mut waypoints = <Vec<super::Waypoint>>::from(mission);

        let color_roll = super::POI_COLORS.iter().cycle();

        let mut poi = mission
            .poi
            .iter()
            .map(super::PointOfInterest::from)
            .scan(color_roll, |picker, mut poi| {
                if let Some(color) = picker.next() {
                    poi.color = *color;
                }

                Some(poi)
            })
            .collect();

        if let Some(last) = waypoints.last_mut() {
            if let Some(actions) = last.actions.as_mut() {
                actions.push(Action::VideoStopCapture);
            } else {
                last.actions = Some(vec![Action::VideoStopCapture]);
            }
        }

        let start = waypoints
            .first()
            .ok_or(Self::Error::MalformedLitchiMission("missing start point"))?;

        let latitude = start.latitude;
        let longitude = start.longitude;

        let mut flightplan = FlightPlan::new("", latitude, longitude);

        flightplan.plan.poi.append(&mut poi);

        flightplan.plan.waypoints.append(&mut waypoints);

        Ok(flightplan)
    }
}

impl From<&'_ LitchiMission> for Vec<super::Waypoint> {
    fn from(mission: &'_ LitchiMission) -> Self {
        mission
            .waypoints
            .iter()
            .map(|w| super::Waypoint {
                latitude: w.latitude,
                longitude: w.longitude,
                altitude: w.altitude as u16,
                yaw: (360.0 - (w.heading as f64).abs()),
                speed: mission.cruising_speed as u8,
                poi: w.poi.map(|v| v as u8),
                dont_stop: true,
                follow_poi: w.poi.is_some(),
                follow: 1,
                last_yaw: 0f64,
                actions: (!w.actions.is_empty())
                    .then(|| w.actions.iter().map(Action::from).collect()),
            })
            .collect()
    }
}

impl From<&'_ PointOfInterest> for super::PointOfInterest {
    fn from(poi: &'_ PointOfInterest) -> Self {
        super::PointOfInterest {
            latitude: poi.latitude,
            longitude: poi.longitude,
            altitude: poi.altitude as i16,
            color: 0x32a852,
        }
    }
}
