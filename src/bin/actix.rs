use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;
use std::sync::Mutex;

// This struct represents state
struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

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

async fn index2(req: HttpRequest) -> impl Responder {
    println!("wtf {:?}", req.connection_info());
    /* req */
    // HttpRequest HTTP/1.1 GET:/again
    //     headers:
    //         "accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9"
    //         "dnt": "1"
    //         "sec-fetch-user": "?1"
    //         "accept-language": "en-US,en;q=0.9"
    //         "sec-fetch-mode": "navigate"
    //         "sec-fetch-site": "none"
    //         "accept-encoding": "gzip, deflate, br"
    //         "host": "localhost:7878"
    //         "user-agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_3) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.132 Safari/537.36"
    //         "connection": "keep-alive"
    //         "upgrade-insecure-requests": "1"
    //         "sec-fetch-dest": "document"
    /* req.connection_info() */
    // ConnectionInfo { scheme: "http", host: "localhost:7878", remote: None, peer: Some("127.0.0.1:61989") }
    /* req.connection_info().scheme() */
    // "http"
    /* req.connection_info().host() */
    // "localhost:7878"
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
            .route("/again", web::get().to(index2))
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
