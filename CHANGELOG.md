# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added
- Coming soon...

---

## [1.0.0] - 2026-04-06

### 🎉 Initial Release

#### ✅ RAM Optimization
- TRUE RAM freeing using `EmptyWorkingSet` API
- Clear Windows Standby List with `SetSystemFileCacheSize`
- No longer pushes memory to Swap/PageFile
- Smart process targeting (Chrome, Edge, Firefox, VSCode, Discord, Slack, Teams)
- Shows breakdown: trimmed vs cache freed

#### ✅ Chrome Optimization
- Reduce Chrome tab memory consumption by 15-40%
- Per-process memory trimming
- Estimate tabs optimized from process count

#### ✅ CPU Optimization
- Lower priority of high-CPU processes (>50% usage)
- Skip critical system processes (system, csrss, svchost)
- Uses `SetPriorityClass` Windows API

#### ✅ Disk Optimization Suite
- **Health Monitoring**: SMART data via `Get-PhysicalDisk`
- **Junk File Scanner**: 6 categories (Temp, Prefetch, Browser Cache, Logs, Recycle Bin, Windows Update)
- **One-Click Cleanup**: Remove junk files safely
- **Disk Optimizer**: Auto-detect SSD (TRIM) vs HDD (Defrag)
- **Usage Analyzer**: Breakdown by top folders with visual bars

#### ✅ GPU Monitoring
- **NVIDIA**: Real usage via `nvidia-smi`
- **AMD**: PowerShell performance counters
- VRAM used/total tracking

#### ✅ Temperature Sensors
- WMI ACPI thermal zone reading
- OpenHardwareMonitor fallback
- Color-coded health indicators (Cool/Warm/Hot)

#### ✅ Real-Time Dashboard
- Circular progress indicators for RAM & CPU
- Color-coded health (green <60% / orange 60-80% / red >80%)
- Process count, swap, GPU, temperature, uptime
- Quick one-click optimization button

#### ✅ Process Monitor
- Chrome process tracking (with green accent)
- Top 10 memory consumers with visual bars
- Top 10 CPU consumers with color coding
- Live process statistics

#### ✅ Auto-Optimization
- Background scheduler (5 min - 2 hours intervals)
- Respects enabled/disabled optimization targets
- Emits events to frontend for logging
- Starts automatically on app launch if enabled

#### ✅ Settings Persistence
- JSON config file in user config directory (`%LOCALAPPDATA%`)
- All checkboxes functional and persist across restarts
- Real-time save with visual feedback ("✅ Saved!")

#### ✅ UI/UX
- Modern dark theme with purple/cyan gradients
- Smooth animations and transitions
- Responsive layout for all screen sizes
- Intuitive tab navigation
- Professional dashboard design
- Optimization log tracking

### 🛠️ Technical
- **Backend**: Rust 1.70+ with Tauri 1.5
- **Frontend**: React 18 + TypeScript 5.3
- **Build**: Vite 5.1
- **System Monitoring**: sysinfo 0.29
- **Windows API**: winapi 0.3
- **Safety**: Non-destructive operations, system process protection
- **Cross-Platform**: Windows, macOS, Linux ready

### 📦 Build & Distribution
- GitHub Actions CI/CD pipeline
- Automated builds for Windows, macOS, Linux
- NSIS/MSI installers for Windows
- DMG installer for macOS
- DEB package and AppImage for Linux
- Auto-tagging on main branch

---

[Unreleased]: https://github.com/tang-vu/OptiLite/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/tang-vu/OptiLite/releases/tag/v1.0.0
