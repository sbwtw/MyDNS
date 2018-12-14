
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate tokio;
extern crate futures;
extern crate trust_dns;
extern crate trust_dns_server;
extern crate trust_dns_resolver;
extern crate trust_dns_proto;

use std::io;
use std::net::SocketAddr;

use futures::future;
use futures::future::Future;
use tokio::runtime::Runtime;
use tokio::net::UdpSocket;
use trust_dns_server::server::*;

mod handler;

use handler::MyDNSHandler;

fn main() {
    pretty_env_logger::init();

    let addr: SocketAddr = "127.0.0.1:5353".parse().unwrap();
    let socket = UdpSocket::bind(&addr).expect("bind fail");

    let handler = MyDNSHandler::new();
    let server = ServerFuture::new(handler);

    let server_future: Box<Future<Item = (), Error = ()> + Send> =
        Box::new(future::lazy(move || {
            server.register_socket(socket);
            future::empty()
        }));

    tokio::run(server_future.map_err(|e| { error!("{:#?}", e); }));
}
