use anyhow::Result;
use std::cmp::Ordering;

use crate::{
    config::CoefficientConfig,
    repository::{IBetRepo, IGameRepo, IGameStatRepo, ISimulationRepo},
    usecase::{CalculateBet, MakeBet, MakeReport},
};
use domain::{
    entity::{Bet, Game, Simulation, Team},
    value_object::{
        Amount, BetStatistics, Coefficient, Event, EventTotal, Id, Margin, PastResults, PastTotals,
        Winner,
    },
};

const EPS: f64 = 1e-7;

pub struct BetService<B: IBetRepo, G: IGameRepo, GS: IGameStatRepo, S: ISimulationRepo> {
    bet_repo: B,
    game_repo: G,
    game_stat_repo: GS,
    simulation_repo: S,
    config: CoefficientConfig,
}

impl<B: IBetRepo, G: IGameRepo, GS: IGameStatRepo, S: ISimulationRepo> MakeBet
    for BetService<B, G, GS, S>
{
    fn make_bet(
        &mut self,
        game: &Game,
        amount: Amount,
        event: Event,
        coefficient: Coefficient,
    ) -> Result<()> {
        let id = self.bet_repo.next_id();
        let simulation_id = game.simulation_id();
        let bet = Bet::new(
            id,
            simulation_id,
            amount,
            coefficient,
            game.id(),
            event,
            None,
        );
        self.bet_repo.add(bet)?;
        let mut simulation = self.simulation_repo.simulation_by_id(simulation_id)?;
        simulation.make_bet(amount)?;
        self.simulation_repo.update_by_id(simulation)?;

        Ok(())
    }

    fn calculate_coefficients(&mut self, game: &Game) -> Result<Vec<(Event, Coefficient)>> {
        let mut coefficients = self.calculate_winner_coefficients(game)?;
        let mut tc = self.calculate_total_coefficients(game)?;
        coefficients.append(&mut tc);

        Ok(coefficients)
    }

    fn calculate_winner_coefficients(&mut self, game: &Game) -> Result<Vec<(Event, Coefficient)>> {
        let home_res = self.past_results_by_team_id(game.home_team_id(), game.simulation_id())?;
        let guest_res = self.past_results_by_team_id(game.guest_team_id(), game.simulation_id())?;
        let h2h_res = self.h2h_results_by_game(game)?;

        BetCalculator::calculate_winner_coefficients(
            home_res,
            guest_res,
            h2h_res,
            self.config.alpha,
            self.config.tracked_games,
            self.config.margin,
        )
    }

    fn calculate_total_coefficients(&mut self, game: &Game) -> Result<Vec<(Event, Coefficient)>> {
        let mut total_coefficients = vec![];
        for &total in self.config.totals.clone().iter() {
            let h2h_totals = self.h2h_totals(game, total)?;
            let home_team_past_totals =
                self.past_totals(game.home_team_id(), game.simulation_id(), total)?;
            let guest_team_past_totals =
                self.past_totals(game.guest_team_id(), game.simulation_id(), total)?;
            let totals = ((h2h_totals + home_team_past_totals)? + guest_team_past_totals)?;

            let mut ttl =
                BetCalculator::calculate_total_coefficients(total, totals, self.config.margin)?;
            total_coefficients.append(&mut ttl);
        }

        Ok(total_coefficients)
    }
}

impl<B: IBetRepo, G: IGameRepo, GS: IGameStatRepo, S: ISimulationRepo> CalculateBet
    for BetService<B, G, GS, S>
{
    fn calculate_bets(&mut self) -> Result<Amount> {
        let mut profit = 0;
        let nc_bets = self.bet_repo.not_calculated_bets();
        if let Some(bet) = nc_bets.get(0) {
            let mut simulation = self.simulation_repo.simulation_by_id(bet.simulation_id())?;
            for bet in nc_bets {
                profit += self.calculate_bet(bet, &mut simulation)?.clear_value();
            }
            self.simulation_repo.update_by_id(simulation)?;
        }

        Ok(Amount::new(profit, None).unwrap())
    }

    fn calculate_bet(&mut self, mut bet: Bet, simulation: &mut Simulation) -> Result<Amount> {
        let profit = match bet.event() {
            Event::WDL(bet_winner) => {
                if let Some(winner) = self.game_stat_repo.winner_by_game_id(bet.game_id(), true) {
                    if bet_winner == winner {
                        bet.set_win()
                    } else {
                        bet.set_lose()
                    }
                } else {
                    Amount::new(0, None).unwrap()
                }
            }
            Event::T(bet_total) => {
                if let Some(score) = self.game_stat_repo.score_by_game_id(bet.game_id(), true) {
                    let total = score.0 + score.1;
                    if total.cmp(&bet_total.total) == bet_total.ordering {
                        bet.set_win()
                    } else {
                        bet.set_lose()
                    }
                } else {
                    Amount::new(0, None).unwrap()
                }
            }
        };
        if bet.is_won().unwrap() {
            simulation.process_bet(profit)?;
        }
        self.bet_repo.update_status(bet)?;

        Ok(profit)
    }
}

impl<B: IBetRepo, G: IGameRepo, GS: IGameStatRepo, S: ISimulationRepo> MakeReport
    for BetService<B, G, GS, S>
{
    fn make_report(&mut self, start_balance: Amount) -> BetStatistics {
        let min_coefficient_lose = self.bet_repo.min_coefficient_lose();

        BetStatistics::new(start_balance, min_coefficient_lose)
    }
}

