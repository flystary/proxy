use std::{future::Future, pin::Pin, task::Poll, sync::{Arc, Mutex}};

use futures::FutureExt;
use hyper::{Body, Client, Method, Request, Response};
use tower_service::Service;

use crate::{headers::get_host, connection::State};

#[derive(Clone)]
pub struct ProxyService {
    state: Arc<Mutex<State>>
}

impl ProxyService {
    pub fn new(state: Arc<Mutex<State>>) -> ProxyService {
        ProxyService { state: state }
    }
}
impl Service<Request<Body>> for ProxyService {
    type Response = Response<Body>;

    type Error = Box<dyn std::error::Error + Send + Sync>;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        if req.method() == Method::CONNECT {
            self.clone().http_connect(req).boxed()
        } else {
            self.clone().http_proxy(req).boxed()
        }
    }

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

impl ProxyService {
    async fn http_connect(
        self,
        req: Request<Body>,
    ) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
        let host = get_host(&req)?;
        if let Some(port) = req.uri().port_u16() {
            if port != 443 && port != 80 {
                return Ok(Response::builder()
                    .status(404)
                    .body("Only tunneling of HTTP or HTTPS is supported".into())
                    .unwrap());
            }
        }
       *self.state.lock().unwrap() = State::Tunnel(host);
        Ok(Response::builder().status(200).body(Body::empty()).unwrap())
    }
    async fn http_proxy(
        self,
        req: Request<Body>,
    ) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
        // let mut connector = HttpConnector::new();
        let client = Client::new();
        let response = client.request(req).await;
        response.or_else(|_| Ok(Response::builder().status(200).body(Body::empty()).unwrap()))
    }
}

