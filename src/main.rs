extern crate bronze;

use bronze::{message, server};
//use std::net::SocketAddr;

fn print_message(pkt: &[u8]) -> Option<&[u8]> {
    let msg_result = message::Message::from_bin(pkt);

    match msg_result {
        Ok(msg) => println!("Got Message: {:?}", msg),
        Err(e) => println!("Got Invalid Packet: {}\n{:?}", e, pkt)
    }

    None
}

fn main() {
    let addr = "127.0.0.1:5683".parse().unwrap();
    println!("CoAP Server Listening on {}", addr);
    server::run(addr, print_message);
}
