import React, { useState, useEffect } from 'react';
import { Bet, Game, Event } from '../types';
import { apiClient } from '../api/client';
import { useApi } from '../hooks/useApi';
import './BetForm.css';

interface BetFormProps {
  games: Game[];
  onBetPlaced: () => void;
}

export const BetForm: React.FC<BetFormProps> = ({ games, onBetPlaced }) => {
  const [selectedGame, setSelectedGame] = useState<Game | null>(null);
  const [selectedEvent, setSelectedEvent] = useState<Event>('home_win');
  const [coefficient, setCoefficient] = useState(1.0);
  const [amount, setAmount] = useState('');
  const [availableEvents, setAvailableEvents] = useState<{event: Event, coefficient: number}[]>([]);

  const { loading, error, callApi, clearError } = useApi();

  useEffect(() => {
    if (selectedGame) {
      loadCoefficients();
    }
  }, [selectedGame]);

  const loadCoefficients = async () => {
    if (!selectedGame) return;

    await callApi(async () => {
      const coefficients = await apiClient.calculateCoefficients({
        game_id: selectedGame.id,
        home_team_id: selectedGame.home_team_id,
        guest_team_id: selectedGame.guest_team_id
      });
      
      setAvailableEvents(coefficients);
      if (coefficients.length > 0) {
        setSelectedEvent(coefficients[0].event);
        setCoefficient(coefficients[0].coefficient);
      }
    });
  };

  const handleGameChange = (gameId: string) => {
    const game = games.find(g => g.id === gameId);
    setSelectedGame(game || null);
  };

  const handleEventChange = (event: Event) => {
    const selected = availableEvents.find(ae => ae.event === event);
    if (selected) {
      setSelectedEvent(event);
      setCoefficient(selected.coefficient);
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!selectedGame || !amount) return;

    const bet: Bet = {
      game: selectedGame,
      event: selectedEvent,
      coefficient: coefficient,
      value: parseFloat(amount)
    };

    await callApi(async () => {
      await apiClient.makeBet(bet);
      onBetPlaced();
      setAmount('');
    });
  };

  return (
    <div className="bet-form">
      <h2>Сделать ставку</h2>
      <form onSubmit={handleSubmit}>
        <div className="form-group">
          <label>Игра:</label>
          <select 
            value={selectedGame?.id || ''}
            onChange={(e) => handleGameChange(e.target.value)}
            required
          >
            <option value="">Выберите игру</option>
            {games.map(game => (
              <option key={game.id} value={game.id}>
                Игра {game.id} (Раунд {game.round})
              </option>
            ))}
          </select>
        </div>

        {selectedGame && availableEvents.length > 0 && (
          <>
            <div className="form-group">
              <label>Событие:</label>
              <select 
                value={selectedEvent}
                onChange={(e) => handleEventChange(e.target.value as Event)}
                required
              >
                {availableEvents.map(({ event, coefficient }) => (
                  <option key={event} value={event}>
                    {event === 'home_win' ? 'Победа дома' : 
                     event === 'guest_win' ? 'Победа гостей' : 'Ничья'} 
                    (коэф. {coefficient.toFixed(2)})
                  </option>
                ))}
              </select>
            </div>

            <div className="coefficient-info">
              Текущий коэффициент: {coefficient.toFixed(2)}
            </div>
          </>
        )}

        <div className="form-group">
          <label>Сумма ставки:</label>
          <input
            type="number"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            min="1"
            step="0.01"
            required
          />
        </div>

        <button 
          type="submit" 
          disabled={loading || !selectedGame}
          className="btn btn-primary"
        >
          {loading ? 'Размещение...' : 'Сделать ставку'}
        </button>
      </form>

      {error && (
        <div className="error">
          {error}
          <button onClick={clearError}>×</button>
        </div>
      )}
    </div>
  );
};