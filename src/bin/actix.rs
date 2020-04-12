extern crate diesel;
extern crate dotenv;
extern crate rust_server;

use self::rust_server::*;
use serde::{Deserialize, Serialize};

/* ====================================== */
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;
use std::sync::Mutex;

mod get_user_1;
use get_user_1::get_user_by_id;

// This struct represents state
struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

/// This struct represents path params,
/// - Rust/Actix: "/users/{id}"
/// - Node/Express: "/users/:id"
#[derive(Debug, Deserialize, Serialize)]
struct Info {
    id: String,
}
/// extract path info using serde
async fn user_by_id(info: web::Path<Info>) -> String {
    println!("Welcome {}!", info.id);

    let id = &info.id;
    // trim input
    // parse it
    let test = id.trim().parse::<u32>();
    // return if test is not an integer
    match test {
        Ok(ok) => println!("You've specified: {}\n", ok),
        Err(e) => return format!("Error: ({})", e),
    };

    let test = test.unwrap();

    let user = get_user_by_id(test as i32);

    format!("{:#?}", user)
}

use self::models::{NewUser, User};
/// a request handler
/// an async function that accepts zero or more params that
/// can be extracted from a request (ie, `impl FromRequest`)
///
/// returns a type that can be converted into an
/// `HttpResponse` (ie, `impl Responder`)
async fn index(data: web::Data<AppStateWithCounter>) -> impl Responder {
    // HttpResponse::Ok().body("Hello from Actix!")
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {}", counter)
    // May get response:
    // App data is not configured, to configure use App::data()
}

/// - Create an `App` instance and register the request handler with
///   the application's `route` on a _path_ and with a particular
///   _HTTP method_.
/// - After that, the application instance can be used with `HttpServer`
///   to listen for incoming connections.
/// - The server accepts a function that should return an application
///   factory.
///
/// After adding ListenFd and installing `cargo-watch` and `systemfd`
/// Run this app with:
///
/// ```bash
/// systemfd --no-pid -s http::7878 -- cargo watch -x 'run --bin actix'
/// ```   
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .app_data(data.clone())
            .route("/", web::get().to(index))
            // .route("/users/{id}", web::get().to(user_by_id))
            .service(web::scope("/users").route("{id}", web::get().to(user_by_id)))
            .service(web::scope("/app").route("hi", web::get().to(index)))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        let host: &str = "127.0.0.1";
        let port = 7878;
        let url = format!("{}:{}", host, port);
        server.bind(url)?
    };

    server.run().await
}
