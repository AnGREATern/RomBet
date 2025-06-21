use anyhow::{Result, bail};
use rand::{Rng, rng};
use tracing::{debug, info};

use crate::usecase::RandomizeRound;
use crate::{
    config::CoefficientConfig,
    repository::{IGameRepo, IGameStatRepo},
};
use domain::entity::{Game, GameStat, Simulation, Team};
use domain::value_object::{Deviation, Id, PastResults, Winner};

pub struct GameService<G: IGameRepo, GS: IGameStatRepo> {
    game_repo: G,
    game_stat_repo: GS,
    config: CoefficientConfig,
}

impl<G: IGameRepo, GS: IGameStatRepo> RandomizeRound for GameService<G, GS> {
    fn randomize_game(&mut self, game: &Game) -> Result<GameStat> {
        let winner = self.randomize_winner(game)?;
        debug!("Winner randomized");
        let (home_team_total, guest_team_total) = self.randomize_totals(game, winner)?;
        debug!("Score randomized");
        let stat_id = self.game_stat_repo.next_id();
        let game_stat = GameStat::new(stat_id, game.id(), home_team_total, guest_team_total);
        self.game_stat_repo.add(game_stat)?;
        debug!("Game stat added");

        Ok(game_stat)
    }

    fn randomize_round(&mut self, simulation: &Simulation) -> Result<Vec<GameStat>> {
        info!("Checking if last round was randomized");
        self.check_last_round_randomized(simulation.round(), simulation.id())?;
        info!("Last round wasn't randomized");
        let mut games_stat = vec![];
        let games_id = self
            .game_repo
            .games_id_by_round(simulation.round(), simulation.id())?;
        debug!("Got games id");
        for game_id in games_id {
            let game = self.game_repo.game_by_id(game_id)?;
            let game_stat = self.randomize_game(&game)?;
            games_stat.push(game_stat);
        }
        debug!("Round randomized");

        Ok(games_stat)
    }
}

impl<G: IGameRepo, GS: IGameStatRepo> GameService<G, GS> {
    pub fn new(game_repo: G, game_stat_repo: GS, config: CoefficientConfig) -> Self {
        Self {
            game_repo,
            game_stat_repo,
            config,
        }
    }

    fn past_results_by_team_id(
        &mut self,
        team_id: Id<Team>,
        simulation_id: Id<Simulation>,
    ) -> Result<PastResults> {
        let games_id = self.game_repo.games_id_by_team_id(
            team_id,
            simulation_id,
            self.config.tracked_games,
        )?;
        let mut past_results = PastResults::new();
        for (game_id, is_home) in games_id {
            if let Some(winner) = self.game_stat_repo.winner_by_game_id(game_id, is_home) {
                past_results.add_result(winner);
            }
        }

        Ok(past_results)
    }

    fn check_last_round_randomized(&mut self, round: u32, simulation_id: Id<Simulation>) -> Result<()> {
        let games_id = self.game_repo.games_id_by_round(round, simulation_id)?;
        for game_id in games_id {
            if self
                .game_stat_repo
                .game_stat_by_game_id(game_id)
                .ok()
                .is_some()
            {
                bail!("Last round already randomized");
            }
        }

        Ok(())
    }

    fn avg_goals_by_team_id(
        &mut self,
        team_id: Id<Team>,
        simulation_id: Id<Simulation>,
    ) -> Result<f64> {
        let games_id = self.game_repo.games_id_by_team_id(
            team_id,
            simulation_id,
            self.config.tracked_games,
        )?;
        let mut goals = 0;
        for (game_id, is_home) in games_id {
            if let Some(g) = self.game_stat_repo.goals_by_game_id(game_id, is_home) {
                goals += g;
            }
        }

        Ok(goals as f64 / self.config.tracked_games as f64)
    }

    fn h2h_results_by_game(&mut self, game: &Game) -> Result<PastResults> {
        let h2hs_id = self.game_repo.h2hs_id_by_team_id(
            game.home_team_id(),
            game.guest_team_id(),
            game.simulation_id(),
            self.config.tracked_games,
        )?;
        let mut past_results = PastResults::new();
        for (game_id, is_home) in h2hs_id {
            if let Some(winner) = self.game_stat_repo.winner_by_game_id(game_id, is_home) {
                past_results.add_result(winner);
            }
        }

        Ok(past_results)
    }

    fn h2h_avg_goals_by_game(&mut self, game: &Game) -> Result<(f64, f64)> {
        let h2hs_id = self.game_repo.h2hs_id_by_team_id(
            game.home_team_id(),
            game.guest_team_id(),
            game.simulation_id(),
            self.config.tracked_games,
        )?;
        let (mut home_team_goals, mut guest_team_goals) = (0, 0);
        for (game_id, is_home) in h2hs_id {
            if let Some((ht_goals, gt_goals)) = self.game_stat_repo.score_by_game_id(game_id, is_home) {
                home_team_goals += ht_goals;
                guest_team_goals += gt_goals;
            }
        }

        Ok((
            home_team_goals as f64 / self.config.tracked_games as f64,
            guest_team_goals as f64 / self.config.tracked_games as f64,
        ))
    }

    fn randomize_winner(&mut self, game: &Game) -> Result<Winner> {
        let home_res = self.past_results_by_team_id(game.home_team_id(), game.simulation_id())?;
        let guest_res = self.past_results_by_team_id(game.guest_team_id(), game.simulation_id())?;
        let h2h_res = self.h2h_results_by_game(game)?;

        Ok(GameRandomizer::randomize_winner(
            home_res,
            guest_res,
            h2h_res,
            self.config.alpha,
            self.config.tracked_games,
            self.config.deviation_min,
            self.config.deviation_max,
        ))
    }

