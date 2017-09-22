
extern crate trust_dns;
extern crate trust_dns_server;
extern crate trust_dns_resolver;

use std::net::UdpSocket;

use trust_dns_server::server::*;

mod handler;
use handler::MyDNSHandler;

fn main() {
    let handler = MyDNSHandler::new();

    let mut server = ServerFuture::new(handler).unwrap();
    // server.register_socket(UdpSocket::bind("127.0.0.1:4555").unwrap());
    server.register_socket(UdpSocket::bind("127.0.0.1:53").unwrap());
    server.listen().unwrap();
}
