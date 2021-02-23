extern crate rand;
extern crate pnet;
extern crate synner;
extern crate pnet_base;
extern crate pnet_packet;
extern crate pnet_datalink;
extern crate pnet_transport;

use synner::tcp::packet::{send_tcp_packets};

use std::env;
use std::net::{Ipv4Addr};

fn print_help() {
    println!("Usage: ./synner destination_mac destination_ip destination_port interface_name count");
}


fn parse_arguments() -> Result<(MacAddr, Ipv4Addr, u16, String, u32), &'static str>{
    let args: Vec<String> = env::args().collect();

    if args.len() != 6 {
        println!("Too few arguments. See usage:");
        panic!(print_help());
    }

    let dst_mac = args[1].parse::<MacAddr>().unwrap();
    let dst_ip = args[2].parse::<Ipv4Addr>().unwrap();
    let dst_port = args[3].parse::<u16>().unwrap();
    let iface = args[4].to_string();
    let count = args[5].parse::<u32>().unwrap();
    
    Ok((dst_mac, dst_ip, dst_port, iface, count))
}


fn main() {
    let parsed_args = parse_arguments().unwrap();

    send_tcp_packets(parsed_args.0, parsed_args.1, parsed_args.2, parsed_args.3, parsed_args.4);  

    println!("Sent {} packet(s)", parsed_args.4);
}
