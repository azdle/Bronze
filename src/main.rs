extern crate bronze;

use bronze::endpoint::Endpoint;
use bronze::nullhandler::NullHandler;

fn main() {
    let local_addr = "127.0.0.1:5683".parse().unwrap();
    println!("CoAP Server Listening on {}", local_addr);
    Endpoint::new(local_addr).run(NullHandler);
}
