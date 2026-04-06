# ⚡ OptiLite - Open Source System Optimizer

A breakthrough system optimization application built with **Rust + Tauri + TypeScript**, designed to help users maximize their system performance by intelligently managing RAM, CPU, GPU resources, and optimizing browser memory usage.

![OptiLite Banner](./banner.png)

## 🚀 Features

### Core Optimization
- **🧹 RAM Cleaning** - Free up memory by optimizing process working sets
- **🌐 Chrome Optimization** - Reduce Chrome tab memory consumption (up to 40% reduction!)
- **⚡ CPU Optimization** - Monitor and optimize high-CPU processes
- **🎮 GPU Monitoring** - Track GPU usage and memory
- **🗑️ Temp File Cleaning** - Remove temporary files to free disk space
- **🔄 DNS Flushing** - Clear DNS cache for faster browsing

### Smart Features
- **📊 Real-time Monitoring** - Live system stats with beautiful visualizations
- **⏱️ Auto-Optimization** - Schedule automatic optimizations at regular intervals
- **📋 Optimization Logs** - Track all optimization activities
- **🎯 Process Monitor** - Detailed view of resource-heavy processes
- **🔔 Smart Alerts** - Get notified when system resources are critical

### UI/UX
- **🎨 Modern Dark Theme** - Beautiful gradient design with smooth animations
- **📱 Responsive Layout** - Works on all screen sizes
- **📈 Circular Progress Indicators** - Intuitive resource usage visualization
- **🎯 Quick Actions** - One-click optimization buttons

## 🛠️ Tech Stack

- **Backend**: Rust (sysinfo crate for system monitoring)
- **Framework**: Tauri (lightweight, secure desktop apps)
- **Frontend**: React + TypeScript
- **Styling**: Custom CSS with modern gradients
- **Build Tool**: Vite (fast HMR and bundling)

## 📦 Installation

### Prerequisites
- Node.js 16+ and npm
- Rust 1.70+ and Cargo
- Tauri CLI

### Setup

1. **Clone the repository**
```bash
git clone https://github.com/OptiLite/OptiLite.git
cd OptiLite
```

2. **Install dependencies**
```bash
npm install
```

3. **Run in development mode**
```bash
npm run dev
```

4. **Build for production**
```bash
npm run build
```

## 🎯 Usage

### Dashboard
The main dashboard provides:
- Real-time RAM and CPU usage with circular progress indicators
- Quick overview of system stats (processes, swap, GPU, temperature, uptime)
- Quick action buttons for common optimizations

### Optimization Panel
Access all optimization features:
- **Full Optimization**: One-click complete system optimization
- **RAM Cleaning**: Free up memory from background processes
- **Chrome Optimization**: Specifically target Chrome browser memory
- **CPU Optimization**: Monitor and manage high-CPU processes
- **Temp File Cleaning**: Remove unnecessary temporary files
- **DNS Flushing**: Clear DNS cache for better performance

### Process Monitor
Detailed process information:
- Chrome processes with resource usage
- Top memory-consuming processes
- Top CPU-intensive processes
- Visual memory/CPU bars

### Settings
Configure preferences:
- Enable/disable auto-optimization
- Set optimization intervals (5min - 2 hours)
- Choose which optimizations to run
- View app information

## 🏗️ Architecture

```
OptiLite/
├── src/                      # Frontend React app
│   ├── components/           # React components
│   │   ├── Dashboard.tsx
│   │   ├── OptimizationPanel.tsx
│   │   ├── ProcessMonitor.tsx
│   │   └── Settings.tsx
│   ├── styles/               # CSS styles
│   │   ├── global.css
│   │   └── App.css
│   ├── App.tsx               # Main app component
│   └── main.tsx              # Entry point
├── src-tauri/                # Rust backend
│   ├── src/
│   │   ├── lib.rs            # Main Rust logic
│   │   └── main.rs           # Tauri entry point
│   ├── Cargo.toml            # Rust dependencies
│   └── tauri.conf.json       # Tauri configuration
└── package.json              # Node dependencies
```

## 🔒 Safety

OptiLite is designed with safety in mind:
- **Non-destructive**: Only optimizes safe-to-modify processes
- **System Process Protection**: Critical system processes are never modified
- **Transparent**: All actions are logged and visible
- **Open Source**: Fully auditable code

## 🤝 Contributing

We welcome contributions! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **sysinfo** crate for system monitoring capabilities
- **Tauri** team for the amazing framework
- **React** community for the excellent ecosystem

## 📧 Contact

- **GitHub**: [@OptiLite](https://github.com/OptiLite)
- **Issues**: [Report bugs or request features](https://github.com/OptiLite/OptiLite/issues)

---

**Made with ❤️ by the OptiLite Team**

*Optimize your system, maximize your performance.*
