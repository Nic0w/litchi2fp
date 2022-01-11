use std::{path::Path, ops::Deref};
use std::{ffi::OsStr, fs};

use clap::{ Parser, Subcommand, IntoApp, ErrorKind };
use kml::{ KmlReader};
use libmtp_rs::{device::{raw::RawDevice, StorageSort}, storage::Parent};

use crate::{error::Error, litchi::kml::Mission};
use crate::litchi::csv::de::MissionRecord;

mod error;
mod flightplan;
mod litchi;
mod mtp;

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
    },

    Mtp

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

        Mtp => {

            let mut first_device = mtp::find_device(None)?;

            let ff6_folder_id = mtp::find_freeflight6_folder(&mut first_device)?;

            //println!("id: {}", ff6_folder_id);

            format!("{:#?}", first_device.storage_pool().files_and_folders(Parent::Folder(ff6_folder_id)))

            //String::default()
        }
    };

    println!("{}", output);

    Ok(())
}
