use diesel::{Insertable, Queryable, Selectable};
use serde::{Serialize, Deserialize};
use crate::schema::users;

#[derive(Serialize, Deserialize, Debug, Selectable, Queryable, Insertable)]
pub struct User {
    id: i32,
    first_name: String,
    last_name: String,
    password: String,
    contact_information_id: i32
}
