use hyper::service::Service;
use core::task::{Context, Poll};
use core::future::Future;
use std::pin::Pin;
use std::io::Cursor;
use hyper::client::connect::{Connection, Connected};
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Clone)]
pub struct CustomTransporter;

unsafe impl Send for CustomTransporter {}

impl CustomTransporter {
    pub fn new() -> CustomTransporter {
        CustomTransporter{}
    }
}

impl Connection for CustomTransporter {
    fn connected(&self) -> Connected {
        Connected::new()
    }
}

pub struct CustomResponse {
    w: Cursor<Vec<u8>>
}

unsafe impl Send for CustomResponse {
    
}

impl Connection for CustomResponse {
    fn connected(&self) -> Connected {
        println!("connected");
        Connected::new()
    }
}

impl AsyncRead for CustomResponse {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>
    ) -> Poll<std::io::Result<()>> {
        println!("poll_read");
        let r = Pin::new(&mut self.w).poll_read(cx, buf);
        println!("did poll_read");
        /*
        match r {
            Poll::Ready(a) => {
                println!("ready!");
                match a.unwrap() {
                    () => {
                        println!("unwrap ()")
                    }
                }
                panic!("p");
            },
            Poll::Pending =>{ 
                println!("pending!");
                panic!("p");
            }
        }
        */
        r
    }
}

impl AsyncWrite for CustomResponse {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8]
    ) -> Poll<Result<usize, std::io::Error>>{
        //let v = vec!();
        println!("poll_write");
        Pin::new(&mut self.w).poll_write(cx, buf)
    }
    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Result<(), std::io::Error>> {
        println!("poll_flush");
        Pin::new(&mut self.w).poll_flush(cx)
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Result<(), std::io::Error>>
    {
        println!("poll_shutdown");
        Pin::new(&mut self.w).poll_shutdown(cx)
    }
}


impl Service<hyper::Uri> for CustomTransporter {
    type Response = CustomResponse;
    type Error = hyper::http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        println!("poll_ready");
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: hyper::Uri) -> Self::Future {
        println!("call");
        // create the body
        let body: Vec<u8> = "HTTP/1.1 200 OK
Date: Sat, 06 Feb 2021 03:35:23 GMT
Server: Apache/2.4.34 (Amazon) OpenSSL/1.0.2k-fips PHP/5.5.38
Last-Modified: Wed, 23 Dec 2015 01:18:20 GMT
ETag: \"353-527867f65e8ad\"
Accept-Ranges: bytes
Content-Length: 851
Keep-Alive: timeout=5, max=100
Connection: Keep-Alive
Content-Type: text/html; charset=UTF-8\n
hello".as_bytes()
            .to_owned();
        // Create the HTTP response
        let resp = CustomResponse{
            w: Cursor::new(body)
        };
         
        // create a response in a future.
        let fut = async {
            Ok(resp)
        };
        println!("gonna return from call");
        // Return the response as an immediate future
        Box::pin(fut)
    }
}