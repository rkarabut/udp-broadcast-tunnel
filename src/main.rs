use log::debug;
use pcap::{Capture, Device};
use std::{env, net, path::Path, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!(
            "Usage: {} <device> <port> <address1,address2...> [client_ip]",
            Path::new(&args[0]).file_name().unwrap().to_str().unwrap()
        );
        println!("  device: a Windows device name, e.g:");
        println!("    \\Device\\NPF_{{C4BF1F2A-4192-4889-A735-F32D70B97000}}");
        println!("  (or a number from the provided list)");
        println!("  port: the UDP port receiving broadcasts");
        println!("  addresses: the IPv4 addresses to retransmit the broadcast to");
        println!("  client_ip: the IPv4 address of the current client (Warcraft 3 hack)");
        println!();
        println!("Available devices:");
        Device::list()
            .unwrap()
            .into_iter()
            .enumerate()
            .for_each(|(i, d)| println!("{}: {} {}", i, d.name, d.desc.unwrap_or("".into())));
        process::exit(0);
    };

    let devname = args[1].clone();
    let port = args[2].parse::<usize>().expect("Invalid port");
    let addresses: Vec<String> = args[3]
        .split(',')
        .map(|x| format!("{}:{}", x, port))
        .collect();

    // in case the current client address is specified, use it for the Warcraft 3 hack
    let warcraft_bind_address: Option<String> = if args.len() > 4 {
        Some(format!("{0}:{1}", args[4].clone(), port))
    } else {
        None
    };   

    let devices = Device::list().unwrap();

    // try to parse the passed device name as a number,
    // otherwise accept it as a string
    let device = match devname.parse::<usize>().ok() {
        Some(n) => devices.get(n).cloned(),
        None => devices.into_iter().find(|d| d.name == devname),
    }
    .expect("Device not found");

    let mut cap = Capture::from_device(device)
        .unwrap()
        .immediate_mode(true)
        .open()
        .unwrap();
    
    // bind to an ephemeral UDP port
    let bind_address: String = "0.0.0.0:0".to_owned();
    let main_sock = net::UdpSocket::bind(bind_address).unwrap();

    // catch broadcast packets
    cap.filter(&format!("host 255.255.255.255 and udp port {}", port))
        .unwrap();

    loop {
        let packet = cap.next();
        debug!("Received packet: {:?}", packet);
        // actual data starts at byte 42
        let data = &packet.unwrap().data[42..];
        
        for address in addresses.iter() {
            debug!("Resending data to {}: {:?}", address, data);
            match warcraft_bind_address {
                Some(ref bind_address) => {
                    // bind the temporary UDP socket (Warcraft 3 hack)
                    let tmp_sock = net::UdpSocket::bind(bind_address).unwrap();
                    let _ = tmp_sock.send_to(data, address).unwrap();
                }
                None => {
                    // use the ephemeral port
                    let _ = main_sock.send_to(data, address).unwrap();
                }
            }
        }
    }
}
