use std::io::BufReader;
use std::net::IpAddr;
use regex::Regex;
use crate::misc::load_file;

pub struct Proxy<'a> {
    url: &'a str,

    forward_auth: bool,

    domain_rule: Regex,
    block_rule: &'a str,
    local_only: bool
}

//TODO: Ponder whether or not this is a good struct name.
// It works, but I feel that it implies that it's strictly a list.
pub struct ProxyList<'a> {
    proxies: Vec<Proxy<'a>>,
    pub(crate) domains: Vec<&'a str>,
    pub(crate) port: u16,
    pub(crate) prod: bool,
}

//TODO: I think some more logging here could do well
//TODO: More flexibility in filename. Maybe .oxi instead?
pub fn load_config() -> ProxyList {
    let config_reader = load_file("oxifile");
    let proxy_list: ProxyList = serde_yaml::from_reader(config_reader)?;
    proxy_list
}