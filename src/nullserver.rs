use message::*;
use endpoint::RequestHandler;

use std::net::SocketAddr;

pub struct NullServer;

impl RequestHandler for NullServer {
    fn handle_request(&self, _addr: &SocketAddr, request: &Message) -> Option<Vec<u8>> {
        println!("{:?}", request);
        // todo: would it be better to use an ack w/ error code?
        match request.mtype {
            Mtype::Confirmable | Mtype::NonConfirmable => {
                let reply = Message{
                    version: 1,
                    mtype: Mtype::Reset,
                    code: Code::Empty,
                    mid: request.mid,
                    token: request.token.clone(),
                    options: vec![],
                    payload: vec![]
                };

                Some(reply.to_bytes().unwrap())
            },
            _ => {
                println!("Not replying to message of type: {:?}", request.mtype);
                None
            }
        }
    }
}
