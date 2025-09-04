import React from 'react';
import { Balance } from '../types';
import './Balance.css';

interface BalanceProps {
  balance: Balance;
}

export const Balance: React.FC<BalanceProps> = ({ balance }) => {
  return (
    <div className="balance">
      <h2>Баланс</h2>
      <div className="balance-amount">
        {balance.amount.toFixed(2)} {balance.currency}
      </div>
    </div>
  );
};