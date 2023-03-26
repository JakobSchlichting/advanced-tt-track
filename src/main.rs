use std::net::SocketAddr;

use axum::{Router, routing::get};
use diesel::{r2d2, PgConnection};
use tracing::{info, error};

mod routes;
mod models;
mod schema;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    // initialise logging
    tracing_subscriber::fmt::init();
    info!("Initialising axum server!");

    // initialise database
    let pg_manager = r2d2::ConnectionManager::<PgConnection>::new("postgres://postgres:test@localhost:5432");
    let pg_pool: DbPool = r2d2::Pool::builder().build(pg_manager).unwrap_or_else(|err| {
        error!("{:?}", err);
        panic!("FATAL: failed to establish database!");
    });


    let app = Router::new()
        .route("/", get(root))
        .nest("/user", routes::user_router::create_user_router())
        .with_state(pg_pool);

    //Start server
    let addr = SocketAddr::from(([0,0,0,0], 8080));
    info!("Server listening on {}", &addr);
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
}

async fn root() -> &'static str {
    return "root";
}
