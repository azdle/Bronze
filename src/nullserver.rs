use message::*;
use endpoint::RequestHandler;

use std::net::SocketAddr;

pub struct NullServer;

impl RequestHandler for NullServer {
    fn handle_request(&self, _addr: SocketAddr, in_pkt: &[u8]) -> Option<Vec<u8>> {
        //verify it's a real coap packet
        match Message::from_bin(in_pkt) {
            // is coap, reply with Reset
            // todo: would it be better to use an ack w/ error code?
            Ok(request) => {
                let reply = Message{
                    version: 1,
                    mtype: Mtype::Reset,
                    code: Code::Empty,
                    mid: request.mid,
                    token: request.token.clone(),
                    options: vec![],
                    payload: vec![]
                };

                Some(reply.as_bin().unwrap())
            },
            // not coap, ignore (prevents participating in reflection attacks)
            Err(_) => {
                None
            }
        }
    }
}
