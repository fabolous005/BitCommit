use {
    http::{
        Request,
        Response,
        StatusCode,
    },
    hyper::{
        server::conn::Http,
        service::service_fn,
        Body,
    },
    std::{
        net::SocketAddr,
        convert::Infallible,
    },
    tokio::{
        net::TcpListener,
    }
};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = ([127, 0, 0, 1], 8080).into();

    let mut tcp_listener = TcpListener::bind(addr).await?;
    loop {
        let (tcp_stream, _) = tcp_listener.accept().await?;
        tokio::task::spawn(async move {
            if let Err(http_err) = Http::new()
                    .http1_only(true)
                    .http1_keep_alive(true)
                    .serve_connection(tcp_stream, service_fn(hello))
                    .await {
                eprintln!("Error while serving HTTP connection: {}", http_err);
            }
        });
    }
}

async fn hello(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
   Ok(Response::new(Body::from("Hello World!")))
}
