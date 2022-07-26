use serde::{
    de::{self, Visitor},
    Deserialize,
};

use crate::litchi::{Action, csv::GimbalSettings};

use super::PhotoInterval;

#[derive(Deserialize, Debug)]
struct WaypointRaw {
    altitude: f32,
    _u32_1: u32,
    heading: f32,
    _u32_2: u32,
    _u32_3: u32,
    latitude: f64,
    longitude: f64,
    curve_size: f32,
    gimbal_settings: (u32, i32),
    nb_actions: u32,
    __trash: u32,
}

#[derive(Debug)]
pub struct WaypointPartial {
    pub altitude: f32,
    pub heading: f32,
    pub latitude: f64,
    pub longitude: f64,
    pub curve_size: f32,

    pub gimbal: Option<GimbalSettings>,

    pub actions: Vec<Action>,
}

impl WaypointPartial {
    fn from_raw(raw: WaypointRaw, actions: Vec<Action>) -> Self {
        WaypointPartial {
            altitude: raw.altitude,
            heading: raw.heading,
            latitude: raw.latitude,
            longitude: raw.longitude,
            curve_size: raw.curve_size,
            gimbal: GimbalSettings::from_tuple(raw.gimbal_settings),
            actions,
        }
    }
}

#[derive(Debug)]
pub struct Waypoint {
    pub altitude: f32,
    pub heading: f32,
    pub latitude: f64,
    pub longitude: f64,
    pub curve_size: f32,
    pub gimbal: Option<GimbalSettings>,

    pub poi: Option<u32>,

    pub interval: Option<PhotoInterval>,

    pub actions: Vec<Action>
}

impl GimbalSettings {
    fn from_tuple((mode, angle): (u32, i32)) -> Option<Self> {
        match mode {
            0x0 => None,

            0x1 => Some(GimbalSettings::FocusPoi(angle as f64)), 

            0x2 => Some(GimbalSettings::Interpolate(angle as f64)), 

            _ => panic!("unknown gimbal settings")
        }
    }
}

struct WaypointVisitor;
impl<'de> Visitor<'de> for WaypointVisitor {
    type Value = WaypointPartial;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a Waypoint struct")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let wp_raw: WaypointRaw = seq
            .next_element()?
            .ok_or_else(|| de::Error::missing_field("waypoint"))?;

        let mut actions = Vec::with_capacity(wp_raw.nb_actions as usize);

        for i in 0..wp_raw.nb_actions {
            let action_tuple: (u32, u32) = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(i as usize, &"more actions"))?;

            let action = Action::from(action_tuple);

            actions.push(action);
        }

        Ok(WaypointPartial::from_raw(wp_raw, actions))
    }
}

impl<'de> Deserialize<'de> for WaypointPartial {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_tuple(usize::MAX, WaypointVisitor)
    }
}

impl From<(u32, u32)> for Action {
    fn from(tuple: (u32, u32)) -> Self {
        match tuple {
            (0, ms) => Action::StayFor { ms: ms as usize },
            (1, _) => Action::TakePhoto,
            (2, _) => Action::StartRecording,
            (3, _) => Action::StopRecording,
            (4, angle) => Action::RotateAircraft {
                angle: angle as u16,
            },
            (5, angle) => Action::TiltCamera {
                angle: angle as i16,
            },

            (code, value) => {
                panic!("Unknown action with code {} and value: {}", code, value)
            }
        }
    }
}