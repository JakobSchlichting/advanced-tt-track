use diesel::{Insertable, Queryable, Selectable};
use crate::schema::players;

#[derive(Debug, Selectable, Queryable, Insertable)]
pub struct Player {
    id: i32,
    first_name: String,
    last_name: String,
    team_id: i32
}
