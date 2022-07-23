use kml::{
    types::{Element, Geometry, LineString, Placemark, Point},
    Kml,
};

use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

pub struct Mission<'kml> {
    pub name: &'kml String,
    pub start: &'kml Point,
    pub end: &'kml Point,
    pub path: &'kml LineString,
}

impl<'kml> TryFrom<&'kml Kml> for Mission<'kml> {
    type Error = Error;

    fn try_from(value: &'kml Kml) -> std::result::Result<Self, Self::Error> {
        if let Kml::KmlDocument(kml) = value {
            if let Some(Kml::Document { elements, .. }) = kml.elements.first() {
                parse_document(elements)
            } else {
                Err(Error::MalformedLitchiMission(
                    "Missing main Document: no mission.",
                ))
            }
        } else {
            // kml lib appears to throw a NoElements error if there are no elements.
            unreachable!()
        }
    }
}

fn parse_document(doc: &[Kml]) -> Result<Mission> {
    let mut name: Option<&String> = None;
    let mut start: Option<&Point> = None;
    let mut end: Option<&Point> = None;
    let mut path: Option<&LineString> = None;

    use Error::MalformedLitchiMission;
    use PlaceMarkType::*;

    for elt in doc {
        match elt {
            Kml::Placemark(elt) => match try_parse_placemark(elt) {
                Some(Start(point)) => {
                    start.get_or_insert(point);
                }

                Some(End(point)) => {
                    end.get_or_insert(point);
                }

                Some(Path(line)) => {
                    path.get_or_insert(line);
                }

                None => todo!(),
            },

            Kml::Element(elt) => {
                if name.is_none() {
                    name = try_parse_name(elt);
                }
            }

            _ => (),
        }
    }

    let mission = Mission {
        name: name.ok_or(MalformedLitchiMission("Missing mission name."))?,

        start: start.ok_or(MalformedLitchiMission("Missing mission start."))?,
        end: end.ok_or(MalformedLitchiMission("Missing mission end."))?,

        path: path.ok_or(MalformedLitchiMission("Missing mission path."))?,
    };

    Ok(mission)
}

enum PlaceMarkType<'e> {
    Start(&'e Point),
    End(&'e Point),
    Path(&'e LineString),
}

fn try_parse_placemark(elt: &Placemark) -> Option<PlaceMarkType> {
    use PlaceMarkType::*;

    match &elt.geometry {
        Some(Geometry::LineString(path)) => Some(Path(path)),

        Some(Geometry::Point(point)) => match elt.name.as_deref() {
            Some("Start Point") => Some(Start(point)),
            Some("End Point") => Some(End(point)),

            _ => None,
        },

        _ => None,
    }
}

fn try_parse_name(elt: &Element) -> Option<&String> {
    if elt.name == "name" {
        elt.content.as_ref()
    } else {
        None
    }
}
