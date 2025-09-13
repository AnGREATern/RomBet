import React from 'react';
import { DisplayedGameStat } from '../types';
import '../App.css';

interface GameResultsProps {
  stats: DisplayedGameStat[];
  round: number;
}

export const GameResults: React.FC<GameResultsProps> = ({ stats, round }) => {
  return (
    <div className="game-results">
      <h3>Результаты игр раунда {round}</h3>
      <div className="results-grid">
        {stats.map((stat, index) => (
          <div key={index} className="result-card">
            <div className="matchup">
              {stat.home_team} {stat.home_score} - {stat.guest_score} {stat.guest_team}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};