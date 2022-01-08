use regex::Regex;
use serde::{
    de::{self, Unexpected, Visitor},
    Deserialize,
};

use crate::litchi::csv::de::helpers::CoordinatesHelper;

use super::{
    helpers::{ActionHelper, GimbalModeHelper},
    Action, MissionRecord,
};

const FIELDS: &[&str] = &[
    "latitude",
    "longitude",
    "altitude(m)",
    "heading(deg)",
    "curvesize(m)",
    "rotationdir",
    "gimbalmode",
    "gimbalpitchangle",
    "altitudemode",
    "speed(m/s)",
    "poi_latitude",
    "poi_longitude",
    "poi_altitude(m)",
    "poi_altitudemode",
    "photo_timeinterval",
    "photo_distinterval",
];

enum MissionField {
    Latitude,
    Longitude,
    Altitude,
    Heading,
    CurveSize,
    RotationDir,
    GimbalMode,
    GimbalPitchAngle,
    AlitudeMode,
    Speed,
    PoiLatitude,
    PoiLongitude,
    PoiAltitude,
    PoiAltitudeMode,
    PhotoTimeInterval,
    PhotoDistInterval,
    ActionType(usize),
    ActionParam(usize),
}

impl<'de> Deserialize<'de> for MissionRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct("MissionRecord", FIELDS, MissionRecordVisitor)
    }
}

struct MissionRecordVisitor;

impl<'de> Visitor<'de> for MissionRecordVisitor {
    type Value = MissionRecord;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("litchi mission record!")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        use MissionField::*;

        let mut waypoint_helper = CoordinatesHelper::default();

        let mut heading: Option<f64> = None;
        let mut curvesize: Option<f64> = None;
        let mut rotationdir: Option<u8> = None;

        let mut gimbal_settings = GimbalModeHelper::default();

        let mut speed: Option<f32> = None;

        let mut poi_helper = CoordinatesHelper::default();

        let mut photo_timeinterval: Option<i8> = None;
        let mut photo_distinterval: Option<i8> = None;

        let mut actions_temp = Vec::<ActionHelper>::with_capacity(15);

        while let Some(key) = map.next_key()? {
            match key {
                Latitude => {
                    waypoint_helper.latitude = Some(map.next_value()?);
                }
                Longitude => {
                    waypoint_helper.longitude = Some(map.next_value()?);
                }
                Altitude => {
                    waypoint_helper.altitude.height = Some(map.next_value()?);
                }
                AlitudeMode => {
                    waypoint_helper.altitude.mode = Some(map.next_value()?);
                }

                Heading => {
                    heading = Some(map.next_value()?);
                }
                CurveSize => {
                    curvesize = Some(map.next_value()?);
                }
                RotationDir => {
                    rotationdir = Some(map.next_value()?);
                }

                GimbalMode => {
                    gimbal_settings.mode = Some(map.next_value()?);
                }
                GimbalPitchAngle => {
                    gimbal_settings.pitch_angle = Some(map.next_value()?);
                }

                Speed => {
                    speed = Some(map.next_value()?);
                }

                PoiLatitude => {
                    let value_raw: &str = map.next_value()?;

                    if let Ok(0) = value_raw.parse() {
                        //nothing to do
                        poi_helper.latitude = None;
                    } else {
                        let value_as_float: f64 = value_raw.parse().map_err(|_| {
                            de::Error::invalid_value(Unexpected::Str(value_raw), &"a valid float")
                        })?;

                        poi_helper.latitude = Some(value_as_float);
                    }
                }

                PoiLongitude => {
                    let value_raw: &str = map.next_value()?;

                    if let Ok(0) = value_raw.parse() {
                        //nothing to do
                        poi_helper.longitude = None;
                    } else {
                        let value_as_float: f64 = value_raw.parse().map_err(|_| {
                            de::Error::invalid_value(Unexpected::Str(value_raw), &"a valid float")
                        })?;

                        poi_helper.longitude = Some(value_as_float);
                    }
                }

                PoiAltitude => {
                    poi_helper.altitude.height = Some(map.next_value()?);
                }
                PoiAltitudeMode => {
                    poi_helper.altitude.mode = Some(map.next_value()?);
                }

                PhotoTimeInterval => {
                    photo_timeinterval = Some(map.next_value()?);
                }
                PhotoDistInterval => {
                    photo_distinterval = Some(map.next_value()?);
                }

                ActionType(i) => {
                    if let Some(action) = actions_temp.get_mut(i - 1) {
                        action.action_type = map.next_value()?;
                    } else {
                        actions_temp.insert(
                            i - 1,
                            ActionHelper {
                                action_type: map.next_value()?,
                                action_param: None,
                            },
                        )
                    }
                }
                ActionParam(i) => {
                    if let Some(action) = actions_temp.get_mut(i - 1) {
                        action.action_param = Some(map.next_value()?);
                    }
                }
            }
        }

