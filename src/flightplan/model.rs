use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FlightPlan<'f> {
    pub version: u8,
    pub title: String,
    pub product: &'f str,

    #[serde(rename = "productId")]
    pub product_id: u16,

    pub uuid: String,
    pub date: u64,

    pub progressive_course_activated: bool,
    pub dirty: bool,

    pub longitude: f64,
    pub latitude: f64,

    #[serde(rename = "longitudeDelta")]
    pub longitude_delta: f64,

    #[serde(rename = "latitudeDelta")]
    pub latitude_delta: f64,

    #[serde(rename = "zoomLevel")]
    pub zoom_level: f64,

    pub rotation: u16,
    pub tilt: u16,

    #[serde(rename = "mapType")]
    pub map_type: u8,

    pub plan: Plan,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Plan {
    pub takeoff: Vec<Action>,
    pub poi: Vec<PointOfInterest>,

    #[serde(rename = "wayPoints")]
    pub waypoints: Vec<Waypoint>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PointOfInterest {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: i16,
    pub color: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Waypoint {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: u16,
    pub yaw: f64,
    pub speed: u8,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub poi: Option<u8>,

    #[serde(rename = "continue")]
    pub dont_stop: bool,

    #[serde(rename = "followPOI")]
    pub follow_poi: bool,
    pub follow: u8,

    #[serde(rename = "lastYaw")]
    pub last_yaw: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Action {
    /// Start video recording
    VideoStartCapture {
        /// Camera id
        /// On the ANAFI drone, 0 is the main (and only) camera.
        #[serde(rename = "cameraId")]
        camera_id: u8,

        /// Resolution in pixels.
        /// Strangely the app seems to have a default of `2073600`,
        /// which is 1920x1080.
        /// Recorded videos are still in 4K though (3840x2160)
        resolution: usize,

        /// Frames per second. Usually 30.
        fps: u8,
    },

    /// Stop video recording
    VideoStopCapture,

    /// Start taking pictures
    ImageStartCapture {
        /// Interval in seconds between pictures
        period: usize,

        /// Pictures' resolution
        /// Why is this a float ?
        /// The 3 availables modes in the App give the 3 following values:
        /// DNG =>          14,
        /// JPEG/Rect =>    12.58291244506836,
        /// JPEG/Wide =>    13.600000381469727,
        resolution: f64,

        /// Not sure what this is :/
        #[serde(rename = "nbOfPictures")]
        nb_of_pictures: usize,
    },

    /// Stop taking pictures
    ImageStopCapture,

    /// Hover for the specified amount of time
    Delay {
        /// Duration in seconds of the hover maneuver
        delay: usize,
    },

    /// Set camera's gimbal tilt angle
    Tilt {
        /// In degrees from -90° to +90°
        angle: i8,

        /// Angular speed of the movement in °/s
        speed: u8,
    },

    /// Start a panorama: drone rotates in a hovering position
    Panorama {
        /// In degrees from -360° to +360°
        angle: i8,

        /// Angular speed of the movement in °/s
        /// Max value seems to be 360.
        speed: u8,
    },

    Landing
}
