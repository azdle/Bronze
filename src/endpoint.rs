extern crate mio;

use message::Message;

use mio::*;
use mio::udp::{UdpSocket};
use std::net::SocketAddr;

const SERVER: Token = Token(0);

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
        event_loop.run(&mut ServerHandler::new(server, handler)).unwrap();
    }
}

pub trait RequestHandler {
    fn handle_request(&self, &SocketAddr, &Message) -> Option<Vec<u8>>;
}

struct ServerHandler<H>{
    sock: UdpSocket,
    handler: H
}

impl<H: RequestHandler>  ServerHandler<H> {
    fn new(sock: UdpSocket, handler: H) -> ServerHandler<H> {
        ServerHandler{
            sock: sock,
            handler: handler
        }
    }
}

impl<H: RequestHandler> Handler for ServerHandler<H> {
    type Timeout = ();
    type Message = ();

    fn ready(&mut self, _event_loop: &mut EventLoop<ServerHandler<H>>, token: Token, _: EventSet) {
        match token {
            SERVER => {
                let mut buf: [u8; 2048] = [0; 2048];
                let (len, addr) = self.sock.recv_from(&mut buf).unwrap().unwrap();

                let pkt = &buf[..len];

                match Message::from_bytes(pkt) {
                    Ok(msg) => {
                        match (self.handler).handle_request(&addr, &msg) {
                            Some(resp) => {
                                self.sock.send_to(&resp, &addr).unwrap_or(None); // UDP is best-effort, right?
                            },
                            None => ()
                        }
                    },
                    Err(_) => ()
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
