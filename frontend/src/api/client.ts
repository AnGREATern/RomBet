import axios from 'axios';
import { 
  Bet, Game, DisplayedGame, Balance, BetStatistics, 
  StartResponse, RandomizeRoundResponse, CreateRoundResponse,
  CoefficientOffer, CalculateCoefficientsRequest
} from '../types';

const api = axios.create({
  baseURL: '/api',
});

export const apiClient = {
  // Старт системы
  start: async (): Promise<StartResponse> => {
    const response = await api.post<StartResponse>('/');
    return response.data;
  },

  // Перезапуск
  restart: async (): Promise<StartResponse> => {
    const response = await api.post<StartResponse>('/restart');
    return response.data;
  },

  // Создать раунд
  createRound: async (): Promise<CreateRoundResponse> => {
    const response = await api.post<CreateRoundResponse>('/create_round');
    return response.data;
  },

  // Рандомизировать раунд
  randomizeRound: async (): Promise<RandomizeRoundResponse> => {
    const response = await api.post<RandomizeRoundResponse>('/randomize_round');
    return response.data;
  },

  // Рассчитать коэффициенты
  calculateCoefficients: async (request: CalculateCoefficientsRequest): Promise<CoefficientOffer[]> => {
    const response = await api.post<{ offers: CoefficientOffer[] }>('/calculate_coefficients', request);
    return response.data.offers;
  },

  // Сделать ставку
  makeBet: async (bet: Bet): Promise<void> => {
    await api.post('/make_bet', bet);
  },

  // Получить отчет
  makeReport: async (): Promise<BetStatistics> => {
    const response = await api.get<{ stat: BetStatistics }>('/make_report');
    return response.data.stat;
  },

  // Получить баланс
  getBalance: async (): Promise<Balance> => {
    const response = await api.get<{ amount: number }>('/balance');
    return { amount: response.data.amount };
  }
};