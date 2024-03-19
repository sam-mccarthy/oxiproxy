use std::net::Ipv4Addr;
use rustls_acme::AcmeConfig;
use rustls_acme::tokio::TokioIncoming;
use tokio_stream::StreamExt;
use crate::config::{Proxy, ProxyList};
use tokio_stream::wrappers::TcpListenerStream;

//TODO: Create a similar function, but for HTTP.
//TODO: Figure out testing methodology.
//TODO: More logging.
pub async fn run_server_https(p_list: &ProxyList, domains: Vec<&str>, port: u16, prod: bool){
    let tcp_listener = tokio::net::TcpListener::bind((Ipv4Addr::UNSPECIFIED, port)).await.unwrap();
    let tcp_incoming = TcpListenerStream::new(tcp_listener);

    //TODO: This probably needs some more parameters.
    let mut tcp_incoming = AcmeConfig::new(domains)
        .directory_lets_encrypt(false)
        .tokio_incoming(tcp_incoming, Vec::new());

    //TODO: Fully develop this.
    while let Some(tls) = tcp_incoming.next().await {
        let mut tls = tls.unwrap();
        tokio::spawn(async move {
           //TODO: Interpret request via ProxyList and regex, then delegate to a reverse_proxy function.
        });
    }

}

//TODO: This function will take a request, interpret it using a ProxyList and regex, and return the respective proxy.
async fn manage_request() -> &Proxy {

}

//TODO: This function will proxy a request between the outside world and the local network.
async fn reverse_proxy(){

}