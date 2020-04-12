#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rust_server;

use self::rust_server::*;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

/* ====================================== */
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;
use std::sync::Mutex;

// This struct represents state
struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

use self::models::{NewPost, Post};
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

extern crate rustc_serialize;
use self::rustc_serialize::json;
#[derive(RustcDecodable, RustcEncodable)]
pub struct MyLittleJSon {
    uri: String,
    query_string: String,
}

async fn again(req: HttpRequest) -> impl Responder {
    println!("head {:?}", req.head());
    println!("uri {:?}", req.uri());
    println!("method {:?}", req.method());
    println!("version {:?}", req.version());
    println!("headers {:?}", req.headers());
    println!("path {:?}", req.path());
    println!("query_string {:?}", req.query_string());
    println!("connection_info {:?}", req.connection_info());
    // let conn = establish_connection();

    // let new_post = NewPost {
    //     title: "test",
    //     body: "testbody",
    // };

    // use schema::posts;
    // let post: Post = diesel::insert_into(posts::table)
    //     .values(&new_post)
    //     .get_result(&conn)
    //     .expect("Error save new post");

    let connection = establish_connection();
    use rust_server::schema::posts::dsl::*;
    let results = posts
        // .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");
    let object = MyLittleJSon {
        uri: format!("{}", req.uri()),
        query_string: format!("{}", results.len()),
    };
    let encoded = json::encode(&object).unwrap();

    // let object = MyLittleJSon {
    //     uri: format!("{}", req.uri()),
    //     query_string: format!("{}", req.query_string()),
    // };
    // let encoded = json::encode(&object).unwrap();
    let res = HttpResponse::Ok()
        .content_type("application/javascript")
        .body(encoded);
    println!("res {:?}", res);
    res
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
            // .data(data.clone()) ??
            .app_data(data.clone())
            .route("/", web::get().to(index))
            .route("/again", web::get().to(again))
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
