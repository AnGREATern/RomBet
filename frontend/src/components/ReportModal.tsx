import React from 'react';
import { BetStatistics } from '../types';
import '../App.css';

interface ReportModalProps {
  report: BetStatistics | null;
  isOpen: boolean;
  onClose: () => void;
}

// Хелпер функция для безопасного форматирования чисел
const formatNumber = (value: any): string => {
  if (value === null || value === undefined) return '0.00';
  
  const num = typeof value === 'number' ? value : Number(value);
  return isNaN(num) ? '0.00' : num.toFixed(2);
};

export const ReportModal: React.FC<ReportModalProps> = ({ report, isOpen, onClose }) => {
  if (!isOpen) return null;

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal-content" onClick={e => e.stopPropagation()}>
        <div className="modal-header">
          <h2>Отчет</h2>
          <button className="modal-close" onClick={onClose}>×</button>
        </div>
        
        {report ? (
          <div className="report-details">
            <div className="report-item">
              <span>Начальный баланс:</span>
              <span>{formatNumber(report.start_balance)}</span>
            </div>
            
            {/* {report.total_bets !== undefined && (
              <div className="report-item">
                <span>Всего ставок:</span>
                <span>{report.total_bets}</span>
              </div>
            )}
            
            {report.total_amount !== undefined && (
              <div className="report-item">
                <span>Общая сумма ставок:</span>
                <span>{formatNumber(report.total_amount)}</span>
              </div>
            )}
            
            {report.rounds_played !== undefined && (
              <div className="report-item">
                <span>Раундов сыграно:</span>
                <span>{report.rounds_played}</span>
              </div>
            )}
            
            {report.profit !== undefined && (
              <div className="report-item">
                <span>Прибыль:</span>
                <span className={report.profit >= 0 ? 'profit-positive' : 'profit-negative'}>
                  {formatNumber(report.profit)}
                </span>
              </div>
            )} */}
            
            {report.min_coefficient_lose !== undefined && (
              <div className="report-item">
                <span>Мин. коэффициент проигрыша:</span>
                <span>{formatNumber(report.min_coefficient_lose)}</span>
              </div>
            )}
          </div>
        ) : (
          <div>Загрузка отчета...</div>
        )}
      </div>
    </div>
  );
};