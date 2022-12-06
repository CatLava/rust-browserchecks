use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::{convert::Infallible, net::SocketAddr};
use futures::{future, Future};
use regex::Regex;


async fn microservice_handler(req: Request<Body> ) -> Result<Response<Body>, Infallible>  {
    println!("request details: {:?}", req );
    println!("UserAgent: {:?}", req.headers()["user-agent"] );
    let response = {
        match req.uri().path() {
            "/" => {
                Response::new("Hello, World!".into())
            },
            "/ua" => {
                Response::new(format!("UserAgent: {:?}", req.headers()["user-agent"]).into() )
            }
            _ => Response::new("Hello, World! Not implemented".into())
        }
    };
    Ok(response)
}


fn get_os_from_us(user_agent: &str) {
    let re = Regex::new(r"/\([^()]+\)/g").unwrap();
    let txtt = re.find(user_agent).unwrap();
    println!("{:#?}", txtt);
}


#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(move |req| microservice_handler(req)))
    });

    let server = Server::bind(&addr).serve(make_svc);
    println!("server running on,: {}", addr);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
