use diesel::{Insertable, Queryable, Selectable};
use crate::schema::sets;

#[derive(Debug, Selectable, Queryable, Insertable)]
struct Set {
    id: i32,
    score_a: i32,
    score_b: i32,
    single_match_id: Option<i32>,
    double_match_id: Option<i32>
}
