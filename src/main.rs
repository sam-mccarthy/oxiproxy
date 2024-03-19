mod server;
mod config;
mod misc;

use crate::config::load_config;
use crate::server::run_server_https;

//TODO: Better logging / error handling?
fn main() {
    let proxy_list = load_config();
    if let Err(e) = run_server_https(&proxy_list, proxy_list.domains, proxy_list.port, proxy_list.prod) {
        eprintln!("Failed... {}", e);
        std::process::exit(1);
    }
}