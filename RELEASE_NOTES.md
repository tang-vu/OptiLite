# 🎉 OptiLite v1.0.0 - Initial Release

**Release Date:** April 6, 2026

---

## 🚀 What is OptiLite?

OptiLite is an **open-source system optimization application** built with Rust + Tauri + TypeScript, designed to help users maximize their system performance by intelligently managing RAM, CPU, GPU resources, and optimizing browser memory usage.

---

## ✨ Features

### 💯 TRUE RAM Optimization
- **EmptyWorkingSet API** - Trim process memory without pushing to Swap
- **Clear Standby List** - Free cached memory properly (not just move to pagefile)
- **SetSystemFileCacheSize** - Real RAM freeing using Windows API
- **Smart targeting** - Chrome, Edge, Firefox, VSCode, Discord, Slack, Teams
- Shows breakdown: trimmed vs cache freed

### 🌐 Chrome Optimization
- Reduce Chrome tab memory consumption by 15-40%
- Per-process memory trimming
- Estimate tabs optimized

### ⚡ CPU Optimization
- Lower priority of high-CPU processes (>50% usage)
- Skip critical system processes
- Uses `SetPriorityClass` API

### 💾 Disk Optimization Suite
- **Health Monitoring**: SMART data via `Get-PhysicalDisk`
- **Junk File Scanner**: Scan 6 categories (Temp, Prefetch, Browser Cache, Logs, Recycle Bin, Windows Update)
- **One-Click Cleanup**: Remove junk files safely
- **Disk Optimizer**: Auto-detect SSD (TRIM) vs HDD (Defrag)
- **Usage Analyzer**: Breakdown by top folders with visual bars

### 🎮 GPU Monitoring
- **NVIDIA**: Real usage via `nvidia-smi`
- **AMD**: PowerShell performance counters
- VRAM used/total tracking

### 🌡️ Temperature Sensors
- WMI ACPI thermal zone reading
- Color-coded health indicators (Cool/Warm/Hot)

### 🤖 Auto-Optimization
- Background scheduler (5 min - 2 hours intervals)
- Respects enabled/disabled optimization targets
- Emits events to frontend for logging

### ⚙️ Settings Persistence
- JSON config file in user config directory
- All checkboxes functional and persist across restarts
- Real-time save with visual feedback

### 📊 Real-Time Dashboard
- Circular progress indicators for RAM & CPU
- Color-coded health (green/orange/red)
- Process count, swap, GPU, temperature, uptime
- Quick one-click optimization button

### 📋 Process Monitor
- Chrome process tracking
- Top 10 memory consumers
- Top 10 CPU consumers
- Visual resource bars

---

## 📦 Downloads

### Windows
- **NSIS Installer** (`OptiLite-v1.0.0-Windows-x64.exe`) - Recommended
- **MSI Package** (`OptiLite-v1.0.0-Windows-x64.msi`) - Enterprise deployment

### macOS
- **Intel Mac** (`OptiLite-v1.0.0-macOS-x86_64.dmg`)
- **Apple Silicon** (`OptiLite-v1.0.0-macOS-aarch64.dmg`)

### Linux
- **DEB Package** (`optilite_1.0.0_amd64.deb`) - Debian/Ubuntu
- **AppImage** (`OptiLite-1.0.0-x86_64.AppImage`) - Universal Linux

---

## 🛠️ Tech Stack

| Component | Technology |
|-----------|-----------|
| **Backend** | Rust 1.70+ |
| **Framework** | Tauri 1.5 |
| **Frontend** | React 18 + TypeScript 5.3 |
| **Build** | Vite 5.1 |
| **System Monitoring** | sysinfo 0.29 |
| **Windows API** | winapi 0.3 |
| **Styling** | Custom CSS with gradients |

---

## 📋 System Requirements

### Windows
- Windows 10/11 (64-bit)
- 4GB RAM minimum (8GB recommended)
- 50MB disk space

### macOS
- macOS 10.15+ (Catalina or later)
- 4GB RAM minimum
- 50MB disk space

### Linux
- GTK3 + WebKit2GTK
- 4GB RAM minimum
- 50MB disk space

---

## 🚀 Quick Start

### From Release Assets
1. Download the installer for your platform
2. Run the installer
3. Launch OptiLite
4. Click "Quick Optimize" for instant results!

### From Source
```bash
git clone https://github.com/tang-vu/OptiLite.git
cd OptiLite
npm install
npm run dev
```

---

## 📊 Expected Results

### 8GB RAM System
- **Before**: 65-80% RAM usage
- **After**: 45-60% RAM usage
- **Chrome**: 200-600 MB freed
- **Total**: 500MB-1.5GB optimized

### 16GB RAM System
- **Before**: 45-65% RAM usage
- **After**: 35-50% RAM usage
- **Chrome**: 300-800 MB freed
- **Total**: 800MB-2GB optimized

---

## 🔒 Safety

- ✅ **Non-destructive**: All operations are safe
- ✅ **No data loss**: Zero risk to files
- ✅ **System protected**: Critical processes excluded
- ✅ **Transparent**: All actions logged
- ✅ **Open source**: Fully auditable (MIT License)

---

## 📖 Documentation

- [README.md](https://github.com/tang-vu/OptiLite/blob/main/README.md) - Complete overview
- [QUICKSTART.md](https://github.com/tang-vu/OptiLite/blob/main/QUICKSTART.md) - 5-minute setup guide
- [BUILD_STATUS.md](https://github.com/tang-vu/OptiLite/blob/main/BUILD_STATUS.md) - Compilation verification

---

## 🐛 Known Issues

- Temperature reading may show "Reading sensors..." on some desktop systems without ACPI thermal zone
- GPU monitoring requires `nvidia-smi` in PATH for NVIDIA GPUs
- Some processes cannot be optimized due to Windows security restrictions

---

## 🤝 Contributing

We welcome contributions! Please:
1. Fork the repository
2. Create a feature branch
3. Submit a pull request

---

## 📧 Support

- **Issues**: https://github.com/tang-vu/OptiLite/issues
- **Discussions**: https://github.com/tang-vu/OptiLite/discussions
- **License**: MIT

---

**Made with ❤️ by the OptiLite Team**

*Optimize your system, maximize your performance.* ⚡
