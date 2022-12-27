use std::io::Read;
use std::{ffi::OsStr, fs};
use std::{path::Path, vec};

use clap::error::ErrorKind;
use clap::{CommandFactory, Parser, Subcommand};
use flightplan::FlightPlan;
use kml::KmlReader;

use crate::litchi::csv::de::MissionRecord;
use crate::{error::Error, litchi::kml::Mission};

mod error;
mod flightplan;
mod litchi;
mod mtp;

/// Converts Litchi Mission exports (KML, CSV) to Parrot FreeFlight's JSON format for the FlightPlan feature.
#[derive(Parser, Debug)]
#[clap(about, version, author = "Nic0w")]
struct CommandLineInterface {
    #[clap(short, long)]
    store: bool,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// To convert a KML file
    Kml {
        /// Input file
        file: Option<String>,
    },

    /// To convert a CSV file
    Csv {
        /// Input file
        file: Option<String>,

        /// Mission name
        #[clap(short, long)]
        title: Option<String>,
    },

    /// To convert a bin file
    Bin {
        /// Input file
        file: Option<String>,

        /// Mission name
        #[clap(short, long)]
        title: Option<String>,
    },
}

fn main() -> Result<(), Error> {
    let args = CommandLineInterface::parse();

    use Commands::*;

    let output = match &args.command {
        Kml { file: None, .. } | Csv { file: None, .. } | Bin { file: None, .. } => {
            CommandLineInterface::command()
                .error(ErrorKind::MissingRequiredArgument, "FILE is required")
                .exit()
        }

        Csv {
            file: Some(path),
            title,
        } => from_csv(path, title.as_deref()),

        Bin {
            file: Some(path),
            title,
        } => from_bin(path, title.as_deref()),

        Kml { file: Some(path) } => from_kml(path),
    }?;

    if args.store {
        let mut device = mtp::find_device(None)?;

        mtp::store_flightplan(&mut device, &output)?;
    } else {
        println!("{}", String::from(&output));
    }

    Ok(())
}

fn from_csv<'f, P: AsRef<Path> + 'f>(
    path: &'f P,
    title: Option<&str>,
) -> Result<FlightPlan<'f>, Error> {
    let file = fs::File::open(path)?;

    let stem = path.as_ref().file_stem().and_then(OsStr::to_str);

    let title = title.or(stem).unwrap_or_else(|| {
        CommandLineInterface::command()
            .error(ErrorKind::MissingRequiredArgument, "a title is required")
            .exit()
    });

    let records: Result<Vec<MissionRecord>, _> =
        csv::Reader::from_reader(file).deserialize().collect();

    let records = records?;

    let fp = flightplan::from_csv(title, records.as_slice())?;

    Ok(fp)
}

fn from_kml<'f, P: AsRef<Path> + 'f>(path: P) -> Result<FlightPlan<'f>, Error> {
    let kml = KmlReader::<_, f64>::from_path(path)?.read()?;

    let mission = &Mission::try_from(&kml)?;

    flightplan::from_kml(mission)
}

fn from_bin<'f, P: AsRef<Path> + 'f>(
    path: &'f P,
    title: Option<&str>,
) -> Result<FlightPlan<'f>, Error> {
    let mut file = fs::File::open(path)?;

    let stem = path.as_ref().file_stem().and_then(OsStr::to_str);

    let title = title.or(stem).unwrap_or_else(|| {
        CommandLineInterface::command()
            .error(ErrorKind::MissingRequiredArgument, "a title is required")
            .exit()
    });

    let mut data = vec![];

    file.read_to_end(&mut data)?;

    let mission = &litchi::bin::from_slice(&data)?;

    flightplan::from_bin(title, mission)
}
