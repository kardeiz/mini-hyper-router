#[macro_use]
extern crate mini_hyper_router;
extern crate hyper;

use hyper::server::{Server, Request, Response};
use hyper::method::Method;


fn handle_numbers(_: &Request, res: Response, id: i32) {
    res.send(format!("NUMBER: {}", id).as_bytes()).unwrap();
}

fn main() {

    let server = {

        let host = ::std::env::var("WEB_HOST")
            .unwrap_or("0.0.0.0".into());
        let port = ::std::env::var("WEB_PORT")
            .ok()
            .as_ref()
            .and_then(|x| x.parse().ok() )
            .unwrap_or(3000u16);

        Server::http((&host as &str, port)).unwrap()
    };

    server.handle(|req: Request, mut res: Response| {
        router!(&req, 
            (Method::Get) (/{id: String}.json) => {
                res.send(format!("{{\"id\": \"{}\"}}", id).as_bytes()).unwrap();
                return;
            },
            (Method::Get) (/{id}) => {
                handle_numbers(&req, res, id);
                return;
            },
            (Method::Post) (/about/) => {
                res.send(b"about").unwrap();
                return;
            }
        );
    }).unwrap();


}