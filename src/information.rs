
/// The `DeviceInformation` type. Holds the data got from a ata device.
#[derive(Debug)]
pub struct DeviceInformation {
    /// The Seriannumber of the device
    pub serial: String,
    /// The Firmwareversion of the device
    pub firmware_version: String,
    /// The Vendor model name of the device
    pub model: String,
}
impl DeviceInformation {
    /// Decodes a raw device info block
    ///
    /// ```
    /// // BLOCKSIZE may be 512 bytes on most machines
    /// let mut buf: [libc::c_uchar;BLOCKSIZE] = [0;BLOCKSIZE];
    /// // fill the buffer with data
    /// let devinfo = DeviceInformation::new(buf);
    /// ```
    pub fn new(raw: [u8; 512]) -> DeviceInformation {
        let sn: Vec<u8> = raw[20..39].iter().cloned().collect();
        let fw: Vec<u8> = raw[46..53].iter().cloned().collect();
        let mo: Vec<u8> = raw[54..93].iter().cloned().collect();
        DeviceInformation {
            serial: String::from_utf8(sn).expect("connot decode serial"),
            firmware_version: String::from_utf8(fw).expect("connot decode fw version"),
            model: String::from_utf8(mo).expect("connot decode model"),
        }
    }
}