        let mut actions = Vec::new();

        for action_tmp in actions_temp {
            let action: Option<Action> = Option::try_from(action_tmp)
                .map_err(|_| de::Error::missing_field("action param"))?;

            if let Some(action) = action {
                actions.push(action);
            }
        }

        let waypoint = waypoint_helper
            .try_into()
            .map_err(|_| de::Error::missing_field("waypoint coordinates"))?;

        let poi = match poi_helper {
            CoordinatesHelper {
                latitude: None,
                longitude: None,
                ..
            } => None,

            helper => Some(
                helper
                    .try_into()
                    .map_err(|_| de::Error::missing_field("POI coordinates"))?,
            ),
        };

        let gimbal = gimbal_settings
            .try_into()
            .map_err(|_| de::Error::missing_field("gimbal settings"))?;

        let parsed_record = MissionRecord {
            waypoint,

            heading: heading.ok_or_else(|| de::Error::missing_field("heading"))?,

            curvesize: curvesize.ok_or_else(|| de::Error::missing_field("curvesize"))?,

            rotationdir: rotationdir.ok_or_else(|| de::Error::missing_field("rotationdir"))?,

            gimbal,

            speed: speed.ok_or_else(|| de::Error::missing_field("speed"))?,

            poi,

            photo_timeinterval: photo_timeinterval
                .ok_or_else(|| de::Error::missing_field("photo_timeinterval"))?,

            photo_distinterval: photo_distinterval
                .ok_or_else(|| de::Error::missing_field("photo_distinterval"))?,

            actions,
        };

        Ok(parsed_record)
    }
}

struct MissionFieldVisitor;

impl<'de> Visitor<'de> for MissionFieldVisitor {
    type Value = MissionField;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("litchi CSV fields")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if let Ok(text) = std::str::from_utf8(v) {
            self.visit_str(text)
        } else {
            Err(E::invalid_type(Unexpected::Bytes(v), &"valid utf-8"))
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let re = Regex::new(r"([a-z_]+)([0-9]+)?").unwrap();

        let matches = re.captures(v).unwrap();

        //println!("in MissionFieldVisitor::visit_str: {}, {}", v, &matches[1]);

        let parse_action = |field: &str, variant: fn(usize) -> MissionField| {
            let parsed_index = field.parse::<usize>().map_err(|_| {
                E::invalid_value(Unexpected::Other("bad action index"), &"valid action index")
            })?;

            Ok(variant(parsed_index))
        };

        use MissionField::*;

        match &matches[1] {
            "latitude" => Ok(Latitude),
            "longitude" => Ok(Longitude),
            "altitude" => Ok(Altitude),
            "heading" => Ok(Heading),
            "curvesize" => Ok(CurveSize),
            "rotationdir" => Ok(RotationDir),
            "gimbalmode" => Ok(GimbalMode),
            "gimbalpitchangle" => Ok(GimbalPitchAngle),
            "altitudemode" => Ok(AlitudeMode),
            "speed" => Ok(Speed),
            "poi_latitude" => Ok(PoiLatitude),
            "poi_longitude" => Ok(PoiLongitude),
            "poi_altitude" => Ok(PoiAltitude),
            "poi_altitudemode" => Ok(PoiAltitudeMode),
            "photo_timeinterval" => Ok(PhotoTimeInterval),
            "photo_distinterval" => Ok(PhotoDistInterval),

            "actiontype" => parse_action(&matches[2], ActionType),

            "actionparam" => parse_action(&matches[2], ActionParam),

            _ => {
                println!("blatringue");
                todo!()
            }
        }
    }
}

impl<'de> Deserialize<'de> for MissionField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_identifier(MissionFieldVisitor)
    }
}
