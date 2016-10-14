#[macro_use]
extern crate mini_hyper_router;
extern crate hyper;

extern crate num_cpus;

use hyper::server::{Server, Request, Response};
use hyper::method::Method;

fn handle_numbers(_: &Request, res: Response, id: i32) {
    res.send(format!("NUMBER: {}", id).as_bytes()).unwrap();
}

fn handlex(req: Request, mut res: Response) {
    routing! { &req, 
        [Method::Get] (/foo/<id>) => {
            let id: String = id;
            res.send(format!("{{\"id\": \"{}\"}}", id).as_bytes()).unwrap();
            return;
        },
        [Method::Get] (/<id>) => {
            handle_numbers(&req, res, id);
            return;
        },
        [Method::Get] (/assets/<<path>>) => {
            let path: String = path;
            res.send(path.as_bytes());
            return;
        },
        [Method::Post] (/about/) => {
            res.send(b"about").unwrap();
            return;
        }
    };
}



fn main() {

    let server = {        
        use std::time::Duration;

        let host = ::std::env::var("WEB_HOST")
            .unwrap_or("0.0.0.0".into());
        let port = ::std::env::var("WEB_PORT")
            .ok()
            .as_ref()
            .and_then(|x| x.parse().ok() )
            .unwrap_or(3000u16);

        let mut server = Server::http((&host as &str, port)).unwrap();
        server.keep_alive(Some(Duration::from_secs(5)));
        server.set_read_timeout(Some(Duration::from_secs(30)));
        server.set_write_timeout(Some(Duration::from_secs(1)));
        server
    };

    server.handle_threads(|req: Request, mut res: Response| {
        routing! { &req, 
            [Method::Get] (/foo/<id>) => {
                let id: String = id;
                res.send(format!("{{\"id\": \"{}\"}}", id).as_bytes()).unwrap();
                return;
            },
            [Method::Get] (/<id>) => {
                handle_numbers(&req, res, id);
                return;
            },
            [Method::Get] (/assets/<<path>>) => {
                let path: String = path;
                res.send(path.as_bytes());
                return;
            },
            [Method::Post] (/about/) => {
                res.send(b"about").unwrap();
                return;
            }
        };

        *res.status_mut() = hyper::status::StatusCode::NotFound;
        res.send(b"Path not found").unwrap();

    }, 8 * ::num_cpus::get()).unwrap();


}