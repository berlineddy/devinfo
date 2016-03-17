extern crate byteorder;
extern crate libc;

pub use self::information::DeviceInformation;
pub use self::smart_attributes::SmartValue;
pub use self::smart_attributes::SmartValueType;

mod information;
mod smart_attributes;

use std::fs::File;
use std::path::Path;
use std::ffi::OsStr;
use std::io;
use std::os::unix::prelude::AsRawFd;

// basic ata defines
const BLOCKSIZE: usize = 512;
const BASICATACMD: usize = 4;

// lunix ioctl syscall
extern "C" {
    fn ioctl(fd: libc::c_int, req: libc::c_ulong, ...) -> libc::c_int;
}

/// The `device` type
pub struct Device<'a> {
    path: &'a Path,
}

impl<'a> Device<'a> {
    /// Create a new device from a path string:
    ///
    /// ```
    /// let d = Device::new("/dev/sda");
    /// ```
    pub fn new<S: AsRef<OsStr> + ?Sized>(fp: &S) -> Device{
        Device {
            path: Path::new(fp),
        }
    }

    /// read the ata device information from the device
    ///
    /// ```
    /// let info = d.read_device_information().unwrap();
    /// ```
    pub fn read_device_information(&self) -> io::Result<DeviceInformation>{
        let mut _ret: i32 = 0;

        let f = try!(File::open(self.path));
        let mut buf: [libc::c_uchar;BLOCKSIZE] = [0;BLOCKSIZE];
        // execute the HDIO_GET_IDENTITY ioctl
        unsafe {_ret = ioctl(f.as_raw_fd(),0x030d,&mut buf);};

        if _ret != 0 {
            return Err(io::Error::from_raw_os_error(_ret))
        }
        Ok(DeviceInformation::new(buf))
    }

    /// read the ata device smart block from the device
    ///
    /// ```
    /// let smart_values = d.read_device_smart_values().unwrap();
    /// ```
    pub fn read_device_smart_values(&self) -> io::Result<Vec<SmartValueType>>{
        let mut _ret: i32 = 0;
        let f = try!(File::open(self.path));
        let mut values: Vec<SmartValueType> = Vec::new();

        // construct the SMART READ VALUE comand
        let mut drive_cmd: [libc::c_uchar;BASICATACMD+BLOCKSIZE] = [0;BASICATACMD+BLOCKSIZE];
        drive_cmd[0] = 0xB0;
        drive_cmd[1] = 0x1;
        drive_cmd[2] = 0xD0;
        drive_cmd[3] = 0x1;

        // execute the HDIO_DRIVE_CMD with prev command
        unsafe {_ret = ioctl(f.as_raw_fd(),0x031f,&drive_cmd);}
        if _ret !=0 {
            return Err(io::Error::from_raw_os_error(_ret))
        }

        // unmap the command from the result
        let buf = &drive_cmd[BASICATACMD..BLOCKSIZE+BASICATACMD];

        for i in 0..30 {
            // map every attribute (12 Bytes)
            let attibute = &buf[(12*i+2)..(12*(i+1)+2)];
            let a = match attibute[0] {
                0x01 => SmartValueType::ReadErrorRate(SmartValue::new_data(attibute)),
                0x05 => SmartValueType::ReallocatedSectorsCount(SmartValue::new_data(attibute)),
                0x09 => SmartValueType::PowerOnHours(SmartValue::new_data(attibute)),
                0x0C => SmartValueType::PowerCycleCount(SmartValue::new_data(attibute)),
                0xC0 => SmartValueType::PowerOffRetractCount(SmartValue::new_data(attibute)),
                0xC2 => SmartValueType::TemperatureCelsius(SmartValue::new_tracker(attibute)),
                0xC4 => SmartValueType::ReallocatedEventCount(SmartValue::new_data(attibute)),
                0xF1 => SmartValueType::TotalLBAWritten(SmartValue::new_data(attibute)),
                0xF2 => SmartValueType::TotalLBARead(SmartValue::new_data(attibute)),
                _ => SmartValueType::Unimplemented(attibute[0]),
            };
            values.push(a);
        }
        Ok(values)
    }
}
