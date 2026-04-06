interface OptimizationPanelProps {
  onOptimizeRAM: () => void;
  onOptimizeChrome: () => void;
  onOptimizeCPU: () => void;
  onFullOptimize: () => void;
  onCleanTempFiles: () => void;
  onFlushDNS: () => void;
  isOptimizing: boolean;
  optimizationLog: string[];
}

export default function OptimizationPanel({
  onOptimizeRAM,
  onOptimizeChrome,
  onOptimizeCPU,
  onFullOptimize,
  onCleanTempFiles,
  onFlushDNS,
  isOptimizing,
  optimizationLog,
}: OptimizationPanelProps) {
  return (
    <div className="optimization-panel">
      <div className="optimize-header">
        <h2>⚡ System Optimization</h2>
        <p>Optimize your system for better performance</p>
      </div>

      <div className="optimize-grid">
        <div className="optimize-card featured">
          <div className="card-icon">🚀</div>
          <h3>Full Optimization</h3>
          <p>Complete system optimization including RAM, CPU, and Chrome</p>
          <button
            className="btn-optimize-primary"
            onClick={onFullOptimize}
            disabled={isOptimizing}
          >
            {isOptimizing ? (
              <>
                <span className="spinner"></span>
                Optimizing...
              </>
            ) : (
              'Optimize Now'
            )}
          </button>
        </div>

        <div className="optimize-grid-2">
          <div className="optimize-card">
            <div className="card-icon">🧹</div>
            <h3>Clean RAM</h3>
            <p>Free up memory by optimizing process working sets</p>
            <button onClick={onOptimizeRAM} disabled={isOptimizing}>
              Clean RAM
            </button>
          </div>

          <div className="optimize-card">
            <div className="card-icon">🌐</div>
            <h3>Optimize Chrome</h3>
            <p>Reduce Chrome tab memory consumption</p>
            <button onClick={onOptimizeChrome} disabled={isOptimizing}>
              Optimize Chrome
            </button>
          </div>

          <div className="optimize-card">
            <div className="card-icon">⚡</div>
            <h3>Optimize CPU</h3>
            <p>Monitor and optimize high-CPU processes</p>
            <button onClick={onOptimizeCPU} disabled={isOptimizing}>
              Optimize CPU
            </button>
          </div>

          <div className="optimize-card">
            <div className="card-icon">🗑️</div>
            <h3>Clean Temp Files</h3>
            <p>Remove temporary files to free disk space</p>
            <button onClick={onCleanTempFiles} disabled={isOptimizing}>
              Clean Temp
            </button>
          </div>

          <div className="optimize-card">
            <div className="card-icon">🔄</div>
            <h3>Flush DNS</h3>
            <p>Clear DNS cache for faster browsing</p>
            <button onClick={onFlushDNS} disabled={isOptimizing}>
              Flush DNS
            </button>
          </div>
        </div>
      </div>

      {optimizationLog.length > 0 && (
        <div className="optimization-log">
          <h3>📋 Optimization Log</h3>
          <div className="log-entries">
            {optimizationLog.slice(0, 20).map((log, index) => (
              <div key={index} className="log-entry">
                {log}
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
