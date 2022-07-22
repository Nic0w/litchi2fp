use serde::{de::{Visitor, self}, Deserialize};

#[derive(Debug)]
pub struct Waypoint {
    altitude: f32,
    heading: f32,
    latitude: f64,
    longitude: f64,
    curve_size: f32,
    gimbal_pitch: i32,

    actions: Vec<Action>
}

#[derive(Debug)]
pub enum Action {
    StayFor(u32),
    TakePhoto,
    StartRecording,
    StopRecording,
    RotateAircraft(f32),
    TitltCamera(f32)
}

#[derive(Deserialize, Debug)]
struct WaypointInternal {
    altitude: f32,
    _u32_1: u32,
    heading: f32,
    _u32_2: u32,
    _u32_3: u32,
    latitude: f64,
    longitude: f64,
    curve_size: f32,
    _u32_4: u32,
    gimbal_pitch: i32,
    nb_actions: u32,
    __trash: u32
}

struct WaypointVisitor;
impl<'de> Visitor<'de> for WaypointVisitor {
    type Value = Waypoint;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a Waypoint struct")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>, {

            let wp_internal: WaypointInternal =  seq.next_element()?
                .ok_or_else(|| de::Error::missing_field("waypoint"))?;

            let mut actions = Vec::with_capacity(wp_internal.nb_actions as usize);

            for i in 0..wp_internal.nb_actions {

                let action: (u32, u32) = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(i as usize, &"more actions"))?;

                let action = match action {

                    (0, ms) => Action::StayFor(ms),
                    (1, _) => Action::TakePhoto,
                    (2, _) => Action::StartRecording,
                    (3, _) => Action::StopRecording,
                    (4, angle) => Action::RotateAircraft(f32::from_bits(angle)),
                    (5, angle) => Action::TitltCamera(f32::from_bits(angle)),

                    (code, value) => { panic!("Unknown action with code {} and value: {}", code, value) }
                };

                actions.push(action);
            }

            let waypoint = Waypoint {
                altitude: wp_internal.altitude,
                heading: wp_internal.heading,
                latitude: wp_internal.latitude,
                longitude: wp_internal.longitude,
                curve_size: wp_internal.curve_size,
                gimbal_pitch: wp_internal.gimbal_pitch,
                actions,
            };

            Ok(waypoint)
    }
}

impl<'de> Deserialize<'de> for Waypoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        
        deserializer.deserialize_tuple(usize::MAX, WaypointVisitor)
    }
}