impl<B: IBetRepo, G: IGameRepo, GS: IGameStatRepo, S: ISimulationRepo> BetService<B, G, GS, S> {
    pub fn new(
        bet_repo: B,
        game_repo: G,
        game_stat_repo: GS,
        simulation_repo: S,
        config: CoefficientConfig,
    ) -> Self {
        Self {
            bet_repo,
            game_repo,
            game_stat_repo,
            simulation_repo,
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

    fn past_totals(
        &mut self,
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
            if let Some(score) = self.game_stat_repo.score_by_game_id(game_id, is_home) {
                let total = score.0 + score.1;
                past_totals.add_total(total);
            }
        }

        Ok(past_totals)
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

    fn h2h_totals(&mut self, game: &Game, total: u8) -> Result<PastTotals> {
        let h2hs_id = self.game_repo.h2hs_id_by_team_id(
            game.home_team_id(),
            game.guest_team_id(),
            game.simulation_id(),
            self.config.tracked_games,
        )?;
        let mut past_totals = PastTotals::new(total);
        for (game_id, is_home) in h2hs_id {
            if let Some(score) = self.game_stat_repo.score_by_game_id(game_id, is_home) {
                let total = score.0 + score.1;
                past_totals.add_total(total);
            }
        }

        Ok(past_totals)
    }
}

struct BetCalculator;

impl BetCalculator {
    fn normalize(probs: &mut [f64]) {
        let sum = probs.iter().sum::<f64>();
        if (sum - 1.).abs() < EPS {
            return;
        }
        for p in probs.iter_mut() {
            *p /= sum;
        }
    }

    pub fn calculate_winner_coefficients(
        home_res: PastResults,
        guest_res: PastResults,
        h2h_res: PastResults,
        alpha: i32,
        tracked_games: u8,
        margin: Margin,
    ) -> Result<Vec<(Event, Coefficient)>> {
        let prob_base = h2h_res.pts_diff() as f64 / alpha as f64;
        let win_prob = (((home_res.wins + 1) + (guest_res.loses + 1)) as f64
            / 2.
            / (tracked_games as u32 + 3) as f64)
            + prob_base;
        let draw_prob = ((home_res.draws + 1) + (guest_res.draws + 1)) as f64
            / 2.
            / (tracked_games as u32 + 3) as f64;
        let lose_prob = (((home_res.loses + 1) + (guest_res.wins + 1)) as f64
            / 2.
            / (tracked_games as u32 + 3) as f64)
            - prob_base;
        let mut probs = [win_prob, draw_prob, lose_prob];
        Self::normalize(&mut probs);
        for p in probs.iter_mut() {
            *p = (1. - f64::from(margin)) / *p;
        }

        Ok(vec![
            (Event::WDL(Winner::W1), probs[0].try_into()?),
            (Event::WDL(Winner::X), probs[1].try_into()?),
            (Event::WDL(Winner::W2), probs[2].try_into()?),
        ])
    }

    pub fn calculate_total_coefficients(
        total: u8,
        totals: PastTotals,
        margin: Margin,
    ) -> Result<Vec<(Event, Coefficient)>> {
        let mut total_coefficients = vec![];

        let n = totals.size() as f64 + 3.;
        let tg = n / (totals.greater() as f64 + 1.);
        let te = n / (totals.equal() as f64 + 1.);
        let tl = n / (totals.less() as f64 + 1.);
        let mut probs = [tg, te, tl];
        Self::normalize(&mut probs);
        for p in probs.iter_mut() {
            *p = (1. - f64::from(margin)) / *p;
        }
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

        Ok(total_coefficients)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize() {
        let mut probs = [0.1, 0.1, 0.1];
        BetCalculator::normalize(&mut probs);
        assert!((probs.iter().sum::<f64>() - 1.).abs() < EPS);

        let mut probs = [0.1];
        BetCalculator::normalize(&mut probs);
        assert!((probs.iter().sum::<f64>() - 1.).abs() < EPS);

        let mut probs = [0.3, 120., 1.076123];
        BetCalculator::normalize(&mut probs);
        assert!((probs.iter().sum::<f64>() - 1.).abs() < EPS);
    }

    #[test]
    fn calculate_total_coefficients() {
        let total = 2;
        let mut totals = PastTotals::new(total);
        totals.add_total(0);
        totals.add_total(1);
        totals.add_total(1);
        totals.add_total(1);
        totals.add_total(2);
        totals.add_total(3);
        let margin = Margin::try_from(0.12).unwrap();

        let res = BetCalculator::calculate_total_coefficients(total, totals, margin).unwrap();

        let mut sum = 0.;
        for (_, coefficient) in res {
            sum += 1. / f64::from(coefficient);
        }
        assert!(sum > 1.);
    }

    #[test]
    fn calculate_winner_coefficients() {
        let home_res = PastResults {
            wins: 6,
            draws: 15,
            loses: 4,
        };
        let guest_res = PastResults {
            wins: 4,
            draws: 9,
            loses: 12,
        };
        let h2h_res = PastResults {
            wins: 5,
            draws: 8,
            loses: 12,
        };
        let margin = Margin::try_from(0.12).unwrap();

        let res = BetCalculator::calculate_winner_coefficients(
            home_res, guest_res, h2h_res, 60, 25, margin,
        )
        .unwrap();

        let mut sum = 0.;
        for (_, coefficient) in res {
            sum += 1. / f64::from(coefficient);
        }
        assert!(sum > 1.);
    }
}
