extern crate rust_server;
extern crate uuid;
extern crate diesel;

use actix::prelude::*;
use actix_web::error::Error;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use self::diesel::prelude::*;
use self::rust_server::*;

use self::models::*;

struct DbExecutor(PgConnection);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

struct CreatePost {
    title: String,
    body: String,
}

impl Message for CreatePost {
    type Result = Result<Post, Error>;
}


pub fn establish_connection() -> PgConnection {
	dotenv().ok();

	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	PgConnection::establish(&database_url).expect(&format!("Error connection to {}", database_url))
}

impl Handler<CreatePost> for DbExecutor {
    type Result = Result<Post, Error>;

    fn handle(&mut self, msg: CreatePost, _: &mut Self::Context) -> Self::Result {
        use rust_server::schema::posts::dsl::*;

        let conn = establish_connection();
        // Create insertion model
        let uuid = format!("{}", uuid::Uuid::new_v4());
        println!("{:?}", uuid);
        let new_post = NewPost {
            title: &msg.title,
            body: "test",
        };

        use schema::posts;
        // normal diesel operations
        diesel::insert_into(posts::table)
            .values(&new_post)
            .get_result(conn)
            .expect("Error inserting person");

        let mut items = posts.
            .filter(published.eq(true))
            .load::<Post>(&conn)
            .expect("Error loading person");

        Ok(items.pop().unwrap())
    }
}

/// This is state where we will store *DbExecutor* address.
struct State {
    db: Addr<DbExecutor>,
}

/// Async handler
fn index(req: &HttpRequest<State>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let name = &req.match_info()["name"];

    // Send message to `DbExecutor` actor
    req.state()
        .db
        .send(CreateUser {
            name: name.to_owned(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn main() {
    let sys = actix::System::new("diesel-example");

    // Start 3 parallel db executors
    let addr = SyncArbiter::start(3, || {
        DbExecutor(SqliteConnection::establish("test.db").unwrap())
    });

    // Start http server
    HttpServer::new(move || {
        App::with_state(State { db: addr.clone() })
            .resource("/{name}", |r| r.method(Method::GET).a(index))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .start()
    .unwrap();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
