use anyhow::Result;
use rand::seq::SliceRandom;
use rand::{Rng, rng};

use crate::usecase::game::{CreateRound, IGameService, RandomizeRound};
use crate::{
    config::CoefficientConfig,
    repository::{IGameRepo, IGameStatRepo, ITeamRepo},
};
use domain::entity::{Game, GameStat, Team};
use domain::value_object::{Deviation, Id, PastResults, Winner};

const TEAMS_PER_GAME: usize = 2;

pub struct GameService<G: IGameRepo, T: ITeamRepo, S: IGameStatRepo> {
    game_repo: G,
    team_repo: T,
    stat_repo: S,
    round: u32,
    config: CoefficientConfig,
}

impl<G: IGameRepo, T: ITeamRepo, S: IGameStatRepo> CreateRound for GameService<G, T, S> {
    fn create_round(&mut self) -> Result<()> {
        self.round += 1;
        let mut teams = self.team_repo.all_teams_id();
        teams.shuffle(&mut rng());
        for h2h in teams.chunks_exact(TEAMS_PER_GAME) {
            let home_team_id = h2h[0];
            let guest_team_id = h2h[1];
            let game_id = self.game_repo.next_id();
            let game = Game::new(game_id, home_team_id, guest_team_id, self.round);
            self.game_repo.add(game)?;
        }

        Ok(())
    }
}

impl<G: IGameRepo, T: ITeamRepo, S: IGameStatRepo> RandomizeRound for GameService<G, T, S> {
    fn randomize_game(&self, game_id: Id<Game>) -> Result<()> {
        let game = self.game_repo.game_by_id(game_id)?;
        let winner = self.randomize_winner(&game)?;
        let (home_team_total, guest_team_total) = self.randomize_totals(&game, winner)?;
        let stat_id = self.stat_repo.next_id();
        let game_stat = GameStat::new(stat_id, game_id, home_team_total, guest_team_total);
        self.stat_repo.add(game_stat)?;

        Ok(())
    }

    fn randomize_round(&self, round: u32) -> Result<()> {
        let games_id = self.game_repo.games_id_by_round(round)?;
        for game_id in games_id {
            self.randomize_game(game_id)?;
        }

        Ok(())
    }
}

impl<G: IGameRepo, T: ITeamRepo, S: IGameStatRepo> IGameService for GameService<G, T, S> {}

impl<G: IGameRepo, T: ITeamRepo, S: IGameStatRepo> GameService<G, T, S> {
    fn new(game_repo: G, team_repo: T, stat_repo: S, config: CoefficientConfig) -> Self {
        let round = 0;
        Self {
            game_repo,
            team_repo,
            stat_repo,
            round,
            config,
        }
    }

    fn rand_event(probs: &[f64]) -> usize {
        let mut rand_num = rng().random_range(0.0..=probs.iter().sum());
        for (ind, &prob) in probs.iter().enumerate() {
            if prob < rand_num {
                return ind;
            }
            rand_num -= prob;
        }

        probs.len()
    }

    fn past_results_by_team_id(&self, team_id: Id<Team>) -> Result<PastResults> {
        let games_id = self
            .game_repo
            .games_id_by_team_id(team_id, self.config.tracked_games)?;
        let mut past_results = PastResults::new();
        for (game_id, is_home) in games_id {
            let winner = self.stat_repo.winner_by_game_id(game_id, is_home)?;
            past_results.add_result(winner);
        }

        Ok(past_results)
    }

    fn avg_goals_by_team_id(&self, team_id: Id<Team>) -> Result<f64> {
        let games_id = self
            .game_repo
            .games_id_by_team_id(team_id, self.config.tracked_games)?;
        let mut goals = 0;
        for (game_id, is_home) in games_id {
            goals += self.stat_repo.goals_by_game_id(game_id, is_home)?;
        }

        Ok(goals as f64 / self.config.tracked_games as f64)
    }

