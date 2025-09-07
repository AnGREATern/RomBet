import React from 'react';
import '../App.css'

interface HeaderProps {
  onRestart: () => void;
  onShowReport: () => void;
}

export const Header: React.FC<HeaderProps> = ({ onRestart, onShowReport }) => {
  return (
    <header className="header">
      <div className="header-content">
        <h1>RomBet</h1>
        <div className="header-actions">
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