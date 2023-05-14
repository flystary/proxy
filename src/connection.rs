use std::sync::{Arc, Mutex};

use anyhow::{ Result, Ok };
use hyper::server::conn::Http;
use tokio::net::TcpStream;

use crate::{service::ProxyService, utils::{WraperStream, SyncSocket}};

#[derive(Clone, Debug)]
pub enum State {
    Proxy,
    Tunnel(String),
    Mitm(String),
}

pub async fn process(stream: TcpStream) -> Result<()> {
    let w_stream = SyncSocket::new(Arc::new(Mutex::new(WraperStream::new(stream))));
    let state = Arc::new(Mutex::new(State::Proxy));
    let http = Http::new();

    http.serve_connection(w_stream.clone(), ProxyService::new(state.clone())).await?;

    let state = state.lock().unwrap().clone();
    match state {
        State::Proxy => {
            println!("proxy");
        },
        State::Tunnel(host) => {
            println!("host ---> {}", host);

            let mut client_to_server_stream = TcpStream::connect(&host).await?;

            tokio::spawn(async move{
               if let core::result::Result::Ok((_xx, _bb)) = tokio::io::copy_bidirectional(&mut w_stream.clone(), &mut client_to_server_stream).await {

               }
            });

        },
        State::Mitm(_) => {
            println!("mitm");
        },
    }
    Ok(())
}
