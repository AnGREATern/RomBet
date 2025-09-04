import React from 'react';
import { Report } from '../types';
import './ReportModal.css';

interface ReportModalProps {
  report: Report | null;
  isOpen: boolean;
  onClose: () => void;
}

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
              <span>Всего ставок:</span>
              <span>{report.totalBets}</span>
            </div>
            <div className="report-item">
              <span>Общая сумма:</span>
              <span>{report.totalAmount.toFixed(2)}</span>
            </div>
            <div className="report-item">
              <span>Раундов сыграно:</span>
              <span>{report.roundsPlayed}</span>
            </div>
            <div className="report-item">
              <span>Прибыль:</span>
              <span className={report.profit >= 0 ? 'profit-positive' : 'profit-negative'}>
                {report.profit.toFixed(2)}
              </span>
            </div>
          </div>
        ) : (
          <div>Загрузка отчета...</div>
        )}
      </div>
    </div>
  );
};