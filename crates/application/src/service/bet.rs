use anyhow::Result;
use std::cmp::Ordering;

use crate::{
    config::CoefficientConfig,
    repository::{IBetRepo, IGameRepo, IGameStatRepo},
    usecase::{CalculateBet, MakeBet, MakeReport},
};
use domain::{
    entity::{Bet, Game, Simulation, Team},
    value_object::{
        Amount, BetStatistics, Coefficient, Event, EventTotal, Id, PastResults, PastTotals, Winner,
    },
};

const EPS: f64 = 1e-7;

pub struct BetService<B: IBetRepo, G: IGameRepo, GS: IGameStatRepo> {
    bet_repo: B,
    game_repo: G,
    game_stat_repo: GS,
    config: CoefficientConfig,
}

impl<B: IBetRepo, G: IGameRepo, GS: IGameStatRepo> MakeBet for BetService<B, G, GS> {
    fn make_bet(
        &self,
        game: &Game,
        amount: Amount,
        event: Event,
        coefficient: Coefficient,
    ) -> Result<()> {
        let id = self.bet_repo.next_id();
        let bet = Bet::new(id, amount, coefficient, game.id(), event);
        self.bet_repo.add(bet)?;

        Ok(())
    }

    fn calculate_coefficients(&self, game: &Game) -> Result<Vec<(Event, Coefficient)>> {
        let mut coefficients = self.calculate_winner_coefficients(game)?;
        let mut tc = self.calculate_total_coefficients(game)?;
        coefficients.append(&mut tc);

        Ok(coefficients)
    }

    fn calculate_winner_coefficients(&self, game: &Game) -> Result<Vec<(Event, Coefficient)>> {
        let home_res = self.past_results_by_team_id(game.home_team_id(), game.simulation_id())?;
        let guest_res = self.past_results_by_team_id(game.guest_team_id(), game.simulation_id())?;
        let h2h_res = self.h2h_results_by_game(game)?;

        let prob_base = (h2h_res.pts_diff() / self.config.alpha) as f64;
        let win_prob = (((home_res.wins + 1) + (guest_res.loses + 1)) as f64
            / 2.
            / (self.config.tracked_games as u32 + 3) as f64)
            + prob_base;
        let draw_prob = (((home_res.draws + 1) + (guest_res.draws + 1)) as f64
            / 2.
            / (self.config.tracked_games as u32 + 3) as f64)
            + prob_base;
        let lose_prob = (((home_res.loses + 1) + (guest_res.wins + 1)) as f64
            / 2.
            / (self.config.tracked_games as u32 + 3) as f64)
            + prob_base;
        let mut probs = [win_prob, draw_prob, lose_prob];
        self.normalize(&mut probs);
        probs.map(|p| p * (1. - f64::from(self.config.margin)));

        Ok(vec![
            (Event::WDL(Winner::W1), probs[0].try_into()?),
            (Event::WDL(Winner::X), probs[1].try_into()?),
            (Event::WDL(Winner::W2), probs[2].try_into()?),
        ])
    }

    fn calculate_total_coefficients(&self, game: &Game) -> Result<Vec<(Event, Coefficient)>> {
        let mut total_coefficients = vec![];
        for &total in self.config.totals.iter() {
            let h2h_totals = self.h2h_totals(game, total)?;
            let home_team_past_totals =
                self.past_totals(game.home_team_id(), game.simulation_id(), total)?;
            let guest_team_past_totals =
                self.past_totals(game.guest_team_id(), game.simulation_id(), total)?;

            let totals = ((h2h_totals + home_team_past_totals)? + guest_team_past_totals)?;
            let n = totals.size() as f64 + 3.;
            let tg = n / (totals.greater() as f64 + 1.);
            let te = n / (totals.equal() as f64 + 1.);
            let tl = n / (totals.less() as f64 + 1.);
            let mut probs = [tg, te, tl];
            self.normalize(&mut probs);
            probs.map(|p| p * (1. - f64::from(self.config.margin)));
            total_coefficients.push((
                Event::T(EventTotal {
                    total,
                    ordering: Ordering::Greater,
                }),
                probs[0].try_into()?,
            ));
            total_coefficients.push((
                Event::T(EventTotal {
                    total,
                    ordering: Ordering::Equal,
                }),
                probs[1].try_into()?,
            ));
            total_coefficients.push((
                Event::T(EventTotal {
                    total,
                    ordering: Ordering::Less,
                }),
                probs[2].try_into()?,
            ));
        }

        Ok(total_coefficients)
    }
}

