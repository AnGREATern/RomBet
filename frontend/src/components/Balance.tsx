import React from 'react';
import { Balance as BalanceType } from '../types';
import '../App.css'

interface BalanceProps {
  balance: BalanceType;
}

export const Balance: React.FC<BalanceProps> = ({ balance }) => {
  return (
    <div className="balance">
      <h2>Баланс</h2>
      <div className="balance-amount">
        {balance.amount.toFixed(2)} руб.
      </div>
    </div>
  );
};