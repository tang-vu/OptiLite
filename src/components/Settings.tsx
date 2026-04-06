interface SettingsProps {
  autoOptimize: boolean;
  onAutoOptimizeChange: (value: boolean) => void;
  autoOptimizeInterval: number;
  onAutoOptimizeIntervalChange: (value: number) => void;
}

export default function Settings({
  autoOptimize,
  onAutoOptimizeChange,
  autoOptimizeInterval,
  onAutoOptimizeIntervalChange,
}: SettingsProps) {
  return (
    <div className="settings-panel">
      <div className="settings-header">
        <h2>⚙️ Settings</h2>
        <p>Configure optimization preferences</p>
      </div>

      <div className="settings-grid">
        <div className="settings-section">
          <h3>🤖 Auto Optimization</h3>
          
          <div className="setting-item">
            <div className="setting-info">
              <label>Enable Auto Optimization</label>
              <p>Automatically optimize your system at regular intervals</p>
            </div>
            <label className="toggle-switch">
              <input
                type="checkbox"
                checked={autoOptimize}
                onChange={(e) => onAutoOptimizeChange(e.target.checked)}
              />
              <span className="toggle-slider"></span>
            </label>
          </div>

          {autoOptimize && (
            <div className="setting-item">
              <div className="setting-info">
                <label>Interval (minutes)</label>
                <p>How often to run automatic optimization</p>
              </div>
              <select
                value={autoOptimizeInterval}
                onChange={(e) => onAutoOptimizeIntervalChange(Number(e.target.value))}
              >
                <option value={5}>5 minutes</option>
                <option value={10}>10 minutes</option>
                <option value={15}>15 minutes</option>
                <option value={30}>30 minutes</option>
                <option value={60}>1 hour</option>
                <option value={120}>2 hours</option>
              </select>
            </div>
          )}
        </div>

        <div className="settings-section">
          <h3>🎯 Optimization Targets</h3>
          
          <div className="setting-item checkbox-item">
            <div className="setting-info">
              <label>RAM Optimization</label>
              <p>Clean unused memory from background processes</p>
            </div>
            <input type="checkbox" defaultChecked />
          </div>

          <div className="setting-item checkbox-item">
            <div className="setting-info">
              <label>Chrome Optimization</label>
              <p>Reduce memory usage of Chrome tabs</p>
            </div>
            <input type="checkbox" defaultChecked />
          </div>

          <div className="setting-item checkbox-item">
            <div className="setting-info">
              <label>CPU Optimization</label>
              <p>Optimize high-CPU processes</p>
            </div>
            <input type="checkbox" defaultChecked />
          </div>

          <div className="setting-item checkbox-item">
            <div className="setting-info">
              <label>Temp File Cleaning</label>
              <p>Clean temporary files automatically</p>
            </div>
            <input type="checkbox" />
          </div>

          <div className="setting-item checkbox-item">
            <div className="setting-info">
              <label>DNS Flushing</label>
              <p>Clear DNS cache periodically</p>
            </div>
            <input type="checkbox" />
          </div>
        </div>

        <div className="settings-section">
          <h3>ℹ️ About OptiLite</h3>
          
          <div className="about-card">
            <div className="about-logo">
              <span className="about-icon">⚡</span>
              <h4>OptiLite</h4>
            </div>
            <p className="about-version">Version 1.0.0</p>
            <p className="about-desc">
              Open Source System Optimizer built with Rust + Tauri + TypeScript.
              Designed to help you get the most out of your system resources.
            </p>
            <div className="about-links">
              <a href="https://github.com/OptiLite/OptiLite" target="_blank" rel="noopener noreferrer">
                📦 GitHub Repository
              </a>
              <a href="#" target="_blank" rel="noopener noreferrer">
                📖 Documentation
              </a>
              <a href="#" target="_blank" rel="noopener noreferrer">
                🐛 Report Issue
              </a>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
