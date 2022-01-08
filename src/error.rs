#[derive(Debug)]
pub enum Error {
    InputOutput(std::io::Error),
    KmlParsingFailed(kml::Error),
    CsvParsingFailed(csv::Error),
    MalformedLitchiMission(&'static str),
    AltitudeOverflow(std::num::IntErrorKind),
    MissingTitle,
    InvalidFileName
}

impl From<kml::Error> for Error {
    fn from(underlying: kml::Error) -> Self {
        match underlying {
            kml::Error::IoError(e) => Error::InputOutput(e),

            _ => Error::KmlParsingFailed(underlying),
        }
    }
}

impl From<csv::Error> for Error {
    fn from(e: csv::Error) -> Self {
        Error::CsvParsingFailed(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(underlying: std::io::Error) -> Self {
        Error::InputOutput(underlying)
    }
}
