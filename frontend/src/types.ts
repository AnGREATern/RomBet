export interface Bet {
    game: Game;
    event: Event;
    coefficient: number;
    value: number;
  }
  
  export interface Game {
    id: string;
    simulation_id: string;
    home_team_id: string;
    guest_team_id: string;
    round: number;
  }
  
  export interface DisplayedGame {
    id: string;
    home_team: string;
    guest_team: string;
    round: number;
  }
  
  export interface DisplayedGameStat {
    game_id: string;
    home_team: string;
    guest_team: string;
    winner: string;
    home_score: number;
    guest_score: number;
  }
  
  export interface Round {
    round: number;
    games: DisplayedGame[];
  }
  
  export interface Balance {
    amount: number;
  }
  
  export interface BetStatistics {
    total_bets: number;
    total_amount: number;
    rounds_played: number;
    profit: number;
  }
  
  export interface StartResponse {
    balance: number;
  }
  
  export interface RandomizeRoundResponse {
    round: number;
    games_stat: DisplayedGameStat[];
    profit: number;
  }
  
  export interface CreateRoundResponse {
    round: number;
    games: DisplayedGame[];
  }
  
  export interface CoefficientOffer {
    event: Event;
    coefficient: number;
  }
  
  export type Event = 'home_win' | 'guest_win' | 'draw';