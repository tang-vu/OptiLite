import { useState, useEffect } from 'react';

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

interface DashboardProps {
  systemStats: SystemStats | null;
  onQuickOptimize: () => void;
  isOptimizing: boolean;
}

function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
}

function formatUptime(seconds: number): string {
  const days = Math.floor(seconds / 86400);
  const hours = Math.floor((seconds % 86400) / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  return `${days}d ${hours}h ${minutes}m`;
}

function CircularProgress({
  percentage,
  color,
  size = 120,
  strokeWidth = 10,
  label,
  value,
}: {
  percentage: number;
  color: string;
  size?: number;
  strokeWidth?: number;
  label: string;
  value: string;
}) {
  const radius = (size - strokeWidth) / 2;
  const circumference = radius * 2 * Math.PI;
  const offset = circumference - (percentage / 100) * circumference;

  return (
    <div className="circular-progress">
      <svg width={size} height={size}>
        <circle
          className="progress-bg"
          cx={size / 2}
          cy={size / 2}
          r={radius}
          fill="none"
          stroke="rgba(255,255,255,0.1)"
          strokeWidth={strokeWidth}
        />
        <circle
          className="progress-bar"
          cx={size / 2}
          cy={size / 2}
          r={radius}
          fill="none"
          stroke={color}
          strokeWidth={strokeWidth}
          strokeDasharray={circumference}
          strokeDashoffset={offset}
          strokeLinecap="round"
          transform={`rotate(-90 ${size / 2} ${size / 2})`}
        />
      </svg>
      <div className="progress-content">
        <div className="progress-value" style={{ color }}>
          {Math.round(percentage)}%
        </div>
      </div>
      <div className="progress-label">{label}</div>
      <div className="progress-detail">{value}</div>
    </div>
  );
}

export default function Dashboard({
  systemStats,
  onQuickOptimize,
  isOptimizing,
}: DashboardProps) {
  const [animate, setAnimate] = useState(false);

  useEffect(() => {
    setAnimate(true);
    const timer = setTimeout(() => setAnimate(false), 1000);
    return () => clearTimeout(timer);
  }, [systemStats]);

  if (!systemStats) {
    return <div className="loading">Loading system statistics...</div>;
  }

  const ramColor =
    systemStats.ram_percent > 80
      ? '#ff4757'
      : systemStats.ram_percent > 60
      ? '#ffa502'
      : '#2ed573';

  const cpuColor =
    systemStats.cpu_usage > 80
      ? '#ff4757'
      : systemStats.cpu_usage > 60
      ? '#ffa502'
      : '#1e90ff';

  return (
    <div className={`dashboard ${animate ? 'animate' : ''}`}>
      <div className="dashboard-header">
        <h2>System Dashboard</h2>
        <button
          className="btn-optimize"
          onClick={onQuickOptimize}
          disabled={isOptimizing}
        >
          {isOptimizing ? (
            <>
              <span className="spinner"></span>
              Optimizing...
            </>
          ) : (
            <>🚀 Quick Optimize</>
          )}
        </button>
      </div>

      <div className="stats-grid">
        <div className="stat-card main-stat">
          <CircularProgress
            percentage={systemStats.ram_percent}
            color={ramColor}
            label="RAM Usage"
            value={`${formatBytes(systemStats.ram_used)} / ${formatBytes(systemStats.ram_total)}`}
          />
        </div>

        <div className="stat-card main-stat">
          <CircularProgress
            percentage={systemStats.cpu_usage}
            color={cpuColor}
            label="CPU Usage"
            value={`${systemStats.cpu_count} Cores | ${systemStats.cpu_usage.toFixed(1)}%`}
          />
        </div>

        <div className="stat-card">
          <div className="stat-icon">📦</div>
          <div className="stat-info">
            <div className="stat-label">Processes</div>
            <div className="stat-value">{systemStats.process_count}</div>
          </div>
        </div>

        <div className="stat-card">
          <div className="stat-icon">💾</div>
          <div className="stat-info">
            <div className="stat-label">Swap</div>
            <div className="stat-value">
              {formatBytes(systemStats.swap_used)} /{' '}
              {formatBytes(systemStats.swap_total)}
            </div>
          </div>
        </div>

        <div className="stat-card">
          <div className="stat-icon">🎮</div>
          <div className="stat-info">
            <div className="stat-label">GPU</div>
            <div className="stat-value">
              {systemStats.gpu_usage > 0
                ? `${systemStats.gpu_usage.toFixed(1)}% | ${formatBytes(systemStats.gpu_memory_used)}`
                : 'Monitoring...'}
            </div>
            {systemStats.gpu_memory_total > 0 && (
              <div className="stat-detail">
                {formatBytes(systemStats.gpu_memory_used)} / {formatBytes(systemStats.gpu_memory_total)}
              </div>
            )}
          </div>
        </div>

        <div className="stat-card">
          <div className="stat-icon">🌡️</div>
          <div className="stat-info">
            <div className="stat-label">Temperature</div>
            <div className="stat-value">
              {systemStats.temperature > 0
                ? `${systemStats.temperature.toFixed(1)}°C`
                : 'Reading sensors...'}
            </div>
            {systemStats.temperature > 0 && (
              <div className="stat-detail">
                {systemStats.temperature > 80 ? '⚠️ Hot!' : systemStats.temperature > 60 ? '🟡 Warm' : '🟢 Cool'}
              </div>
            )}
          </div>
        </div>

        <div className="stat-card">
          <div className="stat-icon">⏱️</div>
          <div className="stat-info">
            <div className="stat-label">Uptime</div>
            <div className="stat-value">{formatUptime(systemStats.uptime)}</div>
          </div>
        </div>
      </div>

      <div className="quick-actions">
        <h3>Quick Actions</h3>
        <div className="action-buttons">
          <button className="action-btn ram">
            <span className="action-icon">🧹</span>
            <div className="action-text">
              <strong>Clean RAM</strong>
              <span>Free up memory</span>
            </div>
          </button>
          <button className="action-btn chrome">
            <span className="action-icon">🌐</span>
            <div className="action-text">
              <strong>Optimize Chrome</strong>
              <span>Reduce tab memory</span>
            </div>
          </button>
          <button className="action-btn cpu">
            <span className="action-icon">⚡</span>
            <div className="action-text">
              <strong>Boost CPU</strong>
              <span>Optimize processes</span>
            </div>
          </button>
          <button className="action-btn clean">
            <span className="action-icon">🗑️</span>
            <div className="action-text">
              <strong>Clean Temp</strong>
              <span>Remove temp files</span>
            </div>
          </button>
        </div>
      </div>
    </div>
  );
}
