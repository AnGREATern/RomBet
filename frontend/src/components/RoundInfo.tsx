import React from 'react';
import { Round } from '../types';
import './RoundInfo.css';

interface RoundInfoProps {
  round: Round;
  onRandomize: (roundId: string) => void;
}

export const RoundInfo: React.FC<RoundInfoProps> = ({ round, onRandomize }) => {
  return (
    <div className={`round-info ${round.isFinished ? 'finished' : 'active'}`}>
      <h3>Раунд {round.id}</h3>
      <div className="teams">
        {round.teams.map(team => (
          <div key={team} className="team">
            <span className="team-name">{team}</span>
            <span className="coefficient">
              Коэффициент: {round.coefficients[team]?.toFixed(2) || '1.00'}
            </span>
          </div>
        ))}
      </div>
      
      {round.isFinished ? (
        <div className="round-result">
          <strong>Победитель: {round.winner}</strong>
        </div>
      ) : (
        <button 
          onClick={() => onRandomize(round.id)}
          className="btn btn-secondary"
        >
          Завершить раунд
        </button>
      )}
    </div>
  );
};