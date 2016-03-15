extern crate mio;

use constants::*;
use message::Message;
use socket_handler::SocketHandler;

use mio::*;
use mio::udp::{UdpSocket};
use std::net::SocketAddr;


pub trait RequestHandler {
    fn handle_request(&self, &SocketAddr, &Message) -> Option<Vec<u8>>;
}

pub struct Endpoint {
    local_addr: SocketAddr,
}

impl Endpoint {
    pub fn new(local_addr: SocketAddr) -> Endpoint {
        Endpoint{local_addr: local_addr}
    }

    pub fn run<H: RequestHandler>(self, handler: H) {
        let server = UdpSocket::bound(&self.local_addr).unwrap();

        let mut event_loop = EventLoop::new().unwrap();
        event_loop.register(&server, SERVER, EventSet::readable(), PollOpt::edge()).unwrap();
        event_loop.run(&mut SocketHandler::new(server, handler)).unwrap();
    }
}
