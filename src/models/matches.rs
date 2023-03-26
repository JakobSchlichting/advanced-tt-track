use diesel::{Insertable, Queryable, Selectable};
use crate::schema::{single_matches, double_matches};

#[derive(Debug, Selectable, Queryable, Insertable)]
#[diesel( table_name = single_matches )]
pub struct SingleMatch {
    id: i32,
    player_a_id: i32,
    player_b_id: i32,
    game_id: i32
}

#[derive(Debug, Selectable, Queryable, Insertable)]
#[diesel( table_name = double_matches )]
pub struct DoubleMatch {
    id: i32,
    player_a1_id: i32,
    player_a2_id: i32,
    player_b1_id: i32,
    player_b2_id: i32,
    game_id: i32
}
