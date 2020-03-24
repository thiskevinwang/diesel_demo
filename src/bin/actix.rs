use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;
// use std::sync::Mutex;

// This struct represents state
struct AppState {
    app_name: String,
}

/// a request handler
/// an async function that accepts zero or more params that
/// can be extracted from a request (ie, `impl FromRequest`)
///
/// returns a type that can be converted into an
/// `HttpResponse` (ie, `impl Responder`)
async fn index(data: web::Data<AppState>) -> impl Responder {
    // HttpResponse::Ok().body("Hello from Actix!")
    let app_name = &data.app_name;

    format!("Hello {}!", app_name)
    // May get response:
    // App data is not configured, to configure use App::data()
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello again!")
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
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: String::from("MY APP BOI"),
            })
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            .service(web::scope("/app").route("hi", web::get().to(index)))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:7878")?
    };

    server.run().await
}
