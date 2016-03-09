extern crate mio;

use message::Message;

use mio::*;
use mio::udp::{UdpSocket};
use std::net::SocketAddr;

const SERVER: Token = Token(0);

struct ServerHandler{
    sock: UdpSocket,
    handler: fn(&[u8]) -> Option<&[u8]>
}

impl  ServerHandler {
    fn new(sock: UdpSocket, handler: fn(&[u8]) -> Option<&[u8]>) -> ServerHandler {
        ServerHandler{
            sock: sock,
            handler: handler
        }
    }
}

impl Handler for ServerHandler {
    type Timeout = ();
    type Message = ();

    fn ready(&mut self, _event_loop: &mut EventLoop<ServerHandler>, token: Token, _: EventSet) {
        match token {
            SERVER => {
                let mut buf: [u8; 2048] = [0; 2048];
                let (len, _addr) = self.sock.recv_from(&mut buf).unwrap().unwrap();

                let pkt = &buf[..len];

                match (self.handler)(pkt) {
                    Some(_) => unimplemented!(),
                    None => ()
                }
            }
            _ => panic!("unexpected token"),
        }
    }

    fn notify(&mut self, _event_loop: &mut EventLoop<Self>, _msg: Self::Message) {
        println!("notify");
    }

    fn interrupted(&mut self, _event_loop: &mut EventLoop<Self>) {
        println!("interrupted");
    }
}

pub fn run(addr: SocketAddr, handler: fn(&[u8]) -> Option<&[u8]>) {
    let server = UdpSocket::bound(&addr).unwrap();

    let mut event_loop = EventLoop::new().unwrap();
    event_loop.register(&server, SERVER, EventSet::readable(), PollOpt::edge()).unwrap();
    event_loop.run(&mut ServerHandler::new(server, handler)).unwrap();
}
