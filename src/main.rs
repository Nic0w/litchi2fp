use std::path::Path;
use std::{ffi::OsStr, fs};

use clap::{ Parser, Subcommand, IntoApp, ErrorKind };
use kml::{ KmlReader};

use crate::{error::Error, litchi::kml::Mission};
use crate::litchi::csv::de::MissionRecord;

mod error;
mod flightplan;
mod litchi;

/// Converts Litchi Mission exports (KML, CSV) to Parrot FreeFlight's JSON format for the FlightPlan feature.
#[derive(Parser, Debug)]
#[clap(about, version, author = "Nic0w")]
struct CommandLineInterface {

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {

    /// To convert a KML file
    FromKml {
        /// Input file
        file: Option<String>,
    },

    /// To convert a CSV file
    FromCsv {
        /// Input file
        file: Option<String>,

        /// Mission name
        #[clap(short, long)]
        title: Option<String>
    }

}

fn main() -> Result<(), Error> {

    let args = CommandLineInterface::parse();

    use Commands::*;

    let output: String = match &args.command {

        FromKml { file: None, .. } | FromCsv { file: None, .. }  => {

            CommandLineInterface::into_app()
                .error(ErrorKind::MissingRequiredArgument, "FILE is required")
                .exit()
        }, 

        FromCsv { file: Some(path) , title} => {

            let path: &Path = Path::new(path);

            let file = fs::File::open(path)?;

            let stem = path.file_stem()
                .and_then(OsStr::to_str);
                            
            let title = title.as_deref().or(stem).unwrap_or_else(|| 
                CommandLineInterface::into_app()
                    .error(ErrorKind::MissingRequiredArgument, "a title is required")
                    .exit()
                );

            let records: Result<Vec<MissionRecord>, _> = csv::Reader::from_reader(file)
                .deserialize()
                .collect();
            
            let records = records?;
            
            let result = flightplan::from_csv(title, records.as_slice())?;

            result.into()
        }

        FromKml { file: Some(path) } => {

            let kml = KmlReader::<_, f64>::from_path(path)?.read()?;

            let mission = Mission::try_from(&kml)?;

            let result = flightplan::from_kml(&mission)?;

            result.into()
        },
    };

    println!("{}", output);

    Ok(())
}
