export interface Bet {
  game: Game;
  event: Event;
  coefficient: number;
  value: number;
}

export interface Game {
  id: string;   simulation_id: string;
  home_team_id: string;
  guest_team_id: string;
  round: number;
}

export interface Team {
  id: string;
  name: string;
}

export interface DisplayedGame {
  id: string;
  home_team: Team;
  guest_team: Team;
}

export interface DisplayedGameStat {
  game_id: string;
  home_team: string;   guest_team: string;   winner: string;
  home_score: number;
  guest_score: number;
}

export interface Balance {
  amount: number;
}

export interface BetStatistics {
  min_coefficient_lose?: number;
  start_balance: number;
  total_bets?: number;
  total_amount?: number;
  rounds_played?: number;
  profit?: number;
}

export interface StartResponse {
  id: string;
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

export interface CalculateCoefficientsResponse {
  events: any[];   coefficients: number[]; }

export interface CalculateCoefficientsRequest {
  game_id: string;
  home_team_id: string;
  guest_team_id: string;
}

export type Event = Winner | EventTotal;

export type Winner = 'W1' | 'X' | 'W2';

export interface EventTotal {
  total: number;
  ordering: -1 | 0 | 1;
}

export const EventHelpers = {
  isWinner: (event: Event): event is Winner => {
    return event === 'W1' || event === 'X' || event === 'W2';
  },
  
  isEventTotal: (event: Event): event is EventTotal => {
    return typeof event === 'object' && 'total' in event && 'ordering' in event;
  },
  
  formatEvent: (event: Event): string => {
    if (EventHelpers.isWinner(event)) {
      switch (event) {
        case 'W1': return 'П1';
        case 'X': return 'X';
        case 'W2': return 'П2';
        default: return event;
      }
    } else {
      switch (event.ordering) {
        case -1: return `ТМ${event.total - 0.5}`;
        case 0: return `${event.total} голов`;
        case 1: return `ТБ${event.total + 0.5}`;
        default: return `Total ${event.total}`;
      }
    }
  },
  
  getEventKey: (event: Event, index: number): string => {
    if (EventHelpers.isWinner(event)) {
      return `winner-${event}-${index}`;
    } else {
      return `total-${event.total}-${event.ordering}-${index}`;
    }
  }
};

export const DataHelpers = {
  parseId: (id: any): string => {
    if (typeof id === 'string') return id;
    if (id && typeof id === 'object' && 'value' in id) {
      return id.value;     }
    return String(id);
  },
  
  parseTeam: (team: any): string => {
    if (typeof team === 'string') return team;
    if (team && typeof team === 'object' && 'name' in team) {
      return team.name;     }
    return String(team);
  }
};