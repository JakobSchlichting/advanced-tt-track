use diesel::{Insertable, Queryable, Selectable};
use crate::schema::teams;

#[derive(Debug, Selectable, Queryable, Insertable)]
pub struct Team {
    id: i32,
    team_name: String,
    owner_id: i32,
    contact_information_id: i32
}
