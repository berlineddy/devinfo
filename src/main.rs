extern crate smartdecode;

use smartdecode::Device;



fn main() {
    let d = Device::new("/dev/sda");
    
    let info = d.read_device_information().unwrap();

    println!("{:?}", info);
    println!("{:?}", d.read_device_smart_values());
}
