import React, { useState, useEffect } from 'react';
import { Header } from './components/Header';
import { BetForm } from './components/BetForm';
import { CreateRoundButton } from './components/CreateRoundButton';
import { Balance } from './components/Balance';
import { ReportModal } from './components/ReportModal';
import { GameResults } from './components/GameResults';
import { useApi } from './hooks/useApi';
import { apiClient } from './api/client';
import { Game, Balance as BalanceType, BetStatistics, DisplayedGame, DisplayedGameStat } from './types';
import './App.css';

function App() {
  const [games, setGames] = useState<Game[]>([]);
  const [currentGames, setCurrentGames] = useState<DisplayedGame[]>([]);
  const [gameStats, setGameStats] = useState<DisplayedGameStat[]>([]);
  const [balance, setBalance] = useState<BalanceType>({ amount: 0 });
  const [report, setReport] = useState<BetStatistics | null>(null);
  const [showReport, setShowReport] = useState(false);
  const [currentRound, setCurrentRound] = useState(0);
  
  const { loading, error, callApi, clearError } = useApi();

  useEffect(() => {
    startSimulation();
  }, []);

  const startSimulation = async () => {
    await callApi(async () => {
      const startData = await apiClient.start();
      setBalance({ amount: startData.balance });
      await loadBalance();
    });
  };

  const loadBalance = async () => {
    const balanceData = await apiClient.getBalance();
    setBalance(balanceData);
  };

  const handleRestart = async () => {
    await callApi(async () => {
      await apiClient.restart();
      await loadBalance();
      setGames([]);
      setCurrentGames([]);
      setGameStats([]);
      setCurrentRound(0);
    });
  };

  const handleCreateRound = async () => {
    await callApi(async () => {
      const roundData = await apiClient.createRound();
      setCurrentGames(roundData.games);
      setCurrentRound(roundData.round);
      
      // Конвертируем DisplayedGame в Game для формы ставок
      const newGames: Game[] = roundData.games.map(game => ({
        id: game.id,
        simulation_id: '', // будет заполнено сервером
        home_team_id: game.home_team,
        guest_team_id: game.guest_team,
        round: roundData.round
      }));
      
      setGames(prev => [...prev, ...newGames]);
    });
  };

  const handleRandomizeRound = async () => {
    await callApi(async () => {
      const result = await apiClient.randomizeRound();
      setGameStats(result.games_stat);
      setCurrentRound(result.round);
      await loadBalance();
    });
  };

  const handleShowReport = async () => {
    const reportData = await callApi(apiClient.makeReport);
    if (reportData) {
      setReport(reportData);
      setShowReport(true);
    }
  };

  if (loading && games.length === 0) {
    return <div className="loading">Загрузка...</div>;
  }

  return (
    <div className="app">
      <Header onRestart={handleRestart} onShowReport={handleShowReport} />
      
      {error && (
        <div className="error">
          {error}
          <button onClick={clearError}>×</button>
        </div>
      )}

      <div className="main-content">
        <div className="sidebar">
          <Balance balance={balance} />
          <CreateRoundButton onRoundCreated={handleCreateRound} />
          {currentGames.length > 0 && (
            <BetForm games={games} onBetPlaced={loadBalance} />
          )}
        </div>

        <div className="content">
          <div className="content-header">
            <h2>Текущий раунд: {currentRound}</h2>
            {currentGames.length > 0 && (
              <button 
                onClick={handleRandomizeRound}
                className="btn btn-secondary"
              >
                Завершить раунд
              </button>
            )}
          </div>

          {currentGames.length > 0 && (
            <div className="current-games">
              <h3>Текущие игры</h3>
              {currentGames.map(game => (
                <div key={game.id} className="game-card">
                  <div className="teams">
                    {game.home_team} vs {game.guest_team}
                  </div>
                  <div className="round">Раунд {game.round}</div>
                </div>
              ))}
            </div>
          )}

          {gameStats.length > 0 && (
            <GameResults stats={gameStats} />
          )}
        </div>
      </div>

      <ReportModal
        report={report}
        isOpen={showReport}
        onClose={() => setShowReport(false)}
      />
    </div>
  );
}

export default App;