import React, { useState, useEffect } from 'react';
import { Bet, Event, DisplayedGame, EventHelpers } from '../types';
import { apiClient } from '../api/client';
import { useApi } from '../hooks/useApi';
import '../App.css';

interface BetFormProps {
  games: DisplayedGame[];
  round: number;
  simulation_id: string;
  onBetPlaced: () => void;
}

export const BetForm: React.FC<BetFormProps> = ({ games, round, simulation_id, onBetPlaced }) => {
  const [selectedGame, setSelectedGame] = useState<DisplayedGame | null>(null);
  const [selectedEventIndex, setSelectedEventIndex] = useState<number>(0);
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
      try {
        const coefficients = await apiClient.calculateCoefficients({
          game_id: selectedGame.id,
          home_team_id: selectedGame.home_team.id,
          guest_team_id: selectedGame.guest_team.id
        });
        
                const validCoefficients = coefficients
          .filter(offer => offer.coefficient !== undefined && offer.coefficient !== null)
          .map(offer => ({
            event: offer.event || 'W1',
            coefficient: offer.coefficient || 1.0
          }));
        
        setAvailableEvents(validCoefficients);
        
        if (validCoefficients.length > 0) {
          setSelectedEventIndex(0);
          setCoefficient(validCoefficients[0].coefficient);
        }
      } catch (error) {
        console.error('Error loading coefficients:', error);
                setAvailableEvents([]);
      }
    });
  };

  const handleGameChange = (gameId: string) => {
    const game = games.find(g => g.id === gameId);
    setSelectedGame(game || null);
    setAvailableEvents([]);
    setSelectedEventIndex(0);
    setCoefficient(1.0);
  };

  const handleEventChange = (eventIndex: number) => {
    if (eventIndex < 0 || eventIndex >= availableEvents.length) return;
    
    const selected = availableEvents[eventIndex];
    if (selected) {
      setSelectedEventIndex(eventIndex);
      setCoefficient(selected.coefficient || 1.0);
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!selectedGame || !amount || availableEvents.length === 0) return;

    const selectedEventData = availableEvents[selectedEventIndex];
    if (!selectedEventData) return;

    const selectedEvent = selectedEventData.event || 'W1';
    const selectedCoefficient = selectedEventData.coefficient || 1.0;

    const bet: Bet = {
      game: {
        id: selectedGame.id,
        simulation_id,
        home_team_id: selectedGame.home_team.id,
        guest_team_id: selectedGame.guest_team.id,
        round
      },
      event: selectedEvent,
      coefficient: selectedCoefficient,
      value: parseFloat(amount) || 0
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
                {game.home_team.name} vs {game.guest_team.name}
              </option>
            ))}
          </select>
        </div>

        {selectedGame && availableEvents.length > 0 && (
          <>
            <div className="form-group">
              <label>Событие:</label>
              <select 
                value={selectedEventIndex}
                onChange={(e) => handleEventChange(parseInt(e.target.value))}
                required
              >
                {availableEvents.map(({ event, coefficient }, index) => {
                  const safeCoefficient = coefficient || 1.0;
                  const safeEvent = event || 'W1';
                  
                  return (
                    <option key={index} value={index}>
                      {EventHelpers.formatEvent(safeEvent)} (коэф. {safeCoefficient.toFixed(2)})
                    </option>
                  );
                })}
              </select>
            </div>

            <div className="coefficient-info">
              Текущий коэффициент: {(coefficient || 1.0).toFixed(2)}
            </div>
          </>
        )}

        {selectedGame && availableEvents.length === 0 && !loading && (
          <div className="no-events">
            Нет доступных событий для ставок
          </div>
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
          disabled={loading || !selectedGame || availableEvents.length === 0}
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