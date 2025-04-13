// @generated automatically by Diesel CLI.

diesel::table! {
    bet (id) {
        id -> Uuid,
        simulation_id -> Uuid,
        amount -> Int8,
        coefficient -> Int4,
        game_id -> Uuid,
        event -> Bytea,
        is_won -> Nullable<Bool>,
    }
}

diesel::table! {
    game (id) {
        id -> Uuid,
        simulation_id -> Uuid,
        home_team_id -> Uuid,
        guest_team_id -> Uuid,
        round -> Int8,
    }
}

diesel::table! {
    gamestat (id) {
        id -> Uuid,
        game_id -> Uuid,
        home_team_total -> Int2,
        guest_team_total -> Int2,
    }
}

diesel::table! {
    simulation (id) {
        id -> Uuid,
        ip -> Text,
        round -> Int8,
        balance -> Int8,
    }
}

diesel::table! {
    team (id) {
        id -> Uuid,
        name -> Text,
    }
}

diesel::joinable!(bet -> game (game_id));
diesel::joinable!(bet -> simulation (simulation_id));
diesel::joinable!(game -> simulation (simulation_id));
diesel::joinable!(gamestat -> game (game_id));

diesel::allow_tables_to_appear_in_same_query!(
    bet,
    game,
    gamestat,
    simulation,
    team,
);
