Bronze
======

[![Build Status](https://travis-ci.org/azdle/bronze.svg?branch=master)](https://travis-ci.org/azdle/bronze)
[![Crates.io Link](http://meritbadge.herokuapp.com/bronze)](https://crates.io/crates/bronze)

Bronze is a [CoAP](https://tools.ietf.org/html/rfc7252) framework generally
aimed at making high performance CoAP servers. It's not currently intended for
use on especially resource constrained devices, but that is a long-term goal.

Bronze is written using mio for all network requests, this means that it has
very low overheads and it should be possible to make it into a very fast and
efficient system.

Status
------

**Bronze is incomplete and you likely shouldn't use it.**

Currently it is possible to create servers that directly deal with incoming
CoAP packets, but there is not yet any automatic handling of retries or
multi-packet messages.

There is not yet any implementation of client requests.

No benchmarks have been done to determine if Bronze in it's current state is as
fast as it could be.


Getting Started
---------------
To use Bronze, add the following to your `Cargo.toml`:

```toml
[dependencies]
bronze = "0.1"
```

Then you'll need to create a run the server, a simple example of this would be:

```rust
extern crate bronze;

use bronze::endpoint::Endpoint;
use bronze::nullserver::NullServer;

fn main() {
    let local_addr = "127.0.0.1:5683".parse().unwrap();
    println!("CoAP Server Listening on {}", local_addr);
    Endpoint::new(local_addr).run(NullServer);
}
```

This example uses the included `NullServer` which is an example of how to write
a request handler. NullServer simply replies to all valid CoAP packets with RST
messages. It's implemented with:

```rust
use message::*;
use endpoint::RequestHandler;

use std::net::SocketAddr;

pub struct NullServer;

impl RequestHandler for NullServer {
    fn handle_request(&self, _addr: &SocketAddr, request: &Message) -> Option<Vec<u8>> {
        println!("{:?}", request);
        // todo: would it be better to use an ack w/ error code?
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
    }
}

```
