use axum::{Router, extract::State, routing::get};
use diesel::*;
use hyper::StatusCode;
use tracing::info;

use crate::{DbPool, models::user::User};

pub fn create_user_router() -> Router<DbPool> {
    return Router::new()
        .route("/", get(user_root));
}


async fn user_root(State(db_pool): State<DbPool>) -> Result<String, (StatusCode, String)> {
    use crate::schema::users::dsl::*;
    let mut db = db_pool.get().unwrap();

    let all_users: Vec<User> = users.load(&mut db).unwrap();
    info!("{:?}", all_users);
    return Ok(String::from("user root"));
}
