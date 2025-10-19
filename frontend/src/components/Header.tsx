import React from 'react';
import '../App.css'

interface HeaderProps {
  onShowTeams: () => void;
  onRestart: () => void;
  onShowReport: () => void;
}

export const Header: React.FC<HeaderProps> = ({ onRestart, onShowReport, onShowTeams }) => {
  return (
    <header className="header">
      <div className="header-content">
        <h1>RomBet</h1>
        <div className="header-actions">
          <button onClick={onShowTeams} className="btn btn-secondary">
            Команды
          </button>
          <button onClick={onShowReport} className="btn btn-secondary">
            Отчет
          </button>
          <button onClick={onRestart} className="btn btn-warning">
            Перезапуск системы
          </button>
        </div>
      </div>
    </header>
  );
};