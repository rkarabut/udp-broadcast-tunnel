use pcap::Device;

fn main() {
    for device in Device::list() {
        println!("{:?}", device);
    }
}
