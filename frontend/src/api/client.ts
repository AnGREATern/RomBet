import axios from 'axios';
import { 
  Bet, DisplayedGameStat, DisplayedGame, Balance, BetStatistics, 
  StartResponse, RandomizeRoundResponse, CreateRoundResponse,
  CalculateCoefficientsRequest, Event,
  DataHelpers,
  EventTotal,
  Winner
} from '../types';

const api = axios.create({
  baseURL: '/api',
});

const serializeEvent = (event: Event): any => {
  if (typeof event === 'string') {
        return { WDL: event };
  } else {
        return { 
      T: {
        total: event.total,
        ordering: event.ordering
      }
    };
  }
};

const transformDisplayedGame = (game: any): DisplayedGame => ({
  id: DataHelpers.parseId(game.id),
  home_team: {
    id: DataHelpers.parseId(game.home_team?.id || game.home_team_id),
    name: DataHelpers.parseTeam(game.home_team)
  },
  guest_team: {
    id: DataHelpers.parseId(game.guest_team?.id || game.guest_team_id),
    name: DataHelpers.parseTeam(game.guest_team)
  }
});

const transformDisplayedGameStat = (stat: any): DisplayedGameStat => ({
  game_id: DataHelpers.parseId(stat.game_id),
  home_team: DataHelpers.parseTeam(stat.home_team),
  guest_team: DataHelpers.parseTeam(stat.guest_team),
  winner: DataHelpers.parseTeam(stat.winner),
  home_score: stat.home_score || stat.home_team_total || 0,
  guest_score: stat.guest_score || stat.guest_team_total || 0
});

const createIdObject = (id: string): any => {
  return { 
    value: id,
    marker: null
  };
};

export const apiClient = {
  start: async (): Promise<StartResponse> => {
    const response = await api.get<StartResponse>('/start');
    return response.data;
  },

  restart: async (): Promise<StartResponse> => {
    const response = await api.post<StartResponse>('/restart');
    return response.data;
  },

  createRound: async (): Promise<CreateRoundResponse | null> => {
    try {
      const response = await api.post<any>('/create_round');
      return {
        round: response.data.round,
        games: response.data.games.map(transformDisplayedGame)
      };
    } catch {
      return null;
    }
  },

  randomizeRound: async (): Promise<RandomizeRoundResponse | null> => {
    try {
      const response = await api.post<any>('/randomize_round');
      return {
        round: response.data.round,
        games_stat: response.data.games_stat.map(transformDisplayedGameStat),
        profit: response.data.profit
      };
    } catch {
      return null;
    }
  },

  calculateCoefficients: async (request: CalculateCoefficientsRequest): Promise<Array<{event: Event; coefficient: number}>> => {
    try {
      const serverRequest = {
        game_id: { value: request.game_id },
        home_team_id: { value: request.home_team_id },
        guest_team_id: { value: request.guest_team_id }
      };

      const response = await api.post<any>(
        '/calculate_coefficients', 
        serverRequest
      );
      
      console.log('Raw response from server:', response.data);
      
      const events = response.data.events || [];
      const coefficients = response.data.coefficients || [];
      
      const minLength = Math.min(events.length, coefficients.length);
      
      const transformedOffers: Array<{event: Event; coefficient: number}> = [];
      
      for (let i = 0; i < minLength; i++) {
        const eventData = events[i];
        const coefficientValue = coefficients[i];
        
        let event: Event = 'W1';
        
        if (eventData && typeof eventData === 'object') {
          if ('WDL' in eventData) {
            event = eventData.WDL as Winner;
          } else if ('T' in eventData) {
            const totalData = eventData.T;
            if (totalData && typeof totalData === 'object') {
              event = {
                total: Number(totalData.total) || 0,
                ordering: [-1, 0, 1].includes(Number(totalData.ordering)) 
                  ? Number(totalData.ordering) as -1 | 0 | 1 
                  : 0
              } as EventTotal;
            }
          }
        } else if (typeof eventData === 'string') {
          if (eventData === 'W1' || eventData === 'X' || eventData === 'W2') {
            event = eventData as Winner;
          }
        }
        
        const coefficient = typeof coefficientValue === 'number' 
          ? coefficientValue / 100
          : 1.0;
        
        transformedOffers.push({ event, coefficient });
      }
      
      console.log('Transformed offers:', transformedOffers);
      return transformedOffers;
      
    } catch (error) {
      console.error('Error in calculateCoefficients:', error);
      return [];
    }
  },
  
  makeBet: async (bet: Bet): Promise<void> => {
        const serverBet = {
      game: {
        id: createIdObject(bet.game.id),
        simulation_id: createIdObject(bet.game.simulation_id || ''),         home_team_id: createIdObject(bet.game.home_team_id),
        guest_team_id: createIdObject(bet.game.guest_team_id),
        round: bet.game.round || 0
      },
      event: serializeEvent(bet.event),
      coefficient: (bet.coefficient * 100) | 0,
      value: bet.value
    };
  
    console.log('Sending bet to server:', JSON.stringify(serverBet, null, 2));
    
    await api.post('/make_bet', serverBet);
  },

  makeReport: async (): Promise<BetStatistics> => {
    const response = await api.get<{ stat: any }>('/make_report');
    const stat = response.data.stat;
    
    console.log('Raw report data:', stat);
    
    return {
      min_coefficient_lose: stat.min_coefficient_lose ? 
        (typeof stat.min_coefficient_lose === 'object' && 'value' in stat.min_coefficient_lose 
          ? Number(stat.min_coefficient_lose.value / 100) 
          : Number(stat.min_coefficient_lose / 100)
        ) : undefined,
      
      start_balance: stat.start_balance ? 
        (typeof stat.start_balance === 'object' && 'value' in stat.start_balance 
          ? Number(stat.start_balance.value / 100) 
          : Number(stat.start_balance / 100)
        ) : 0,
      
      total_bets: stat.total_bets ? Number(stat.total_bets) : 0,
      total_amount: stat.total_amount ? 
        (typeof stat.total_amount === 'object' && 'value' in stat.total_amount 
          ? Number(stat.total_amount.value) 
          : Number(stat.total_amount)
        ) : 0,
      
      rounds_played: stat.rounds_played ? Number(stat.rounds_played) : 0,
      profit: stat.profit ? Number(stat.profit) : 0
    };
  },

  getBalance: async (): Promise<Balance> => {
    const response = await api.get<{ amount: number }>('/balance');
    return { amount: response.data.amount };
  }
};