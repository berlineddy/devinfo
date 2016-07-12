
extern crate smartdecode;

extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use smartdecode::Device;

#[derive(Debug, RustcDecodable)]
struct Args {
  flag_device: Option<String>
}


const USAGE: &'static str = "Read SMART Values from a device

Usage:
    foo --device DEVICE
    foo --help

Options:
    --device -d DEVICE  read from device
    --help -h           Show this screen.";


fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| Ok(d.help(true)))
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    println!("{:?}", args);
    let device = args.flag_device.unwrap();

    let d = Device::new(&device);
    
    let info = d.read_device_information().unwrap();

    println!("{:#?}", info);
    println!("{:#?}", d.read_device_smart_values());
}
