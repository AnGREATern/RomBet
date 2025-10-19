import React from 'react';
import { BetStatistics } from '../types';
import '../App.css';

interface ReportModalProps {
  report: BetStatistics | null;
  isOpen: boolean;
  onClose: () => void;
}

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