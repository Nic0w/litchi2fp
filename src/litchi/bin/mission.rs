use serde::{Deserialize, de::{Visitor, self}};

use super::{PointOfInterest, FinishAction, PathMode, waypoint::Waypoint, WaypointPartial, PhotoInterval};

#[derive(Debug)]
pub struct LitchiMission {

    pub finish_action: FinishAction,
    pub path_mode: PathMode,
    pub cruising_speed: f32,
    pub max_speed: f32,
    
    pub waypoints: Vec<Waypoint>,
    pub poi: Vec<PointOfInterest>,

}

#[derive(Deserialize, Debug)]
pub struct LitchiMissionPartial {

    _plop: u32,
    finish_action: FinishAction,
    path_mode: PathMode,
    cruising_speed: f32,
    max_speed: f32,

    _b : [u32; 4], 
    
    waypoints: Vec<WaypointPartial>,
    poi: Vec<PointOfInterest>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct OtherSettings {
    __u32_1: u32,
    __u32_2: u32,
    __u32_3: u32,

    photo_capture_interval_seconds: f32,
    photo_capture_interval_meters: f32,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct WaypointDetails {
    is_above_ground: u16,
    wp_altitude: f32,
    waypoint_poi: u32
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct PoIDetails {
    is_above_ground: u16,
    wp_altitude: f32,
}

struct MissionVisitor;
impl<'de> Visitor<'de> for MissionVisitor {
    type Value = LitchiMission;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a litchi mission file")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>, {
        
            let maybe_magic: u32 = seq.next_element()?
                .ok_or_else(|| de::Error::missing_field("magic"))?;

            if maybe_magic != super::MAGIC {
                let bad_magic = de::Error::invalid_value(
                    de::Unexpected::Unsigned(maybe_magic as u64), &"'lchm' (0x6C63686D)");

                return Err(bad_magic)
            }

            let mission_part: LitchiMissionPartial = seq.next_element()?
                .ok_or_else(|| de::Error::custom("failed to deserialize mission?"))?;

            let nb_waypoints = mission_part.waypoints.len();

            let mut wp_details = Vec::with_capacity(nb_waypoints);

            for i in 0..nb_waypoints {
                let wp_detail: WaypointDetails = seq.next_element()?.
                    ok_or_else(|| de::Error::invalid_length(i as usize, &"more waypoint details"))?;
                wp_details.push(wp_detail);
            }

            for i in 0..mission_part.poi.len() {
                let _poi_detail: PoIDetails = seq.next_element()?.
                    ok_or_else(|| de::Error::invalid_length(i as usize, &"more poi details"))?;
            }

            let _other_settings: OtherSettings = seq.next_element()?
                .ok_or_else(|| de::Error::missing_field("other settings"))?;

            let mut wp_intervals = Vec::with_capacity(nb_waypoints);

            for i in 0..nb_waypoints {

                let interval: (f32, f32) = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(i as usize, &"more intervals"))?;

                wp_intervals.push(interval)
            }
            
            let waypoints = mission_part.waypoints.into_iter()
                .zip(wp_details.iter())
                .zip(wp_intervals.into_iter())
                .map(|((waypoint, details), intervals)| {

                    let interval = PhotoInterval::from_tuple(intervals);

                    let poi = (details.waypoint_poi != 0xFFFFFFFF).then(|| details.waypoint_poi);

                    Waypoint {
                        altitude: waypoint.altitude,
                        heading: waypoint.heading,
                        latitude: waypoint.latitude,
                        longitude: waypoint.longitude,
                        curve_size: waypoint.curve_size,
                        gimbal: waypoint.gimbal,
                        poi,
                        interval,
                        actions: waypoint.actions,
                    }
                }).collect();
            
            let mission = LitchiMission {
                finish_action: mission_part.finish_action,
                path_mode: mission_part.path_mode,
                cruising_speed: mission_part.cruising_speed,
                max_speed: mission_part.max_speed,
                waypoints,
                poi: mission_part.poi,
            };

            Ok(mission)
    }
}

impl<'de> Deserialize<'de> for LitchiMission {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
    
        deserializer.deserialize_tuple(usize::MAX, MissionVisitor)
    }
}

impl PhotoInterval {
    fn from_tuple((time_interval, dist_interval): (f32, f32)) -> Option<Self> {

        if time_interval != -1.0 {
            Some(PhotoInterval::Time { seconds: time_interval })
        } 
        else if dist_interval != -1.0 {
            Some(PhotoInterval::Distance { meters: dist_interval })
        }
        else {
            None
        }
    }
}