    fn randomize_totals(&mut self, game: &Game, winner: Winner) -> Result<(u8, u8)> {
        let h2h_avg_goals = self.h2h_avg_goals_by_game(game)?;
        let home_team_avg_goals =
            self.avg_goals_by_team_id(game.home_team_id(), game.simulation_id())? + h2h_avg_goals.0;
        let guest_team_avg_goals = self
            .avg_goals_by_team_id(game.guest_team_id(), game.simulation_id())?
            + h2h_avg_goals.1;

        Ok(GameRandomizer::randomize_totals(
            winner,
            home_team_avg_goals,
            guest_team_avg_goals,
        ))
    }
}

struct GameRandomizer;

impl GameRandomizer {
    fn rand_event(probs: &[f64]) -> usize {
        let mut rand_num = rng().random_range(0.0..=probs.iter().sum());
        for (ind, &prob) in probs.iter().enumerate() {
            if prob > rand_num {
                return ind;
            }
            rand_num -= prob;
        }

        probs.len()
    }

    pub fn randomize_winner(
        home_res: PastResults,
        guest_res: PastResults,
        h2h_res: PastResults,
        alpha: i32,
        tracked_games: u8,
        deviation_min: f64,
        deviation_max: f64,
    ) -> Winner {
        let prob_base = h2h_res.pts_diff() as f64 / alpha as f64;
        let win_prob = ((((home_res.wins + 1) + (guest_res.loses + 1)) as f64
            / 2.
            / (tracked_games as u32 + 3) as f64)
            + prob_base)
            * Deviation::generate(deviation_min, deviation_max).value();
        let draw_prob = (((home_res.draws + 1) + (guest_res.draws + 1)) as f64
            / 2.
            / (tracked_games as u32 + 3) as f64)
            * Deviation::generate(deviation_min, deviation_max).value();
        let lose_prob = ((((home_res.loses + 1) + (guest_res.wins + 1)) as f64
            / 2.
            / (tracked_games as u32 + 3) as f64)
            - prob_base)
            * Deviation::generate(deviation_min, deviation_max).value();
        let probs = [win_prob, draw_prob, lose_prob];

        match Self::rand_event(&probs) {
            0 => Winner::W1,
            1 => Winner::X,
            _ => Winner::W2,
        }
    }

    pub fn randomize_totals(
        winner: Winner,
        home_team_avg_goals: f64,
        guest_team_avg_goals: f64,
    ) -> (u8, u8) {
        let (home_team_goals, guest_team_goals) = match winner {
            Winner::W1 => {
                let rand_home_team_goals = rng()
                    .random_range((home_team_avg_goals - 1.).max(1.)..=(home_team_avg_goals + 1.));
                let home_team_goals = rand_home_team_goals.round_ties_even();
                let rand_guest_team_goals = rng().random_range(
                    (guest_team_avg_goals - 1.)
                        .max(0.)
                        .min(home_team_goals - 1.)
                        ..=(guest_team_avg_goals + 1.).min(home_team_goals - 1.),
                );
                let guest_team_goals = rand_guest_team_goals.round_ties_even() as u8;

                (home_team_goals as u8, guest_team_goals)
            }
            Winner::X => {
                let avg_goals = (home_team_avg_goals + guest_team_avg_goals) / 2.;
                let rand_goals = rng().random_range((avg_goals - 1.).max(0.)..=(avg_goals + 1.));
                let goals = rand_goals.round_ties_even() as u8;

                (goals, goals)
            }
            Winner::W2 => {
                let rand_guest_team_goals = rng().random_range(
                    (guest_team_avg_goals - 1.).max(1.)..=(guest_team_avg_goals + 1.),
                );
                let guest_team_goals = rand_guest_team_goals.round_ties_even();
                let rand_home_team_goals = rng().random_range(
                    (home_team_avg_goals - 1.)
                        .max(0.)
                        .min(guest_team_goals - 1.)
                        ..=(home_team_avg_goals + 1.).min(guest_team_goals - 1.),
                );
                let home_team_goals = rand_home_team_goals.round_ties_even() as u8;

                (home_team_goals, guest_team_goals as u8)
            }
        };

        (home_team_goals, guest_team_goals)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn randomize_totals_winner1() {
        let winner = Winner::W1;
        let home_team_avg_goals = 2.8;
        let guest_team_avg_goals = 3.1;
        let res =
            GameRandomizer::randomize_totals(winner, home_team_avg_goals, guest_team_avg_goals);

        assert!(res.0 > res.1)
    }

    #[test]
    fn randomize_totals_draw() {
        let winner = Winner::X;
        let home_team_avg_goals = 0.1;
        let guest_team_avg_goals = 3.3;
        let res =
            GameRandomizer::randomize_totals(winner, home_team_avg_goals, guest_team_avg_goals);

        assert!(res.0 == res.1)
    }

    #[test]
    fn randomize_totals_winner2() {
        let winner = Winner::W2;
        let home_team_avg_goals = 0.1;
        let guest_team_avg_goals = 3.3;
        let res =
            GameRandomizer::randomize_totals(winner, home_team_avg_goals, guest_team_avg_goals);

        assert!(res.0 < res.1)
    }
}
