// @generated automatically by Diesel CLI.

diesel::table! {
    contact_informations (id) {
        id -> Int4,
        email -> Varchar,
        phone -> Nullable<Varchar>,
    }
}

diesel::table! {
    double_matches (id) {
        id -> Int4,
        player_a1_id -> Int4,
        player_a2_id -> Int4,
        player_b1_id -> Int4,
        player_b2_id -> Int4,
        game_id -> Int4,
    }
}

diesel::table! {
    games (id) {
        id -> Int4,
        date -> Timestamp,
        location_id -> Int4,
        team_a_id -> Int4,
        team_b_id -> Int4,
    }
}

diesel::table! {
    league_participants (id) {
        id -> Int4,
        rank -> Int4,
        team_id -> Int4,
        league_id -> Int4,
    }
}

diesel::table! {
    leagues (id) {
        id -> Int4,
        league_name -> Varchar,
        owner_id -> Int4,
        contact_information_id -> Int4,
    }
}

diesel::table! {
    locations (id) {
        id -> Int4,
        plz -> Int4,
        street_name -> Varchar,
        street_number -> Int4,
        city_name -> Varchar,
        team_id -> Int4,
    }
}

diesel::table! {
    players (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        team_id -> Int4,
    }
}

diesel::table! {
    sets (id) {
        id -> Int4,
        score_a -> Int4,
        score_b -> Int4,
        single_match_id -> Int4,
        double_match_id -> Int4,
    }
}

diesel::table! {
    single_matches (id) {
        id -> Int4,
        player_a_id -> Int4,
        player_b_id -> Int4,
        game_id -> Int4,
    }
}

diesel::table! {
    teams (id) {
        id -> Int4,
        team_name -> Varchar,
        owner_id -> Int4,
        contact_information_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        password -> Varchar,
        contact_information_id -> Int4,
    }
}

diesel::joinable!(double_matches -> games (game_id));
diesel::joinable!(games -> locations (location_id));
diesel::joinable!(league_participants -> leagues (league_id));
diesel::joinable!(league_participants -> teams (team_id));
diesel::joinable!(leagues -> contact_informations (contact_information_id));
diesel::joinable!(leagues -> users (owner_id));
diesel::joinable!(locations -> teams (team_id));
diesel::joinable!(players -> teams (team_id));
diesel::joinable!(sets -> double_matches (double_match_id));
diesel::joinable!(sets -> single_matches (single_match_id));
diesel::joinable!(single_matches -> games (game_id));
diesel::joinable!(teams -> contact_informations (contact_information_id));
diesel::joinable!(teams -> users (owner_id));
diesel::joinable!(users -> contact_informations (contact_information_id));

diesel::allow_tables_to_appear_in_same_query!(
    contact_informations,
    double_matches,
    games,
    league_participants,
    leagues,
    locations,
    players,
    sets,
    single_matches,
    teams,
    users,
);
