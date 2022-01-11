use std::{str::FromStr, num::ParseIntError};

use libmtp_rs::{device::{raw::RawDevice, StorageSort, MtpDevice}, storage::{Parent, files::File}, internals::DeviceEntry, object::Object};

use crate::error::Error;

#[derive(Debug)]
pub enum MtpError {
    InternalFailure(libmtp_rs::error::Error),
    NoMtpDeviceDetected,
    MalformedDeviceId,
    UnableToOpenDevice,
    NoFreeFlight6Folder
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

                _ => Err(MtpError::MalformedDeviceId)
            }
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

                let DeviceEntry { vendor_id, product_id, ..} = device.device_entry();

                if vendor_id == vendor && product_id == product {

                    return Ok(device)
                }
            }

            Err(MtpError::NoMtpDeviceDetected)
        },

        None => devices.next().ok_or(MtpError::NoMtpDeviceDetected),
    }
}


fn find_freeflight6<'a, 'f>(files: &'f [File<'a>]) -> Option<&'f File<'a>> {

    for file in files {
        if file.name() == "FreeFlight 6" {
            return Some(file);
        }
    }
    None
}

pub fn find_device(device: Option<&str>) -> Result<MtpDevice, Error> {

    find_raw_device(device)?
        .open_uncached()
        .ok_or(Error::from(MtpError::UnableToOpenDevice))
}

pub fn find_freeflight6_folder(device: &mut MtpDevice) -> Result<u32, Error> {

    device.update_storage(StorageSort::NotSorted).map_err(MtpError::InternalFailure)?;

    if let Some((_, storage)) = device.storage_pool().iter().next() {

        let root = storage.files_and_folders(Parent::Root);

        let freeflight6_folder = find_freeflight6(root.as_slice())
            .ok_or(MtpError::NoFreeFlight6Folder)?;

        return Ok(freeflight6_folder.id())
    }

    Err(Error::from(MtpError::NoFreeFlight6Folder))
}

pub fn store_flightplan() {
    
}