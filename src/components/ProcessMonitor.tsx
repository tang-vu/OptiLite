interface ProcessInfo {
  pid: number;
  name: string;
  cpu_usage: number;
  memory: number;
  is_chrome: boolean;
}

interface ProcessMonitorProps {
  processes: ProcessInfo[];
}

function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
}

export default function ProcessMonitor({ processes }: ProcessMonitorProps) {
  const chromeProcesses = processes.filter(p => p.is_chrome);
  const topMemoryProcesses = processes.slice(0, 10);
  const topCPUProcesses = [...processes].sort((a, b) => b.cpu_usage - a.cpu_usage).slice(0, 10);

  return (
    <div className="process-monitor">
      <div className="process-header">
        <h2>⚙️ Process Monitor</h2>
        <div className="process-stats">
          <div className="proc-stat">
            <span className="proc-stat-label">Total</span>
            <span className="proc-stat-value">{processes.length}</span>
          </div>
          <div className="proc-stat chrome">
            <span className="proc-stat-label">Chrome</span>
            <span className="proc-stat-value">{chromeProcesses.length}</span>
          </div>
        </div>
      </div>

      <div className="process-grid">
        <div className="process-section">
          <h3>🌐 Chrome Processes</h3>
          {chromeProcesses.length > 0 ? (
            <div className="process-list">
              {chromeProcesses.map(proc => (
                <div key={proc.pid} className="process-item chrome-process">
                  <div className="process-info">
                    <span className="process-name">{proc.name}</span>
                    <span className="process-pid">PID: {proc.pid}</span>
                  </div>
                  <div className="process-metrics">
                    <span className="metric cpu">
                      <span className="metric-icon">⚡</span>
                      {proc.cpu_usage.toFixed(1)}%
                    </span>
                    <span className="metric memory">
                      <span className="metric-icon">💾</span>
                      {formatBytes(proc.memory)}
                    </span>
                  </div>
                </div>
              ))}
            </div>
          ) : (
            <div className="empty-state">No Chrome processes running</div>
          )}
        </div>

        <div className="process-section">
          <h3>💾 Top Memory Usage</h3>
          <div className="process-list">
            {topMemoryProcesses.map((proc, index) => (
              <div key={proc.pid} className="process-item">
                <div className="process-rank">#{index + 1}</div>
                <div className="process-info">
                  <span className="process-name">{proc.name}</span>
                  <span className="process-pid">PID: {proc.pid}</span>
                </div>
                <div className="process-metrics">
                  <span className="metric memory">
                    {formatBytes(proc.memory)}
                  </span>
                </div>
                <div className="memory-bar-container">
                  <div
                    className="memory-bar"
                    style={{
                      width: `${Math.min(
                        (proc.memory / topMemoryProcesses[0].memory) * 100,
                        100
                      )}%`,
                    }}
                  ></div>
                </div>
              </div>
            ))}
          </div>
        </div>

        <div className="process-section">
          <h3>⚡ Top CPU Usage</h3>
          <div className="process-list">
            {topCPUProcesses.map((proc, index) => (
              <div key={proc.pid} className="process-item">
                <div className="process-rank">#{index + 1}</div>
                <div className="process-info">
                  <span className="process-name">{proc.name}</span>
                  <span className="process-pid">PID: {proc.pid}</span>
                </div>
                <div className="process-metrics">
                  <span
                    className="metric cpu"
                    style={{
                      color:
                        proc.cpu_usage > 50 ? '#ff4757' : '#1e90ff',
                    }}
                  >
                    {proc.cpu_usage.toFixed(1)}%
                  </span>
                </div>
                <div className="memory-bar-container">
                  <div
                    className="memory-bar cpu-bar"
                    style={{
                      width: `${Math.min(proc.cpu_usage, 100)}%`,
                      background:
                        proc.cpu_usage > 50
                          ? 'linear-gradient(90deg, #ffa502, #ff4757)'
                          : 'linear-gradient(90deg, #1e90ff, #2ed573)',
                    }}
                  ></div>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
