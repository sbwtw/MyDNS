
use std::time::Duration;

use trust_dns::op::message::Message;
use trust_dns::op::query::Query;
use trust_dns::rr::resource::Record;
use trust_dns_server::server::*;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::lookup::Lookup;
use trust_dns_resolver::config::*;

pub struct MyDNSHandler {
    resolver: Resolver,
}

impl MyDNSHandler {
    pub fn new() -> MyDNSHandler {

        // 8.8.8.8:53 over tcp
        let google_tcp = NameServerConfig {
                            socket_addr: "8.8.8.8:53".parse().unwrap(),
                            protocol: Protocol::Tcp,
                         };
        let local_dnscrypt = NameServerConfig {
            socket_addr: "127.0.0.1:40".parse().unwrap(),
            protocol: Protocol::Udp,
        };
        let mut res_cfg = ResolverConfig::new();
        res_cfg.add_name_server(google_tcp);
        // res_cfg.add_name_server(local_dnscrypt);
        let mut res_opts = ResolverOpts::default();
        res_opts.timeout = Duration::new(1, 0);
        let res = Resolver::new(res_cfg, res_opts).unwrap();

        MyDNSHandler {
            resolver: res,
        }
    }

    fn lookup(&self, query: &Query) -> Option<Lookup> {
        let addr = query.name().to_string();
        let record_type = query.query_type();

        if let Ok(r) = self.resolver.lookup(&addr, record_type) {
            return Some(r);
        }

        None
    }
}

impl RequestHandler for MyDNSHandler {
    fn handle_request(&self, request: &Request) -> Message {

        let ref msg = request.message;
        let mut response = msg.clone();

        for q in msg.queries() {
            let lookup = self.lookup(&q);
            if lookup.is_none() { continue; }

            let record_type = q.query_type();
            response.add_answers(lookup.unwrap().iter().map(|rdata| {
                Record::from_rdata(q.name().clone(), 100, record_type, rdata.clone())
            }));
        }

        response
    }
}