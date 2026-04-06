import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

interface AppConfig {
  auto_optimize: boolean;
  auto_optimize_interval: number;
  optimize_ram: boolean;
  optimize_chrome: boolean;
  optimize_cpu: boolean;
  clean_temp: boolean;
  flush_dns: boolean;
}

interface SettingsProps {
  // Config is now managed internally
}

export default function Settings({}: SettingsProps) {
  const [config, setConfig] = useState<AppConfig>({
    auto_optimize: false,
    auto_optimize_interval: 30,
    optimize_ram: true,
    optimize_chrome: true,
    optimize_cpu: true,
    clean_temp: false,
    flush_dns: false,
  });
  const [saved, setSaved] = useState(false);

  // Load config on mount
  useEffect(() => {
    loadConfig();
  }, []);

  const loadConfig = async () => {
    try {
      const loadedConfig = await invoke<AppConfig>('get_config');
      setConfig(loadedConfig);
    } catch (error) {
      console.error('Failed to load config:', error);
    }
  };

  const saveConfig = async (newConfig: AppConfig) => {
    try {
      await invoke('save_config_command', { config: newConfig });
      setSaved(true);
      setTimeout(() => setSaved(false), 2000);
      
      // If auto-optimize enabled, restart it
      if (newConfig.auto_optimize) {
        await invoke('start_auto_optimize_command');
      } else {
        await invoke('stop_auto_optimize_command');
      }
    } catch (error) {
      console.error('Failed to save config:', error);
    }
  };

  const updateConfig = (updates: Partial<AppConfig>) => {
    const newConfig = { ...config, ...updates };
    setConfig(newConfig);
    saveConfig(newConfig);
  };

  return (
    <div className="settings-panel">
      <div className="settings-header">
        <h2>⚙️ Settings</h2>
        <p>Configure optimization preferences</p>
        {saved && (
          <div className="save-notification">✅ Saved!</div>
        )}
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
                checked={config.auto_optimize}
                onChange={(e) => updateConfig({ auto_optimize: e.target.checked })}
              />
              <span className="toggle-slider"></span>
            </label>
          </div>

          {config.auto_optimize && (
            <div className="setting-item">
              <div className="setting-info">
                <label>Interval (minutes)</label>
                <p>How often to run automatic optimization</p>
              </div>
              <select
                value={config.auto_optimize_interval}
                onChange={(e) => updateConfig({ auto_optimize_interval: Number(e.target.value) })}
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
            <input 
              type="checkbox" 
              checked={config.optimize_ram}
              onChange={(e) => updateConfig({ optimize_ram: e.target.checked })}
            />
          </div>

          <div className="setting-item checkbox-item">
            <div className="setting-info">
              <label>Chrome Optimization</label>
              <p>Reduce memory usage of Chrome tabs</p>
            </div>
            <input 
              type="checkbox" 
              checked={config.optimize_chrome}
              onChange={(e) => updateConfig({ optimize_chrome: e.target.checked })}
            />
          </div>

          <div className="setting-item checkbox-item">
            <div className="setting-info">
              <label>CPU Optimization</label>
              <p>Lower priority of high-CPU processes</p>
            </div>
            <input 
              type="checkbox" 
              checked={config.optimize_cpu}
              onChange={(e) => updateConfig({ optimize_cpu: e.target.checked })}
            />
          </div>

          <div className="setting-item checkbox-item">
            <div className="setting-info">
              <label>Temp File Cleaning</label>
              <p>Clean temporary files automatically</p>
            </div>
            <input 
              type="checkbox" 
              checked={config.clean_temp}
              onChange={(e) => updateConfig({ clean_temp: e.target.checked })}
            />
          </div>

          <div className="setting-item checkbox-item">
            <div className="setting-info">
              <label>DNS Flushing</label>
              <p>Clear DNS cache periodically</p>
            </div>
            <input 
              type="checkbox" 
              checked={config.flush_dns}
              onChange={(e) => updateConfig({ flush_dns: e.target.checked })}
            />
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
