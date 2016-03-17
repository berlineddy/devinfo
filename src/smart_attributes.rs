use byteorder::{LittleEndian,ByteOrder};

/// The `SmartValueType` holds the different possible Smart attributes
#[derive(Debug)]
pub enum SmartValueType {
    /// ReadErrorRate should be smal! if it is > 0 replace the disc
    ReadErrorRate(SmartValue),
    ///
    ReallocatedSectorsCount(SmartValue),
    /// PowerOnHours holds the overall runtime of the disc
    PowerOnHours(SmartValue),
    /// PowerCycleCount holds how often the device was turned on
    PowerCycleCount(SmartValue),
    ///
    PowerOffRetractCount(SmartValue),
    ///
    ReallocatedEventCount(SmartValue),
    /// Holds the Device Minimal, Maximal und Current Teperature
    TemperatureCelsius(SmartValue),
    /// Holds how many LBA Sectors where written during total lifetime
    TotalLBAWritten(SmartValue),
    /// Holds how many LBA Sectors where read during total lifetime
    TotalLBARead(SmartValue),
    /// every other value
    Unimplemented(u8),
}

/// The SmartValue type holds the actual value for SmartValueType
#[derive(Debug)]
pub struct SmartValue {
    /// indicated how the value should be interpreted (prefail, warning, ...)
    pub flag: u16,
    /// the current health of the value
    pub value: u8,
    /// the raw value payload data (most times a counter)
    pub data: u64,
    /// special! only some smart attributes indicate a worts possible value
    pub worst: u8,
    /// special! only some smart attributes indicate a maximal lifetime value
    pub max: i64,
    /// special! only some smart attributes indicate a minimal lifetime value
    pub min: i64,
    /// special! only some smart attributes indicate a current running value
    pub current: i64,
    /// special! only some smart attributes indicate an initial value
    pub initial: u64,
}
impl SmartValue {
    /// create a new `SmartValue`. this method might be the best to use in most cases. It parses the raw data payload.
    pub fn new_data (raw: &[u8])-> SmartValue {
        let mut data: Vec<u8> = raw[5..12].iter().cloned().collect();
        data.push(0);
        SmartValue {
            flag: LittleEndian::read_u16(&raw[0..2]),
            value: raw[3],
            worst: raw[4],
            data: LittleEndian::read_u64(data.as_slice()),
            min: 0,
            max: 0,
            current: 0,
            initial: 0,
        }
    }
    /// create a new `SmartValue` . This method should be used to parse attributes with a min/max/current payload
    pub fn new_tracker(raw: &[u8]) -> SmartValue {
        SmartValue {
            flag: LittleEndian::read_u16(&raw[0..2]),
            value: 0,
            worst: 0,
            data: 0,
            min: (raw[6] as i8 )as i64,
            max: (raw[7] as i8) as i64,
            current: (raw[5] as i8) as i64,
            initial: 0,
        }
    }
}
