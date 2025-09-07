import React from 'react';
import { apiClient } from '../api/client';
import { useApi } from '../hooks/useApi';

interface CreateRoundButtonProps {
  onRoundCreated: () => void;
}

export const CreateRoundButton: React.FC<CreateRoundButtonProps> = ({ onRoundCreated }) => {
  const { loading, error, callApi, clearError } = useApi();

  const handleCreateRound = async () => {
    await callApi(async () => {
            onRoundCreated();
    });
  };

  return (
    <div className="create-round-section">
      <h3>Управление раундами</h3>
      {error && (
        <div className="error">
          {error}
          <button onClick={clearError}>×</button>
        </div>
      )}
      <button 
        onClick={handleCreateRound} 
        disabled={loading}
        className="btn btn-primary"
      >
        {loading ? 'Создание...' : 'Создать новый раунд'}
      </button>
    </div>
  );
};