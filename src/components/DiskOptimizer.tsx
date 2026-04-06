import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

interface DiskInfo {
  drive_letter: string;
  drive_type: string;
  total_space: number;
  free_space: number;
  used_space: number;
  percent_used: number;
  file_system: string;
  health_status: string;
  temperature: number;
  read_speed: string;
  write_speed: string;
}

interface JunkFileCategory {
  name: string;
  file_count: number;
  total_size: number;
  paths: string[];
  extensions: string[];
}

interface DiskCleanupResult {
  success: boolean;
  files_deleted: number;
  space_freed: number;
  categories_cleaned: string[];
  message: string;
}

interface DiskUsageEntry {
  name: string;
  size: number;
}

function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
}

export default function DiskOptimizer() {
  const [disks, setDisks] = useState<DiskInfo[]>([]);
  const [junkFiles, setJunkFiles] = useState<JunkFileCategory[]>([]);
  const [diskUsage, setDiskUsage] = useState<DiskUsageEntry[]>([]);
  const [isScanning, setIsScanning] = useState(false);
  const [isCleaning, setIsCleaning] = useState(false);
  const [cleanupLog, setCleanupLog] = useState<string[]>([]);
  const [selectedDrive, setSelectedDrive] = useState('C');

  useEffect(() => {
    loadDiskInfo();
  }, []);

  const loadDiskInfo = async () => {
    try {
      const diskList = await invoke<DiskInfo[]>('get_disk_info');
      setDisks(diskList);
      if (diskList.length > 0) {
        setSelectedDrive(diskList[0].drive_letter.charAt(0));
        analyzeUsage(diskList[0].drive_letter);
      }
    } catch (error) {
      console.error('Failed to load disk info:', error);
    }
  };

  const analyzeUsage = async (drive: string) => {
    try {
      const usage = await invoke<DiskUsageEntry[]>('analyze_disk_usage', { driveLetter: drive });
      setDiskUsage(usage);
    } catch (error) {
      console.error('Failed to analyze disk usage:', error);
    }
  };

  const scanJunkFiles = async () => {
    setIsScanning(true);
    try {
      const junk = await invoke<JunkFileCategory[]>('scan_junk_files');
      setJunkFiles(junk);
      setCleanupLog(prev => [
        `[${new Date().toLocaleTimeString()}] Scan complete: Found ${junk.length} junk categories`,
        ...prev,
      ]);
    } catch (error) {
      console.error('Failed to scan junk files:', error);
    } finally {
      setIsScanning(false);
    }
  };

  const cleanJunkFiles = async () => {
    setIsCleaning(true);
    try {
      const result = await invoke<DiskCleanupResult>('clean_junk_files');
      setCleanupLog(prev => [
        `[${new Date().toLocaleTimeString()}] ✅ Cleanup: ${result.message}`,
        ...prev,
      ]);
      loadDiskInfo(); // Refresh disk info
    } catch (error) {
      console.error('Failed to clean junk files:', error);
    } finally {
      setIsCleaning(false);
    }
  };

  const optimizeDrive = async () => {
    setIsCleaning(true);
    try {
      const result: any = await invoke('optimize_disk', { driveLetter: selectedDrive });
      setCleanupLog(prev => [
        `[${new Date().toLocaleTimeString()}] ✅ Drive ${selectedDrive}: ${result.message}`,
        ...prev,
      ]);
      loadDiskInfo(); // Refresh disk info
    } catch (error) {
      console.error('Failed to optimize drive:', error);
    } finally {
      setIsCleaning(false);
    }
  };

  return (
    <div className="disk-optimizer">
      <div className="disk-header">
        <h2>💾 Disk Optimization</h2>
        <p>Monitor disk health, clean junk files, and optimize performance</p>
      </div>

      {/* Disk Overview */}
      <div className="disks-grid">
        {disks.map((disk, index) => (
          <div 
            key={index} 
            className={`disk-card ${disk.drive_letter.charAt(0) === selectedDrive ? 'selected' : ''}`}
            onClick={() => {
              setSelectedDrive(disk.drive_letter.charAt(0));
              analyzeUsage(disk.drive_letter);
            }}
          >
            <div className="disk-card-header">
              <div className="disk-icon">{disk.drive_type.includes('SSD') ? '💾' : '💿'}</div>
              <div className="disk-info">
                <h3>Drive {disk.drive_letter}</h3>
                <span className="disk-type">{disk.drive_type}</span>
              </div>
              <div className={`health-badge ${disk.health_status.toLowerCase()}`}>
                {disk.health_status}
              </div>
            </div>

            <div className="disk-stats">
              <div className="stat-row">
                <span className="stat-label">Capacity</span>
                <span className="stat-value">{formatBytes(disk.total_space)}</span>
              </div>
              <div className="stat-row">
                <span className="stat-label">Used</span>
                <span className="stat-value">{formatBytes(disk.used_space)} ({disk.percent_used.toFixed(0)}%)</span>
              </div>
              <div className="stat-row">
                <span className="stat-label">Free</span>
                <span className="stat-value">{formatBytes(disk.free_space)}</span>
              </div>
            </div>

            <div className="disk-usage-bar">
              <div 
                className={`disk-usage-fill ${disk.percent_used > 90 ? 'critical' : disk.percent_used > 75 ? 'warning' : ''}`}
                style={{ width: `${Math.min(disk.percent_used, 100)}%` }}
              ></div>
            </div>
          </div>
        ))}
      </div>

      {/* Disk Usage Breakdown */}
      {diskUsage.length > 0 && (
        <div className="disk-usage-section">
          <h3>📊 Drive {selectedDrive} Usage Breakdown</h3>
          <div className="usage-breakdown">
            {diskUsage.map((entry, index) => (
              <div key={index} className="usage-item">
                <div className="usage-info">
                  <span className="usage-name">📁 {entry.name}</span>
                </div>
                <div className="usage-size">{formatBytes(entry.size)}</div>
                <div className="usage-bar-container">
                  <div 
                    className="usage-bar"
                    style={{ width: `${(entry.size / diskUsage[0].size) * 100}%` }}
                  ></div>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Junk File Scanner */}
      <div className="junk-scanner-section">
        <div className="scanner-header">
          <h3>🗑️ Junk File Scanner</h3>
          <div className="scanner-actions">
            <button 
              className="btn-scan"
              onClick={scanJunkFiles}
              disabled={isScanning}
            >
              {isScanning ? '🔍 Scanning...' : '🔍 Scan Junk Files'}
            </button>
            <button 
              className="btn-clean"
              onClick={cleanJunkFiles}
              disabled={isCleaning || junkFiles.length === 0}
            >
              {isCleaning ? '🧹 Cleaning...' : '🧹 Clean All Junk Files'}
            </button>
          </div>
        </div>

        {junkFiles.length > 0 && (
          <div className="junk-categories">
            {junkFiles.map((category, index) => (
              <div key={index} className="junk-card">
                <div className="junk-header">
                  <span className="junk-name">{category.name}</span>
                  <span className="junk-size">{formatBytes(category.total_size)}</span>
                </div>
                <div className="junk-details">
                  <span className="junk-count">{category.file_count} files</span>
                  <span className="junk-paths">{category.paths.length} locations</span>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Disk Optimization */}
      <div className="disk-optimize-section">
        <h3>⚡ Drive Optimization</h3>
        <div className="optimize-drive-card">
          <div className="optimize-drive-info">
            <span className="drive-select-label">Selected Drive:</span>
            <select 
              value={selectedDrive}
              onChange={(e) => {
                setSelectedDrive(e.target.value);
                analyzeUsage(e.target.value);
              }}
            >
              {disks.map((disk, i) => (
                <option key={i} value={disk.drive_letter.charAt(0)}>
                  {disk.drive_letter} ({disk.drive_type})
                </option>
              ))}
            </select>
          </div>
          <button 
            className="btn-optimize-drive"
            onClick={optimizeDrive}
            disabled={isCleaning}
          >
            {isCleaning ? '⚙️ Optimizing...' : `⚡ Optimize Drive ${selectedDrive}`}
          </button>
          <p className="optimize-note">
            {disks.find(d => d.drive_letter.charAt(0) === selectedDrive)?.drive_type.includes('SSD') 
              ? 'Will run TRIM command for SSD' 
              : 'Will run defragmentation for HDD'}
          </p>
        </div>
      </div>

      {/* Cleanup Log */}
      {cleanupLog.length > 0 && (
        <div className="cleanup-log">
          <h3>📋 Cleanup Log</h3>
          <div className="log-entries">
            {cleanupLog.slice(0, 20).map((log, index) => (
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
