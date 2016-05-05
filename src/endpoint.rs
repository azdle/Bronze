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
    event_loop: EventLoop<SocketHandler<MsgHandler>>,
    server: UdpSocket
}

impl Endpoint {
    pub fn new(local_addr: SocketAddr) -> Endpoint {
        let server = UdpSocket::bound(local_addr).unwrap();
        let mut event_loop = EventLoop::new().unwrap();
        event_loop.register(&server, SERVER, EventSet::readable(), PollOpt::edge()).unwrap();

        Endpoint{local_addr: local_addr, event_loop: event_loop}
    }

    pub fn run<H: MsgHandler>(self, handler: H) {
        self.event_loop.run(&mut SocketHandler::new(self.server, handler)).unwrap();
    }
}
