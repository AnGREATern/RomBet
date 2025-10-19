import React, { useState, useEffect } from 'react';
import { Team, CreateTeamRequest, UpdateTeamRequest } from '../types';
import { apiClient } from '../api/client';
import { useApi } from '../hooks/useApi';
import '../TeamsManagement.css';

interface TeamsManagementProps {
  isOpen: boolean;
  onClose: () => void;
}

export const TeamsManagement: React.FC<TeamsManagementProps> = ({ isOpen, onClose }) => {
  const [teams, setTeams] = useState<Team[]>([]);
  const [editingTeam, setEditingTeam] = useState<Team | null>(null);
  const [newTeamName, setNewTeamName] = useState('');
  const [editTeamName, setEditTeamName] = useState('');

  const { loading, error, callApi, clearError } = useApi();

  useEffect(() => {
    if (isOpen) {
      loadTeams();
    }
  }, [isOpen]);

  const loadTeams = async () => {
    await callApi(async () => {
      const teamsData = await apiClient.getAllTeams();
      setTeams(teamsData);
    });
  };

  const handleCreateTeam = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newTeamName.trim()) return;

    await callApi(async () => {
      await apiClient.createTeam({ name: newTeamName.trim() });
      setNewTeamName('');
      await loadTeams();
    });
  };

  const handleUpdateTeam = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!editingTeam || !editTeamName.trim()) return;

    await callApi(async () => {
      await apiClient.updateTeam(editingTeam.id, { name: editTeamName.trim() });
      setEditingTeam(null);
      setEditTeamName('');
      await loadTeams();
    });
  };

  const handleDeleteTeam = async (teamId: string) => {
    if (!window.confirm('Вы уверены, что хотите удалить эту команду?')) return;

    await callApi(async () => {
      await apiClient.deleteTeam(teamId);
      await loadTeams();
    });
  };

  const startEditing = (team: Team) => {
    setEditingTeam(team);
    setEditTeamName(team.name);
  };

  const cancelEditing = () => {
    setEditingTeam(null);
    setEditTeamName('');
  };

  if (!isOpen) return null;

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal-content teams-management-modal" onClick={e => e.stopPropagation()}>
        <div className="modal-header">
          <h2>Управление командами</h2>
          <button className="modal-close" onClick={onClose}>×</button>
        </div>

        {error && (
          <div className="error">
            {error}
            <button onClick={clearError}>×</button>
          </div>
        )}

        <div className="teams-content">
          {/* Форма создания новой команды */}
          <div className="create-team-section">
            <h3>Добавить новую команду</h3>
            <form onSubmit={handleCreateTeam} className="team-form">
              <div className="form-group">
                <input
                  type="text"
                  value={newTeamName}
                  onChange={(e) => setNewTeamName(e.target.value)}
                  placeholder="Введите название команды"
                  required
                  disabled={loading}
                />
              </div>
              <button type="submit" disabled={loading || !newTeamName.trim()} className="btn btn-primary">
                {loading ? 'Создание...' : 'Создать команду'}
              </button>
            </form>
          </div>

          {/* Список команд */}
          <div className="teams-list-section">
            <h3>Список команд ({teams.length})</h3>
            
            {loading ? (
              <div className="loading">Загрузка команд...</div>
            ) : teams.length === 0 ? (
              <div className="no-teams">Команды не найдены</div>
            ) : (
              <div className="teams-list">
                {teams.map((team, index) => (
                  <div key={team.id || `team-${index}`} className="team-item">
                    {editingTeam?.id === team.id ? (
                      // Режим редактирования
                      <form onSubmit={handleUpdateTeam} className="team-edit-form">
                        <input
                          type="text"
                          value={editTeamName}
                          onChange={(e) => setEditTeamName(e.target.value)}
                          required
                          disabled={loading}
                          className="edit-input"
                        />
                        <div className="team-actions">
                          <button type="submit" disabled={loading} className="btn btn-success btn-sm">
                            Сохранить
                          </button>
                          <button type="button" onClick={cancelEditing} disabled={loading} className="btn btn-secondary btn-sm">
                            Отмена
                          </button>
                        </div>
                      </form>
                    ) : (
                      // Режим просмотра
                      <>
                        <span className="team-name">{team.name}</span>
                        <div className="team-actions">
                          <button 
                            onClick={() => startEditing(team)} 
                            className="btn btn-secondary btn-sm"
                          >
                            Редактировать
                          </button>
                          <button 
                            onClick={() => handleDeleteTeam(team.id)} 
                            className="btn btn-danger btn-sm"
                            disabled={loading}
                          >
                            Удалить
                          </button>
                        </div>
                      </>
                    )}
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};