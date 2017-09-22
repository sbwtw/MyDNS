
use std::time::Duration;

use trust_dns::op::message::Message;
use trust_dns::rr::record_type::RecordType;
use trust_dns::rr::resource::Record;
use trust_dns_server::server::*;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;

pub struct MyDNSHandler {
    resolver: Resolver,
}

impl MyDNSHandler {
    pub fn new() -> MyDNSHandler {

        let nameserver = NameServerConfig {
                            socket_addr: "8.8.8.8:53".parse().unwrap(),
                            protocol: Protocol::Tcp,
                         };

        let mut res_cfg = ResolverConfig::new();
        res_cfg.add_name_server(nameserver);
        let mut res_opts = ResolverOpts::default();
        res_opts.timeout = Duration::new(1, 0);

        let res = Resolver::new(res_cfg, res_opts).unwrap();


        MyDNSHandler {
            resolver: res,
        }
    }
}

impl RequestHandler for MyDNSHandler {
    fn handle_request(&self, request: &Request) -> Message {

        let ref msg = request.message;
        let mut response = msg.clone();

        for q in msg.queries() {
            let query = q.name().to_string();
            let lookup = self.resolver.lookup(&query, RecordType::A);
            println!("{:?}", lookup);
            if lookup.is_err() { continue; }
            for rdata in lookup.unwrap().iter() {
                println!("{:?}", rdata);
                response.add_answer(Record::from_rdata(q.name().clone(), 360000, RecordType::A, rdata.clone()));
            }
        }

        response
    }
}