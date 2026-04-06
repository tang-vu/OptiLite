import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import Dashboard from './components/Dashboard';
import OptimizationPanel from './components/OptimizationPanel';
import ProcessMonitor from './components/ProcessMonitor';
import Settings from './components/Settings';
import './styles/App.css';

interface SystemStats {
  ram_used: number;
  ram_total: number;
  ram_percent: number;
  swap_used: number;
  swap_total: number;
  cpu_usage: number;
  cpu_count: number;
  process_count: number;
  gpu_usage: number;
  gpu_memory_used: number;
  gpu_memory_total: number;
  uptime: number;
  temperature: number;
}

interface ProcessInfo {
  pid: number;
  name: string;
  cpu_usage: number;
  memory: number;
  is_chrome: boolean;
}

type TabType = 'dashboard' | 'optimize' | 'processes' | 'settings';

function App() {
  const [activeTab, setActiveTab] = useState<TabType>('dashboard');
  const [systemStats, setSystemStats] = useState<SystemStats | null>(null);
  const [processes, setProcesses] = useState<ProcessInfo[]>([]);
  const [isOptimizing, setIsOptimizing] = useState(false);
  const [optimizationLog, setOptimizationLog] = useState<string[]>([]);
  const [autoOptimize, setAutoOptimize] = useState(false);
  const [autoOptimizeInterval, setAutoOptimizeInterval] = useState(30);

  const fetchSystemStats = useCallback(async () => {
    try {
      const stats = await invoke<SystemStats>('get_system_stats');
      setSystemStats(stats);
    } catch (error) {
      console.error('Failed to fetch system stats:', error);
    }
  }, []);

  const fetchProcesses = useCallback(async () => {
    try {
      const procs = await invoke<ProcessInfo[]>('get_processes');
      setProcesses(procs.sort((a, b) => b.memory - a.memory).slice(0, 50));
    } catch (error) {
      console.error('Failed to fetch processes:', error);
    }
  }, []);

  useEffect(() => {
    fetchSystemStats();
    fetchProcesses();

    const statsInterval = setInterval(fetchSystemStats, 2000);
    const processInterval = setInterval(fetchProcesses, 5000);

    return () => {
      clearInterval(statsInterval);
      clearInterval(processInterval);
    };
  }, [fetchSystemStats, fetchProcesses]);

  const handleOptimizeRAM = async () => {
    setIsOptimizing(true);
    try {
      const result: any = await invoke('optimize_ram');
      const mbFreed = (result.freed_memory / 1_000_000).toFixed(0);
      setOptimizationLog(prev => [
        `[${new Date().toLocaleTimeString()}] ✅ RAM Optimization: Freed ${mbFreed} MB`,
        ...prev,
      ]);
      fetchSystemStats(); // Refresh stats immediately
    } catch (error) {
      console.error('RAM optimization failed:', error);
    } finally {
      setIsOptimizing(false);
    }
  };

  const handleOptimizeChrome = async () => {
    setIsOptimizing(true);
    try {
      const result: any = await invoke('optimize_chrome');
      const mbFreed = (result.memory_freed / 1_000_000).toFixed(0);
      setOptimizationLog(prev => [
        `[${new Date().toLocaleTimeString()}] 🌐 Chrome Optimization: Freed ${mbFreed} MB (${result.tabs_optimized} tabs)`,
        ...prev,
      ]);
      fetchSystemStats(); // Refresh stats immediately
    } catch (error) {
      console.error('Chrome optimization failed:', error);
    } finally {
      setIsOptimizing(false);
    }
  };

  const handleOptimizeCPU = async () => {
    setIsOptimizing(true);
    try {
      const result: any = await invoke('optimize_cpu');
      setOptimizationLog(prev => [
        `[${new Date().toLocaleTimeString()}] CPU Optimization: ${result.message}`,
        ...prev,
      ]);
    } catch (error) {
      console.error('CPU optimization failed:', error);
    } finally {
      setIsOptimizing(false);
    }
  };

  const handleFullOptimize = async () => {
    setIsOptimizing(true);
    try {
      const result: any = await invoke('full_optimize');
      const mbFreed = (result.freed_memory / 1_000_000).toFixed(0);
      setOptimizationLog(prev => [
        `[${new Date().toLocaleTimeString()}] 🚀 FULL Optimization: Freed ${mbFreed} MB RAM!`,
        ...prev,
      ]);
      fetchSystemStats(); // Refresh stats immediately
    } catch (error) {
      console.error('Full optimization failed:', error);
    } finally {
      setIsOptimizing(false);
    }
  };

  const handleCleanTempFiles = async () => {
    setIsOptimizing(true);
    try {
      const result: any = await invoke('clean_temp_files');
      setOptimizationLog(prev => [
        `[${new Date().toLocaleTimeString()}] Temp Files: ${result.message}`,
        ...prev,
      ]);
    } catch (error) {
      console.error('Temp file cleaning failed:', error);
    } finally {
      setIsOptimizing(false);
    }
  };

  const handleFlushDNS = async () => {
    setIsOptimizing(true);
    try {
      const result: any = await invoke('flush_dns');
      setOptimizationLog(prev => [
        `[${new Date().toLocaleTimeString()}] DNS Flush: ${result.message}`,
        ...prev,
      ]);
    } catch (error) {
      console.error('DNS flush failed:', error);
    } finally {
      setIsOptimizing(false);
    }
  };

  return (
    <div className="app-container">
      <header className="app-header">
        <div className="header-content">
          <div className="logo">
            <span className="logo-icon">⚡</span>
            <h1>OptiLite</h1>
            <span className="version-badge">v1.0.0</span>
          </div>
          <div className="header-status">
            <div className="status-item">
              <span className="status-dot"></span>
              <span className="status-text">System Active</span>
            </div>
          </div>
        </div>
      </header>

      <nav className="tab-navigation">
        <button
          className={`tab-button ${activeTab === 'dashboard' ? 'active' : ''}`}
          onClick={() => setActiveTab('dashboard')}
        >
          <span className="tab-icon">📊</span>
          Dashboard
        </button>
        <button
          className={`tab-button ${activeTab === 'optimize' ? 'active' : ''}`}
          onClick={() => setActiveTab('optimize')}
        >
          <span className="tab-icon">🚀</span>
          Optimize
        </button>
        <button
          className={`tab-button ${activeTab === 'processes' ? 'active' : ''}`}
          onClick={() => setActiveTab('processes')}
        >
          <span className="tab-icon">⚙️</span>
          Processes
        </button>
        <button
          className={`tab-button ${activeTab === 'settings' ? 'active' : ''}`}
          onClick={() => setActiveTab('settings')}
        >
          <span className="tab-icon">⚙️</span>
          Settings
        </button>
      </nav>

      <main className="app-main">
        {activeTab === 'dashboard' && (
          <Dashboard
            systemStats={systemStats}
            onQuickOptimize={handleFullOptimize}
            isOptimizing={isOptimizing}
          />
        )}

        {activeTab === 'optimize' && (
          <OptimizationPanel
            onOptimizeRAM={handleOptimizeRAM}
            onOptimizeChrome={handleOptimizeChrome}
            onOptimizeCPU={handleOptimizeCPU}
            onFullOptimize={handleFullOptimize}
            onCleanTempFiles={handleCleanTempFiles}
            onFlushDNS={handleFlushDNS}
            isOptimizing={isOptimizing}
            optimizationLog={optimizationLog}
          />
        )}

        {activeTab === 'processes' && (
          <ProcessMonitor processes={processes} />
        )}

        {activeTab === 'settings' && (
          <Settings
            autoOptimize={autoOptimize}
            onAutoOptimizeChange={setAutoOptimize}
            autoOptimizeInterval={autoOptimizeInterval}
            onAutoOptimizeIntervalChange={setAutoOptimizeInterval}
          />
        )}
      </main>

      <footer className="app-footer">
        <div className="footer-content">
          <p>🔋 Open Source System Optimizer</p>
          <p>Built with Rust + Tauri + TypeScript</p>
        </div>
      </footer>
    </div>
  );
}

export default App;
