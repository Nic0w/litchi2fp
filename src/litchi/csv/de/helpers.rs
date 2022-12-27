use crate::litchi::Action;

use super::{Altitude, Coordinates, GimbalSettings};

#[derive(Debug)]
pub struct ActionHelper {
    pub action_type: Option<i8>,
    pub action_param: Option<isize>,
}

#[derive(Default)]
pub struct GimbalModeHelper {
    pub mode: Option<u8>,
    pub pitch_angle: Option<f64>,
}

#[derive(Default)]
pub struct AltitudeHelper {
    pub mode: Option<u8>,
    pub height: Option<u16>,
}

#[derive(Default)]
pub struct CoordinatesHelper {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,

    pub altitude: AltitudeHelper,
}

pub enum ComposedFieldError {
    MissingParameter,
    UnknownType,
}

impl TryFrom<AltitudeHelper> for Altitude {
    type Error = ComposedFieldError;

    fn try_from(value: AltitudeHelper) -> Result<Self, Self::Error> {
        use ComposedFieldError::*;

        let height = value.height.ok_or(MissingParameter)?;

        match value.mode {
            Some(0) => Ok(Altitude::Absolute(height)),
            Some(1) => Ok(Altitude::AboveGround(height)),

            Some(_) => Err(UnknownType),

            None => Err(MissingParameter),
        }
    }
}

impl TryFrom<CoordinatesHelper> for Coordinates {
    type Error = ComposedFieldError;

    fn try_from(value: CoordinatesHelper) -> Result<Self, Self::Error> {
        use ComposedFieldError::*;

        let latitude = value.latitude.ok_or(MissingParameter)?;

        let longitude = value.longitude.ok_or(MissingParameter)?;

        let altitude = value.altitude.try_into()?;

        Ok(Coordinates {
            latitude,
            longitude,
            altitude,
        })
    }
}

impl TryFrom<GimbalModeHelper> for Option<GimbalSettings> {
    type Error = ComposedFieldError;

    fn try_from(value: GimbalModeHelper) -> Result<Self, Self::Error> {
        use ComposedFieldError::*;

        let result = match value.mode {
            Some(0) => Option::None,

            Some(1) => Some(GimbalSettings::FocusPoi(
                value.pitch_angle.ok_or(MissingParameter)?,
            )),

            Some(2) => Some(GimbalSettings::Interpolate(
                value.pitch_angle.ok_or(MissingParameter)?,
            )),

            Some(_) => return Err(UnknownType),

            None => return Err(MissingParameter),
        };

        Ok(result)
    }
}

impl TryFrom<ActionHelper> for Option<Action> {
    type Error = ComposedFieldError;

    fn try_from(value: ActionHelper) -> Result<Self, Self::Error> {
        use ComposedFieldError::*;

        let result = match value.action_type {
            Some(-1) => Option::None,

            Some(0) => Some(Action::StayFor {
                ms: value.action_param.ok_or(MissingParameter)? as usize,
            }),

            Some(1) => Some(Action::TakePhoto),

            Some(2) => Some(Action::StartRecording),
            Some(3) => Some(Action::StopRecording),

            Some(4) => Some(Action::RotateAircraft {
                angle: value.action_param.ok_or(MissingParameter)? as u16,
            }),

            Some(5) => Some(Action::TiltCamera {
                angle: value.action_param.ok_or(MissingParameter)? as i16,
            }),

            Some(_) => return Err(UnknownType),

            None => return Err(MissingParameter),
        };

        Ok(result)
    }
}