impl<B: IBetRepo, G: IGameRepo, GS: IGameStatRepo> CalculateBet for BetService<B, G, GS> {
    fn calculate_bets(&self) -> Result<f64> {
        let mut profit = 0.;
        let nc_bets = self.bet_repo.not_calculated_bets();
        for bet in nc_bets {
            profit += self.calculate_bet(bet)?;
        }

        Ok(profit)
    }

    fn calculate_bet(&self, mut bet: Bet) -> Result<f64> {
        let profit = match bet.event() {
            Event::WDL(bet_winner) => {
                let winner = self.game_stat_repo.winner_by_game_id(bet.game_id(), true)?;
                if bet_winner == winner {
                    bet.set_win()
                } else {
                    bet.set_lose()
                }
            }
            Event::T(bet_total) => {
                let score = self.game_stat_repo.score_by_game_id(bet.game_id(), true)?;
                let total = score.0 + score.1;
                if bet_total.total.cmp(&total) == bet_total.ordering {
                    bet.set_win()
                } else {
                    bet.set_lose()
                }
            }
        };
        self.bet_repo.update(bet)?;

        Ok(profit)
    }
}

impl<B: IBetRepo, G: IGameRepo, GS: IGameStatRepo> MakeReport for BetService<B, G, GS> {
    fn make_report(&self, start_balance: Amount) -> BetStatistics {
        let min_coefficient_lose = self.bet_repo.min_coefficient_lose();

        BetStatistics::new(start_balance, min_coefficient_lose)
    }
}

impl<B: IBetRepo, G: IGameRepo, GS: IGameStatRepo> BetService<B, G, GS> {
    fn new(bet_repo: B, game_repo: G, game_stat_repo: GS, config: CoefficientConfig) -> Self {
        Self {
            bet_repo,
            game_repo,
            game_stat_repo,
            config,
        }
    }

    fn past_results_by_team_id(
        &self,
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
            let winner = self.game_stat_repo.winner_by_game_id(game_id, is_home)?;
            past_results.add_result(winner);
        }

        Ok(past_results)
    }

    fn past_totals(
        &self,
        team_id: Id<Team>,
        simulation_id: Id<Simulation>,
        total: u8,
    ) -> Result<PastTotals> {
        let games_id = self.game_repo.games_id_by_team_id(
            team_id,
            simulation_id,
            self.config.tracked_games,
        )?;
        let mut past_totals = PastTotals::new(total);
        for (game_id, is_home) in games_id {
            let score = self.game_stat_repo.score_by_game_id(game_id, is_home)?;
            let total = score.0 + score.1;
            past_totals.add_total(total);
        }

        Ok(past_totals)
    }

    fn h2h_results_by_game(&self, game: &Game) -> Result<PastResults> {
        let h2hs_id = self.game_repo.h2hs_id_by_team_id(
            game.home_team_id(),
            game.guest_team_id(),
            game.simulation_id(),
            self.config.tracked_games,
        )?;
        let mut past_results = PastResults::new();
        for (game_id, is_home) in h2hs_id {
            let winner = self.game_stat_repo.winner_by_game_id(game_id, is_home)?;
            past_results.add_result(winner);
        }

        Ok(past_results)
    }

    fn h2h_totals(&self, game: &Game, total: u8) -> Result<PastTotals> {
        let h2hs_id = self.game_repo.h2hs_id_by_team_id(
            game.home_team_id(),
            game.guest_team_id(),
            game.simulation_id(),
            self.config.tracked_games,
        )?;
        let mut past_totals = PastTotals::new(total);
        for (game_id, is_home) in h2hs_id {
            let score = self.game_stat_repo.score_by_game_id(game_id, is_home)?;
            let total = score.0 + score.1;
            past_totals.add_total(total);
        }

        Ok(past_totals)
    }

    fn normalize(&self, probs: &mut [f64]) {
        let sum = probs.iter().sum::<f64>();
        if (sum - 1.).abs() < EPS {
            return;
        }
        for p in probs.iter_mut() {
            *p /= sum;
        }
    }
}