    fn h2h_results_by_game(&self, game: &Game) -> Result<PastResults> {
        let h2hs_id = self.game_repo.h2hs_id_by_team_id(
            game.home_team_id(),
            game.guest_team_id(),
            self.config.tracked_games,
        )?;
        let mut past_results = PastResults::new();
        for (game_id, is_home) in h2hs_id {
            let winner = self.stat_repo.winner_by_game_id(game_id, is_home)?;
            past_results.add_result(winner);
        }

        Ok(past_results)
    }

    fn h2h_avg_goals_by_game(&self, game: &Game) -> Result<(f64, f64)> {
        let h2hs_id = self.game_repo.h2hs_id_by_team_id(
            game.home_team_id(),
            game.guest_team_id(),
            self.config.tracked_games,
        )?;
        let (mut home_team_goals, mut guest_team_goals) = (0, 0);
        for (game_id, is_home) in h2hs_id {
            let (ht_goals, gt_goals) = self.stat_repo.score_by_game_id(game_id, is_home)?;
            home_team_goals += ht_goals;
            guest_team_goals += gt_goals;
        }

        Ok((
            home_team_goals as f64 / self.config.tracked_games as f64,
            guest_team_goals as f64 / self.config.tracked_games as f64,
        ))
    }

    fn randomize_winner(&self, game: &Game) -> Result<Winner> {
        let home_res = self.past_results_by_team_id(game.home_team_id())?;
        let guest_res = self.past_results_by_team_id(game.guest_team_id())?;
        let h2h_res = self.h2h_results_by_game(game)?;

        let prob_base = (h2h_res.pts_diff() / self.config.alpha) as f64;
        let win_prob = ((((home_res.wins + 1) + (guest_res.loses + 1)) as f64
            / 2.
            / (self.config.tracked_games as u32 + 3) as f64)
            + prob_base)
            * f64::from(Deviation::generate());
        let draw_prob = ((((home_res.draws + 1) + (guest_res.draws + 1)) as f64
            / 2.
            / (self.config.tracked_games as u32 + 3) as f64)
            + prob_base)
            * f64::from(Deviation::generate());
        let lose_prob = ((((home_res.loses + 1) + (guest_res.wins + 1)) as f64
            / 2.
            / (self.config.tracked_games as u32 + 3) as f64)
            + prob_base)
            * f64::from(Deviation::generate());
        let probs = [win_prob, draw_prob, lose_prob];

        Ok(match Self::rand_event(&probs) {
            0 => Winner::W1,
            1 => Winner::X,
            _ => Winner::W2,
        })
    }

    fn randomize_totals(&self, game: &Game, winner: Winner) -> Result<(u8, u8)> {
        let h2h_avg_goals = self.h2h_avg_goals_by_game(game)?;
        let home_team_avg_goals = self.avg_goals_by_team_id(game.home_team_id())? + h2h_avg_goals.0;
        let guest_team_avg_goals =
            self.avg_goals_by_team_id(game.guest_team_id())? + h2h_avg_goals.1;

        let (home_team_goals, guest_team_goals) = match winner {
            Winner::W1 => {
                let rand_home_team_goals = rng()
                    .random_range((home_team_avg_goals - 1.).max(0.)..=(home_team_avg_goals + 1.));
                let home_team_goals = rand_home_team_goals.round_ties_even();
                let rand_guest_team_goals = rng().random_range(
                    (guest_team_avg_goals - 1.).max(0.)
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
                    (guest_team_avg_goals - 1.).max(0.)..=(guest_team_avg_goals + 1.),
                );
                let guest_team_goals = rand_guest_team_goals.round_ties_even();
                let rand_home_team_goals = rng().random_range(
                    (home_team_avg_goals - 1.).max(0.)
                        ..=(home_team_avg_goals + 1.).min(guest_team_goals - 1.),
                );
                let home_team_goals = rand_home_team_goals.round_ties_even() as u8;

                (home_team_goals, guest_team_goals as u8)
            }
        };

        Ok((home_team_goals, guest_team_goals))
    }
}
