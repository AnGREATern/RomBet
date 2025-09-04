import React from 'react';
import { DisplayedGameStat } from '../types';
import './GameResults.css';

interface GameResultsProps {
  stats: DisplayedGameStat[];
}

export const GameResults: React.FC<GameResultsProps> = ({ stats }) => {
  return (
    <div className="game-results">
      <h3>Результаты игр</h3>
      <div className="results-grid">
        {stats.map(stat => (
          <div key={stat.game_id} className="result-card">
            <div className="matchup">
              {stat.home_team} {stat.home_score} - {stat.guest_score} {stat.guest_team}
            </div>
            <div className="winner">
              Победитель: {stat.winner}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};