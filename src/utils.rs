use std::{sync::{Arc, Mutex}, pin::Pin, io, task::Poll};

use tokio::{net::TcpStream, io::{AsyncRead, AsyncWrite, ReadBuf}};

pub struct WraperStream {
    stream: TcpStream,
}
impl WraperStream {
    pub fn new(stream: TcpStream) -> WraperStream {
        WraperStream{ stream: stream}
    }
}
impl AsyncRead for WraperStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let temp_self = &mut *self;
        let temp_stream = &mut temp_self.stream;
        AsyncRead::poll_read(Pin::new(temp_stream), cx, buf)
    }
}
impl AsyncWrite for WraperStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        let temp_self = &mut *self;
        let temp_stream = &mut temp_self.stream;
        AsyncWrite::poll_write(Pin::new(temp_stream), cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), io::Error>> {
        let temp_self = &mut *self;
        let temp_stream = &mut temp_self.stream;
        AsyncWrite::poll_flush(Pin::new(temp_stream), cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), io::Error>> {
        let temp_self = &mut *self;
        let temp_stream = &mut temp_self.stream;
        AsyncWrite::poll_shutdown(Pin::new(temp_stream), cx)
    }
}

#[derive(Clone)]
pub struct SyncSocket {
    stream: Arc<Mutex<WraperStream>>,
}
impl SyncSocket {
    pub fn new(stream: Arc<Mutex<WraperStream>>) -> Self {
        SyncSocket {
            stream: stream
        }
    }
}

impl AsyncRead for SyncSocket {
    fn poll_read(
            self: Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
            buf: &mut ReadBuf<'_>,
        ) -> Poll<io::Result<()>> {
            AsyncRead::poll_read(Pin::new(&mut *self.stream.lock().unwrap()), cx, buf)
    }
}

impl AsyncWrite for SyncSocket {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        AsyncWrite::poll_write(Pin::new(&mut *self.stream.lock().unwrap()), cx, buf)
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), io::Error>> {
        AsyncWrite::poll_flush(Pin::new(&mut *self.stream.lock().unwrap()), cx)
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), io::Error>> {
        AsyncWrite::poll_shutdown(Pin::new(&mut *self.stream.lock().unwrap()), cx)
    }
}

