use constants::*;
use message::Message;
use socket_handler::SocketHandler;

use mio::*;
use mio::udp::{UdpSocket};
use std::net::SocketAddr;


pub trait MsgHandler {
    fn handle_msg(&self, &SocketAddr, &Message) -> Option<Vec<u8>>;
}

pub struct Endpoint {
    local_addr: SocketAddr,
}

impl Endpoint {
    pub fn new(local_addr: SocketAddr) -> Endpoint {
        Endpoint{local_addr: local_addr}
    }

    pub fn run<H: MsgHandler>(self, handler: H) {
        let server = UdpSocket::bound(&self.local_addr).unwrap();

        let mut event_loop = EventLoop::new().unwrap();
        event_loop.register(&server, SERVER, EventSet::readable(), PollOpt::edge()).unwrap();
        event_loop.run(&mut SocketHandler::new(server, handler)).unwrap();
    }
}
