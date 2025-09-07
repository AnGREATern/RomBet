import React, { useState, useEffect } from 'react';
import { Header } from './components/Header';
import { BetForm } from './components/BetForm';
import { CreateRoundButton } from './components/CreateRoundButton';
import { Balance } from './components/Balance';
import { ReportModal } from './components/ReportModal';
import { GameResults } from './components/GameResults';
import { useApi } from './hooks/useApi';
import { apiClient } from './api/client';
import { Balance as BalanceType, BetStatistics, DisplayedGame, DisplayedGameStat } from './types';
import './App.css';

function App() {
  const [id, setId] = useState("")
  const [games, setGames] = useState<DisplayedGame[]>([]);
  const [currentGames, setCurrentGames] = useState<DisplayedGame[]>([]);
  const [gameStats, setGameStats] = useState<DisplayedGameStat[]>([]);
  const [balance, setBalance] = useState<BalanceType>({ amount: 0 });
  const [report, setReport] = useState<BetStatistics | null>(null);
  const [showReport, setShowReport] = useState(false);
  const [currentRound, setCurrentRound] = useState(0);
  const [lastRound, setLastRound] = useState(0);
  const [createState, setCurrentState] = useState(true);
  
  const { loading, error, callApi, clearError } = useApi();

  useEffect(() => {
    startSimulation();
  }, []);

  const startSimulation = async () => {
    await callApi(async () => {
      const startData = await apiClient.start();
      setBalance({ amount: startData.balance });
      setId(startData.id);
    });
  };

  const loadBalance = async () => {
    const balanceData = await apiClient.getBalance();
    setBalance(balanceData);
  };

  const handleRestart = async () => {
    await callApi(async () => {
      const resp = await apiClient.restart();
      setBalance({ amount: resp.balance });
      setGames([]);
      setCurrentGames([]);
      setGameStats([]);
      setCurrentRound(0);
      setLastRound(0);
      setId(resp.id);
      setCurrentState(true);
    });
  };

  const handleCreateRound = async () => {
    if (!createState) {
      alert("Раунд ещё не закончен. Он будет рандомизирован");
    }
    await callApi(async () => {
      const roundData = await apiClient.createRound();
      if (roundData === null) {
        await handleRandomizeRound();
      } else {
        setCurrentGames(roundData.games);
        setCurrentRound(roundData.round);
        setGames(prev => [...prev, ...roundData.games]);
        setCurrentState(false);
      }
    });
  };

  const handleRandomizeRound = async () => {
    if (createState) {
      alert("Раунд уже закончен. Будет создан новый раунд");
    } 
    await callApi(async () => {
      const result = await apiClient.randomizeRound();
      if (result === null) {
        await handleCreateRound();
      } else {
        setGameStats(result.games_stat);
        setCurrentRound(result.round);
        setLastRound(result.round);
        await loadBalance();
        setCurrentState(true);
      }
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
          {currentGames.length > 0 && currentRound > lastRound && (
            <BetForm games={currentGames} round={currentRound} simulation_id={id} onBetPlaced={loadBalance} />
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
                    {game.home_team.name} vs {game.guest_team.name}
                  </div>
                </div>
              ))}
            </div>
          )}

          {gameStats.length > 0 && (
            <GameResults stats={gameStats} round={lastRound} />
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