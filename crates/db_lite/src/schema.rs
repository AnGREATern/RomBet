// @generated automatically by Diesel CLI.

diesel::table! {
    Bet (id) {
        id -> Text,
        simulation_id -> Text,
        amount -> BigInt,
        coefficient -> Integer,
        game_id -> Text,
        event -> Binary,
        is_won -> Nullable<Bool>,
    }
}

diesel::table! {
    Game (id) {
        id -> Text,
        simulation_id -> Text,
        home_team_id -> Text,
        guest_team_id -> Text,
        round -> BigInt,
    }
}

diesel::table! {
    GameStat (id) {
        id -> Text,
        game_id -> Text,
        home_team_total -> SmallInt,
        guest_team_total -> SmallInt,
    }
}

diesel::table! {
    Simulation (id) {
        id -> Text,
        ip -> Text,
        round -> BigInt,
        balance -> BigInt,
    }
}

diesel::table! {
    Team (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::joinable!(Bet -> Game (game_id));
diesel::joinable!(Bet -> Simulation (simulation_id));
diesel::joinable!(Game -> Simulation (simulation_id));
diesel::joinable!(GameStat -> Game (game_id));

diesel::allow_tables_to_appear_in_same_query!(Bet, Game, GameStat, Simulation, Team,);
