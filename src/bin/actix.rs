use actix_web::{web, App, HttpResponse, HttpServer, Responder};

/// a request handler
/// an async function that accepts zero or more params that
/// can be extracted from a request (ie, `impl FromRequest`)
///
/// returns a type that can be converted into an
/// `HttpResponse` (ie, `impl Responder`)
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from Actix!")
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
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
    })
    .bind("127.0.0.1:7878")?
    .run()
    .await
}
