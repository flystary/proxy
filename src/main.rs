use anyhow::Result;
// use hyper::server::conn::Http;
use tokio::net::{TcpListener, TcpStream};

mod connection;
mod error;
mod headers;
mod service;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    println!("welcome to coming 7x http proxy");

    let listener = TcpListener::bind("0.0.0.0:8081").await?;

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
           if let core::result::Result::Ok(_) = connection::process(stream).await {

           }
        });
    }
    Ok(())
}

