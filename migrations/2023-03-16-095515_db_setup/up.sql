-- Your SQL goes here
create table contact_informations (
    id serial primary key,
    email varchar not null,
    phone varchar(15)
);

create table users (
    id serial primary key,
    first_name varchar not null,
    last_name varchar not null,
    password varchar not null,
    contact_information_id serial references contact_informations(id)
);

create table teams (
    id serial primary key,
    team_name varchar not null,
    owner_id serial references users(id),
    contact_information_id serial references contact_informations(id)
);

create table locations (
    id serial primary key,
    plz int not null,
    street_name varchar not null,
    street_number int not null,
    city_name varchar not null,
    team_id serial references teams(id)
);

create table players (
    id serial primary key,
    first_name varchar not null,
    last_name varchar not null,
    team_id serial references teams(id)
);

create table leagues (
    id serial primary key,
    league_name varchar not null,
    owner_id serial references users(id),
    contact_information_id serial references contact_informations(id)
);

create table league_participants (
    id serial primary key,
    rank int not null,
    team_id serial references teams(id),
    league_id serial references leagues(id)
);

create table games (
    id serial primary key,
    date timestamp not null,
    location_id serial references locations(id),
    team_a_id serial references teams(id),
    team_b_id serial references teams(id)
);

create table single_matches (
    id serial primary key,
    player_a_id serial references players(id),
    player_b_id serial references players(id),
    game_id serial references games(id)
);

create table double_matches (
    id serial primary key,
    player_a1_id serial references players(id),
    player_a2_id serial references players(id),
    player_b1_id serial references players(id),
    player_b2_id serial references players(id),
    game_id serial references games(id)
);

create table sets (
    id serial primary key,
    score_a int not null,
    score_b int not null,
    single_match_id serial references single_matches(id),
    double_match_id serial references double_matches(id)
);
