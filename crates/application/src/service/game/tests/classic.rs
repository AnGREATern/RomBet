use super::super::*;

#[test]
fn randomize_totals_winner1() {
    let winner = Winner::W1;
    let home_team_avg_goals = 2.8;
    let guest_team_avg_goals = 3.1;

    let res = GameRandomizer::randomize_totals(winner, home_team_avg_goals, guest_team_avg_goals);

    assert!(res.0 > res.1)
}

#[test]
fn randomize_totals_draw() {
    let winner = Winner::X;
    let home_team_avg_goals = 0.1;
    let guest_team_avg_goals = 3.3;

    let res = GameRandomizer::randomize_totals(winner, home_team_avg_goals, guest_team_avg_goals);

    assert!(res.0 == res.1)
}

#[test]
fn randomize_totals_winner2() {
    let winner = Winner::W2;
    let home_team_avg_goals = 0.1;
    let guest_team_avg_goals = 3.3;

    let res = GameRandomizer::randomize_totals(winner, home_team_avg_goals, guest_team_avg_goals);

    assert!(res.0 < res.1)
}

#[test]
fn randomize_winner_small_alpha() {
    let home_res = PastResults {
        wins: 10,
        draws: 6,
        loses: 9,
    };
    let guest_res = PastResults {
        wins: 8,
        draws: 7,
        loses: 8,
    };
    let h2h_res = PastResults {
        wins: 8,
        draws: 5,
        loses: 12,
    };
    let alpha = 30;
    let tracked_games = 25;
    let min_deviation = 0.8;
    let max_deviation = 1.2;
    let mut pr = PastResults::new();

    for _ in 1..40 {
        pr.add_result(GameRandomizer::randomize_winner(
            home_res.clone(),
            guest_res.clone(),
            h2h_res.clone(),
            alpha,
            tracked_games,
            min_deviation,
            max_deviation,
        ));
    }
    println!("{}, {}, {}", pr.wins, pr.draws, pr.loses);

    assert!(pr.pts_diff() < 0)
}

#[test]
fn randomize_winner_big_alpha() {
    let home_res = PastResults {
        wins: 15,
        draws: 6,
        loses: 4,
    };
    let guest_res = PastResults {
        wins: 4,
        draws: 7,
        loses: 12,
    };
    let h2h_res = PastResults {
        wins: 8,
        draws: 5,
        loses: 12,
    };
    let alpha = 200;
    let tracked_games = 25;
    let min_deviation = 0.8;
    let max_deviation = 1.2;
    let mut pr = PastResults::new();

    for _ in 1..40 {
        pr.add_result(GameRandomizer::randomize_winner(
            home_res.clone(),
            guest_res.clone(),
            h2h_res.clone(),
            alpha,
            tracked_games,
            min_deviation,
            max_deviation,
        ));
    }
    println!("{}, {}, {}", pr.wins, pr.draws, pr.loses);

    assert!(pr.pts_diff() >= 0)
}
