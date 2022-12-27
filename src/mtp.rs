use std::str::FromStr;

use chrono::TimeZone;
use libmtp_rs::{
    device::{raw::RawDevice, MtpDevice, StorageSort},
    internals::DeviceEntry,
    object::{filetypes::Filetype, Object},
    storage::{
        files::{File, FileMetadata},
        Parent,
    },
    util::HandlerReturn,
};

use crate::{error::Error, flightplan::FlightPlan};

#[derive(Debug)]
pub enum MtpError {
    InternalFailure(libmtp_rs::error::Error),
    NoMtpDeviceDetected,
    MalformedDeviceId,
    UnableToOpenDevice,
    NoFreeFlight6Folder,
    NoStorage,
}

impl From<libmtp_rs::error::Error> for MtpError {
    fn from(underlying: libmtp_rs::error::Error) -> Self {
        MtpError::InternalFailure(underlying)
    }
}

struct DeviceId(u16, u16);

impl FromStr for DeviceId {
    type Err = MtpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((vendor, product)) = s.split_once(':') {
            return match (u16::from_str(vendor), u16::from_str(product)) {
                (Ok(v), Ok(p)) => Ok(DeviceId(v, p)),

                _ => Err(MtpError::MalformedDeviceId),
            };
        }

        Err(MtpError::MalformedDeviceId)
    }
}

fn find_raw_device(device_id: Option<&str>) -> Result<RawDevice, MtpError> {
    let mut devices = libmtp_rs::device::raw::detect_raw_devices()?.into_iter();

    let device_id = device_id.map(DeviceId::from_str).transpose()?;

    match device_id {
        Some(DeviceId(vendor, product)) => {
            for device in devices {
                let DeviceEntry {
                    vendor_id,
                    product_id,
                    ..
                } = device.device_entry();

                if vendor_id == vendor && product_id == product {
                    return Ok(device);
                }
            }

            Err(MtpError::NoMtpDeviceDetected)
        }

        None => devices.next().ok_or(MtpError::NoMtpDeviceDetected),
    }
}

fn find_folder_by_name<'a, 'f>(name: &'_ str, files: &'f [File<'a>]) -> Option<&'f File<'a>> {
    files.iter().find(|&file| file.name() == name)
}

pub fn find_device(device: Option<&str>) -> Result<MtpDevice, Error> {
    find_raw_device(device)?
        .open_uncached()
        .ok_or_else(|| Error::from(MtpError::UnableToOpenDevice))
}

pub fn find_some_folder(
    device: &mut MtpDevice,
    parent: Parent,
    name: &str,
) -> Result<Option<u32>, Error> {
    device
        .update_storage(StorageSort::NotSorted)
        .map_err(MtpError::InternalFailure)?;

    if let Some((_, storage)) = device.storage_pool().iter().next() {
        let root = storage.files_and_folders(parent);

        let folder = find_folder_by_name(name, root.as_slice()).map(File::id);

        return Ok(folder);
    }

    Ok(None)
}

pub fn find_freeflight6_folder(device: &mut MtpDevice) -> Result<u32, Error> {
    find_some_folder(device, Parent::Root, "FreeFlight 6")?
        .ok_or_else(|| Error::from(MtpError::NoFreeFlight6Folder))
}

fn find_flightplan_folder(
    device: &mut MtpDevice,
    ff6_folder_id: u32,
) -> Result<Option<u32>, Error> {
    find_some_folder(device, Parent::Folder(ff6_folder_id), "flightPlan")
}

fn create_folder(device: &mut MtpDevice, parent: Parent, name: &str) -> Result<u32, Error> {
    device
        .update_storage(StorageSort::NotSorted)
        .map_err(MtpError::InternalFailure)?;

    let pool = device.storage_pool();

    let (_, storage) = pool.iter().next().ok_or(MtpError::NoStorage)?;

    let (folder_id, _) = storage
        .create_folder(name, parent)
        .map_err(MtpError::InternalFailure)?;

    Ok(folder_id)
}

pub fn store_flightplan(device: &mut MtpDevice, flightplan: &FlightPlan) -> Result<(), Error> {
    let ff6_folder_id = find_freeflight6_folder(device)?;

    let fp_folder_id = match find_flightplan_folder(device, ff6_folder_id)? {
        Some(id) => id,
        None => create_folder(device, Parent::Folder(ff6_folder_id), "flightPlan")?,
    };

    let dest = create_folder(device, Parent::Folder(fp_folder_id), &flightplan.uuid)?;

    store_flightplan_at(device, dest, flightplan)
}

pub fn store_flightplan_at(
    device: &mut MtpDevice,
    dest_folder_id: u32,
    flightplan: &FlightPlan,
) -> Result<(), Error> {
    let modification_date = chrono::Utc
        .timestamp_millis_opt(flightplan.date as i64)
        .single()
        .unwrap_or_default();

    let mut buffer: Vec<u8> = flightplan.into();

    let metadata = FileMetadata {
        file_size: buffer.len() as u64,
        file_name: "savedPlan.json",
        file_type: Filetype::Text,
        modification_date,
    };

    let handler = |mut data: &mut [u8]| {
        use std::io::Write;

        match data.write(buffer.as_slice()) {
            Ok(n) => {
                buffer.drain(0..n);

                HandlerReturn::Ok(n as u32)
            }
            Err(_) => HandlerReturn::Error,
        }
    };

    let pool = device.storage_pool();

    pool.send_file_from_handler(handler, Parent::Folder(dest_folder_id), metadata)
        .map_err(MtpError::InternalFailure)
        .map_err(Error::from)
        .and(Ok(()))
}
