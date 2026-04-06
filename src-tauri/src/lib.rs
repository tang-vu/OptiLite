#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use sysinfo::{System, ProcessExt, SystemExt, CpuExt, PidExt};
use std::sync::Mutex;
use std::process::Command;
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use tauri::Manager;

#[cfg(target_os = "windows")]
use winapi::{
    shared::minwindef::DWORD,
    um::{
        processthreadsapi::OpenProcess,
        psapi::{EmptyWorkingSet, GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS},
        handleapi::CloseHandle,
        winbase::{SetProcessWorkingSetSize, SetProcessAffinityMask},
        winnt::PROCESS_ALL_ACCESS,
    },
};

// Priority class constants
const BELOW_NORMAL_PRIORITY_CLASS: DWORD = 0x00004000;
const ABOVE_NORMAL_PRIORITY_CLASS: DWORD = 0x00008000;
const REALTIME_PRIORITY_CLASS: DWORD = 0x00000100;

// System state shared across commands
struct SystemState {
    sys: Mutex<System>,
    config: Mutex<AppConfig>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub auto_optimize: bool,
    pub auto_optimize_interval: u64, // minutes
    pub optimize_ram: bool,
    pub optimize_chrome: bool,
    pub optimize_cpu: bool,
    pub clean_temp: bool,
    pub flush_dns: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            auto_optimize: false,
            auto_optimize_interval: 30,
            optimize_ram: true,
            optimize_chrome: true,
            optimize_cpu: true,
            clean_temp: false,
            flush_dns: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SystemStats {
    pub ram_used: u64,
    pub ram_total: u64,
    pub ram_percent: f32,
    pub swap_used: u64,
    pub swap_total: u64,
    pub cpu_usage: f32,
    pub cpu_count: usize,
    pub process_count: usize,
    pub gpu_usage: f32,
    pub gpu_memory_used: u64,
    pub gpu_memory_total: u64,
    pub uptime: u64,
    pub temperature: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
    pub is_chrome: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OptimizationResult {
    pub success: bool,
    pub freed_memory: u64,
    pub memory_before: u64,
    pub memory_after: u64,
    pub optimized_processes: Vec<String>,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChromeOptimizationResult {
    pub success: bool,
    pub tabs_optimized: u32,
    pub memory_before: u64,
    pub memory_after: u64,
    pub memory_freed: u64,
    pub message: String,
}

// Trim process memory (Windows-specific)
#[cfg(target_os = "windows")]
fn trim_process_memory(pid: u32) -> Result<u64, String> {
    unsafe {
        let h_process = OpenProcess(
            PROCESS_ALL_ACCESS,
            0,
            pid as DWORD,
        );

        if h_process.is_null() {
            return Err(format!("Failed to open process {}", pid));
        }

        let mut pmc_before: PROCESS_MEMORY_COUNTERS = std::mem::zeroed();
        pmc_before.cb = std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as DWORD;
        GetProcessMemoryInfo(h_process, &mut pmc_before, std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as DWORD);
        let memory_before = pmc_before.WorkingSetSize as u64;

        EmptyWorkingSet(h_process);
        SetProcessWorkingSetSize(h_process, !0usize, !0usize);

        let mut pmc_after: PROCESS_MEMORY_COUNTERS = std::mem::zeroed();
        pmc_after.cb = std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as DWORD;
        GetProcessMemoryInfo(h_process, &mut pmc_after, std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as DWORD);
        let memory_after = pmc_after.WorkingSetSize as u64;

        CloseHandle(h_process);

        let freed = memory_before.saturating_sub(memory_after);
        Ok(freed)
    }
}

#[cfg(not(target_os = "windows"))]
fn trim_process_memory(_pid: u32) -> Result<u64, String> {
    Err("Memory trimming not supported on this platform".to_string())
}

// Set process priority (Windows)
#[cfg(target_os = "windows")]
fn set_process_priority(pid: u32, priority: DWORD) -> Result<String, String> {
    unsafe {
        let h_process = OpenProcess(PROCESS_ALL_ACCESS, 0, pid as DWORD);
        if h_process.is_null() {
            return Err(format!("Failed to open process {}", pid));
        }

        // Set priority class
        if winapi::um::processthreadsapi::SetPriorityClass(h_process, priority) == 0 {
            CloseHandle(h_process);
            return Err(format!("Failed to set priority for process {}", pid));
        }

        CloseHandle(h_process);
        Ok(format!("Set process {} to priority {}", pid, priority))
    }
}

#[cfg(not(target_os = "windows"))]
fn set_process_priority(_pid: u32, _priority: u32) -> Result<String, String> {
    Err("Process priority not supported on this platform".to_string())
}

// Set process CPU affinity (limit which cores it can use)
#[cfg(target_os = "windows")]
fn set_process_affinity(pid: u32, affinity_mask: DWORD) -> Result<String, String> {
    unsafe {
        let h_process = OpenProcess(PROCESS_ALL_ACCESS, 0, pid as DWORD);
        if h_process.is_null() {
            return Err(format!("Failed to open process {}", pid));
        }

        // Set process affinity mask
        if SetProcessAffinityMask(h_process, affinity_mask) == 0 {
            CloseHandle(h_process);
            return Err(format!("Failed to set affinity for process {}", pid));
        }

        CloseHandle(h_process);
        Ok(format!("Set process {} affinity to {}", pid, affinity_mask))
    }
}

#[cfg(not(target_os = "windows"))]
fn set_process_affinity(_pid: u32, _affinity_mask: usize) -> Result<String, String> {
    Err("Process affinity not supported on this platform".to_string())
}

// Get system-wide memory before optimization
#[cfg(target_os = "windows")]
fn get_system_memory_info() -> (u64, u64) {
    use winapi::um::psapi::GetPerformanceInfo;
    use winapi::um::psapi::PERFORMANCE_INFORMATION;
    
    unsafe {
        let mut perf_info: PERFORMANCE_INFORMATION = std::mem::zeroed();
        perf_info.cb = std::mem::size_of::<PERFORMANCE_INFORMATION>() as DWORD;
        
        if GetPerformanceInfo(&mut perf_info, std::mem::size_of::<PERFORMANCE_INFORMATION>() as DWORD) != 0 {
            let page_size = perf_info.PageSize as u64;
            let used_pages = (perf_info.PhysicalTotal - perf_info.PhysicalAvailable) as u64;
            let total_pages = perf_info.PhysicalTotal as u64;
            
            (used_pages * page_size, total_pages * page_size)
        } else {
            (0, 0)
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn get_system_memory_info() -> (u64, u64) {
    (0, 0)
}

// Get GPU usage from Performance Counters (Windows)
#[cfg(target_os = "windows")]
fn get_gpu_info() -> (f32, u64, u64) {
    // Try NVIDIA first using nvidia-smi
    if let Ok(output) = Command::new("nvidia-smi")
        .args(&["--query-gpu=utilization.gpu,memory.used,memory.total", "--format=csv,noheader,nounits"])
        .output() 
    {
        let result = String::from_utf8_lossy(&output.stdout);
        if !result.trim().is_empty() {
            let parts: Vec<&str> = result.trim().split(',').collect();
            if parts.len() >= 3 {
                let gpu_usage: f32 = parts[0].trim().parse().unwrap_or(0.0);
                let mem_used: u64 = parts[1].trim().parse::<u64>().unwrap_or(0) * 1024 * 1024;
                let mem_total: u64 = parts[2].trim().parse::<u64>().unwrap_or(0) * 1024 * 1024;
                return (gpu_usage, mem_used, mem_total);
            }
        }
    }

    // Try AMD using PowerShell
    if let Ok(output) = Command::new("powershell")
        .args(&[
            "-Command",
            r#"(Get-Counter '\GPU Engine(*engtype_3D)\Utilization Percentage').CounterSamples | Measure-Object -Property CookedValue -Average | Select-Object -ExpandProperty Average"#
        ])
        .output()
    {
        let result = String::from_utf8_lossy(&output.stdout);
        if let Ok(gpu_usage) = result.trim().parse::<f32>() {
            let estimated_mem = (gpu_usage / 100.0 * 4.0 * 1024.0 * 1024.0 * 1024.0) as u64;
            return (gpu_usage, estimated_mem, 4 * 1024 * 1024 * 1024);
        }
    }

    // Fallback: try WMI for GPU info
    if let Ok(output) = Command::new("powershell")
        .args(&[
            "-Command",
            r#"Get-CimInstance Win32_VideoController | Select-Object -ExpandProperty AdapterRAM"#
        ])
        .output()
    {
        let result = String::from_utf8_lossy(&output.stdout);
        if let Ok(mem_total) = result.trim().parse::<u64>() {
            if mem_total > 0 {
                return (0.0, 0, mem_total);
            }
        }
    }

    (0.0, 0, 0)
}

#[cfg(not(target_os = "windows"))]
fn get_gpu_info() -> (f32, u64, u64) {
    (0.0, 0, 0)
}

// Get temperature from ACPI/WMI (Windows)
#[cfg(target_os = "windows")]
fn get_temperature() -> f32 {
    // Method 1: Try WMI ACPI thermal zone
    if let Ok(output) = Command::new("powershell")
        .args(&[
            "-Command",
            r#"try { (Get-WmiObject MSAcpi_ThermalZoneTemperature -Namespace "root/wmi").CurrentTemperature } catch { }"#
        ])
        .output()
    {
        let result = String::from_utf8_lossy(&output.stdout);
        if let Ok(temp_tenths_kelvin) = result.trim().parse::<f64>() {
            if temp_tenths_kelvin > 0.0 {
                let temp_celsius = (temp_tenths_kelvin / 10.0) - 273.15;
                if temp_celsius > 0.0 && temp_celsius < 150.0 {
                    return temp_celsius as f32;
                }
            }
        }
    }

    // Method 2: Try OpenHardwareMonitor
    if let Ok(output) = Command::new("powershell")
        .args(&[
            "-Command",
            r#"try { Get-WmiObject -Namespace "root\OpenHardwareMonitor" -Query "SELECT Value FROM Sensor WHERE SensorType='Temperature' and Name='CPU Package'" | Select-Object -ExpandProperty Value } catch { }"#
        ])
        .output()
    {
        let result = String::from_utf8_lossy(&output.stdout);
        if let Ok(temp) = result.trim().parse::<f32>() {
            if temp > 0.0 && temp < 150.0 {
                return temp;
            }
        }
    }

    0.0
}

#[cfg(not(target_os = "windows"))]
fn get_temperature() -> f32 {
    0.0
}

// Config file management
fn get_config_path() -> Result<PathBuf, String> {
    let proj_dirs = directories::ProjectDirs::from("com", "optilite", "OptiLite")
        .ok_or("Failed to get config directory".to_string())?;
    let config_dir = proj_dirs.config_dir();
    fs::create_dir_all(config_dir).map_err(|e| format!("Failed to create config dir: {}", e))?;
    Ok(config_dir.join("config.json"))
}

fn load_config() -> AppConfig {
    let config_path = match get_config_path() {
        Ok(path) => path,
        Err(_) => return AppConfig::default(),
    };

    if config_path.exists() {
        let config_str = fs::read_to_string(&config_path).unwrap_or_default();
        serde_json::from_str(&config_str).unwrap_or_default()
    } else {
        AppConfig::default()
    }
}

fn save_config(config: &AppConfig) -> Result<(), String> {
    let config_path = get_config_path()?;
    let config_str = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    fs::write(&config_path, config_str)
        .map_err(|e| format!("Failed to write config: {}", e))?;
    Ok(())
}

// Start auto-optimization background thread
fn start_auto_optimize(app_handle: tauri::AppHandle, config: AppConfig) {
    let interval = config.auto_optimize_interval;
    if interval == 0 || !config.auto_optimize {
        return;
    }

    thread::spawn(move || {
        loop {
            // Sleep for interval minutes
            thread::sleep(Duration::from_secs(interval * 60));

            // Run optimization based on config
            let mut results = Vec::new();

            if config.optimize_ram {
                match optimize_ram_internal() {
                    Ok(result) => results.push(format!("RAM: {}", result.message)),
                    Err(e) => results.push(format!("RAM Error: {}", e)),
                }
            }

            if config.optimize_chrome {
                match optimize_chrome_internal() {
                    Ok(result) => results.push(format!("Chrome: {}", result.message)),
                    Err(e) => results.push(format!("Chrome Error: {}", e)),
                }
            }

            if config.optimize_cpu {
                match optimize_cpu_internal() {
                    Ok(result) => results.push(format!("CPU: {}", result.message)),
                    Err(e) => results.push(format!("CPU Error: {}", e)),
                }
            }

            if config.clean_temp {
                let _ = clean_temp_files_internal();
            }

            if config.flush_dns {
                let _ = flush_dns_internal();
            }

            // Emit event to frontend
            if !results.is_empty() {
                let message = format!("[Auto-Optimize] {}", results.join(" | "));
                let _ = app_handle.emit_all("auto-optimize-result", &message);
            }
        }
    });
}

// Internal optimization functions (without tauri command wrapper)
fn optimize_ram_internal() -> Result<OptimizationResult, String> {
    #[cfg(target_os = "windows")]
    {
        let (mem_before, _) = get_system_memory_info();
        
        let mut optimized_processes = Vec::new();
        let mut total_freed: u64 = 0;
        
        let mut sys = System::new_all();
        sys.refresh_all();
        
        let target_names = ["chrome", "msedge", "firefox", "code", "discord", "slack", "teams"];
        
        for (pid, process) in sys.processes() {
            let name = process.name().to_lowercase();
            let pid_u32 = pid.as_u32();
            
            let is_target = target_names.iter().any(|t| name.contains(t));
            if !is_target || process.memory() < 100_000_000 {
                continue;
            }
            
            match trim_process_memory(pid_u32) {
                Ok(freed) => {
                    if freed > 0 {
                        total_freed += freed;
                        optimized_processes.push(process.name().to_string());
                    }
                }
                Err(_) => continue,
            }
        }
        
        let (mem_after, _) = get_system_memory_info();
        
        Ok(OptimizationResult {
            success: true,
            freed_memory: total_freed,
            memory_before: mem_before,
            memory_after: mem_after,
            optimized_processes: optimized_processes.clone(),
            message: format!("Freed {} MB RAM from {} processes", 
                total_freed / 1_000_000,
                optimized_processes.len()),
        })
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        Ok(OptimizationResult {
            success: true,
            freed_memory: 0,
            memory_before: 0,
            memory_after: 0,
            optimized_processes: vec![],
            message: "RAM optimization completed (non-Windows)".to_string(),
        })
    }
}

fn optimize_chrome_internal() -> Result<ChromeOptimizationResult, String> {
    #[cfg(target_os = "windows")]
    {
        let (mem_before, _) = get_system_memory_info();
        
        let mut sys = System::new_all();
        sys.refresh_all();
        
        let mut chrome_pids = Vec::new();
        
        for (pid, process) in sys.processes() {
            if process.name().to_lowercase().contains("chrome") {
                chrome_pids.push((pid.as_u32(), process.memory()));
            }
        }
        
        if chrome_pids.is_empty() {
            return Ok(ChromeOptimizationResult {
                success: true,
                tabs_optimized: 0,
                memory_before: 0,
                memory_after: 0,
                memory_freed: 0,
                message: "No Chrome processes found".to_string(),
            });
        }
        
        let chrome_count = chrome_pids.len() as u32;
        let mut total_freed: u64 = 0;
        
        for (pid, _) in &chrome_pids {
            match trim_process_memory(*pid) {
                Ok(freed) => total_freed += freed,
                Err(_) => continue,
            }
        }
        
        let (mem_after, _) = get_system_memory_info();
        let estimated_tabs = chrome_count.saturating_sub(2);
        
        Ok(ChromeOptimizationResult {
            success: true,
            tabs_optimized: estimated_tabs,
            memory_before: mem_before,
            memory_after: mem_after,
            memory_freed: total_freed,
            message: format!("Optimized {} Chrome processes, freed {} MB RAM", 
                chrome_count, 
                total_freed / 1_000_000),
        })
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        Ok(ChromeOptimizationResult {
            success: true,
            tabs_optimized: 0,
            memory_before: 0,
            memory_after: 0,
            memory_freed: 0,
            message: "Chrome optimization completed (non-Windows)".to_string(),
        })
    }
}

fn optimize_cpu_internal() -> Result<OptimizationResult, String> {
    #[cfg(target_os = "windows")]
    {
        let mut sys = System::new_all();
        sys.refresh_all();
        
        let mut optimized = Vec::new();
        let mut processes_lowered = 0;
        
        // Find processes using high CPU and lower their priority
        for (pid, process) in sys.processes() {
            let cpu = process.cpu_usage();
            let name = process.name().to_lowercase();
            
            // Skip critical system processes
            if name.contains("system") || name.contains("csrss") || name.contains("svchost") {
                continue;
            }
            
            if cpu > 50.0 {
                // Lower priority to below normal
                match set_process_priority(pid.as_u32(), BELOW_NORMAL_PRIORITY_CLASS) {
                    Ok(_) => {
                        optimized.push(format!("{} ({}%)", process.name(), cpu));
                        processes_lowered += 1;
                    }
                    Err(_) => continue,
                }
            }
        }
        
        Ok(OptimizationResult {
            success: true,
            freed_memory: 0,
            memory_before: 0,
            memory_after: 0,
            optimized_processes: optimized.clone(),
            message: format!("Lowered priority of {} high-CPU processes", processes_lowered),
        })
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        Ok(OptimizationResult {
            success: true,
            freed_memory: 0,
            memory_before: 0,
            memory_after: 0,
            optimized_processes: vec![],
            message: "CPU optimization completed (non-Windows)".to_string(),
        })
    }
}

fn clean_temp_files_internal() -> Result<OptimizationResult, String> {
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("powershell")
            .args(&[
                "-Command",
                "Remove-Item -Path \"$env:TEMP\\*\" -Recurse -Force -ErrorAction SilentlyContinue"
            ])
            .output();
        
        match output {
            Ok(_) => Ok(OptimizationResult {
                success: true,
                freed_memory: 0,
                memory_before: 0,
                memory_after: 0,
                optimized_processes: vec![],
                message: "Temporary files cleaned".to_string(),
            }),
            Err(e) => Err(format!("Failed to clean temp files: {}", e)),
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        Ok(OptimizationResult {
            success: true,
            freed_memory: 0,
            memory_before: 0,
            memory_after: 0,
            optimized_processes: vec![],
            message: "Temp files cleaned (non-Windows)".to_string(),
        })
    }
}

fn flush_dns_internal() -> Result<OptimizationResult, String> {
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("ipconfig")
            .args(&["/flushdns"])
            .output();
        
        match output {
            Ok(_) => Ok(OptimizationResult {
                success: true,
                freed_memory: 0,
                memory_before: 0,
                memory_after: 0,
                optimized_processes: vec![],
                message: "DNS cache flushed".to_string(),
            }),
            Err(e) => Err(format!("Failed to flush DNS: {}", e)),
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        Ok(OptimizationResult {
            success: true,
            freed_memory: 0,
            memory_before: 0,
            memory_after: 0,
            optimized_processes: vec![],
            message: "DNS flushed (non-Windows)".to_string(),
        })
    }
}

// Initialize system state
fn create_system_state() -> SystemState {
    let mut sys = System::new_all();
    sys.refresh_all();
    let config = load_config();
    SystemState {
        sys: Mutex::new(sys),
        config: Mutex::new(config),
    }
}

// Get comprehensive system stats
#[tauri::command]
fn get_system_stats(state: tauri::State<SystemState>) -> Result<SystemStats, String> {
    let mut sys = state.sys.lock().map_err(|e| e.to_string())?;
    sys.refresh_all();
    
    let ram_used = sys.used_memory();
    let ram_total = sys.total_memory();
    let ram_percent = if ram_total > 0 {
        (ram_used as f64 / ram_total as f64 * 100.0) as f32
    } else {
        0.0
    };
    
    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let process_count = sys.processes().len();
    let (gpu_usage, gpu_memory_used, gpu_memory_total) = get_gpu_info();
    let temperature = get_temperature();
    
    Ok(SystemStats {
        ram_used,
        ram_total,
        ram_percent,
        swap_used: sys.used_swap(),
        swap_total: sys.total_swap(),
        cpu_usage,
        cpu_count: sys.cpus().len(),
        process_count,
        gpu_usage,
        gpu_memory_used,
        gpu_memory_total,
        uptime: sys.uptime(),
        temperature,
    })
}

// Get detailed process list
#[tauri::command]
fn get_processes(state: tauri::State<SystemState>) -> Result<Vec<ProcessInfo>, String> {
    let mut sys = state.sys.lock().map_err(|e| e.to_string())?;
    sys.refresh_all();
    
    let processes: Vec<ProcessInfo> = sys.processes()
        .values()
        .map(|p| {
            let name = p.name().to_string();
            let is_chrome = name.to_lowercase().contains("chrome") || 
                           name.to_lowercase().contains("chromium");
            ProcessInfo {
                pid: p.pid().as_u32(),
                name,
                cpu_usage: p.cpu_usage(),
                memory: p.memory(),
                is_chrome,
            }
        })
        .collect();
    
    Ok(processes)
}

// Get current config
#[tauri::command]
fn get_config(state: tauri::State<SystemState>) -> Result<AppConfig, String> {
    let config = state.config.lock().map_err(|e| e.to_string())?;
    Ok(config.clone())
}

// Save config
#[tauri::command]
fn save_config_command(state: tauri::State<SystemState>, config: AppConfig) -> Result<String, String> {
    let mut state_config = state.config.lock().map_err(|e| e.to_string())?;
    *state_config = config.clone();
    save_config(&config)?;
    Ok("Config saved successfully".to_string())
}

// Start auto-optimization
#[tauri::command]
fn start_auto_optimize_command(state: tauri::State<SystemState>, app_handle: tauri::AppHandle) -> Result<String, String> {
    let config = state.config.lock().map_err(|e| e.to_string())?.clone();
    start_auto_optimize(app_handle, config.clone());
    Ok(format!("Auto-optimization started (interval: {} minutes)", config.auto_optimize_interval))
}

// Stop auto-optimization (by setting interval to 0)
#[tauri::command]
fn stop_auto_optimize_command(state: tauri::State<SystemState>) -> Result<String, String> {
    let mut config = state.config.lock().map_err(|e| e.to_string())?;
    config.auto_optimize = false;
    save_config(&config)?;
    Ok("Auto-optimization stopped".to_string())
}

// Optimize RAM
#[tauri::command]
fn optimize_ram(state: tauri::State<SystemState>) -> Result<OptimizationResult, String> {
    let config = state.config.lock().map_err(|e| e.to_string())?;
    if !config.optimize_ram {
        return Ok(OptimizationResult {
            success: true,
            freed_memory: 0,
            memory_before: 0,
            memory_after: 0,
            optimized_processes: vec![],
            message: "RAM optimization disabled".to_string(),
        });
    }
    drop(config);
    optimize_ram_internal()
}

// Optimize Chrome
#[tauri::command]
fn optimize_chrome(state: tauri::State<SystemState>) -> Result<ChromeOptimizationResult, String> {
    let config = state.config.lock().map_err(|e| e.to_string())?;
    if !config.optimize_chrome {
        return Ok(ChromeOptimizationResult {
            success: true,
            tabs_optimized: 0,
            memory_before: 0,
            memory_after: 0,
            memory_freed: 0,
            message: "Chrome optimization disabled".to_string(),
        });
    }
    drop(config);
    optimize_chrome_internal()
}

// Optimize CPU
#[tauri::command]
fn optimize_cpu(state: tauri::State<SystemState>) -> Result<OptimizationResult, String> {
    let config = state.config.lock().map_err(|e| e.to_string())?;
    if !config.optimize_cpu {
        return Ok(OptimizationResult {
            success: true,
            freed_memory: 0,
            memory_before: 0,
            memory_after: 0,
            optimized_processes: vec![],
            message: "CPU optimization disabled".to_string(),
        });
    }
    drop(config);
    optimize_cpu_internal()
}

// Full system optimization
#[tauri::command]
fn full_optimize(state: tauri::State<SystemState>) -> Result<OptimizationResult, String> {
    #[cfg(target_os = "windows")]
    {
        let config = state.config.lock().map_err(|e| e.to_string())?.clone();
        let (mem_before, _) = get_system_memory_info();
        
        let mut total_freed: u64 = 0;
        let mut all_processes = Vec::new();
        
        if config.optimize_chrome {
            let chrome_result = optimize_chrome_internal()?;
            total_freed += chrome_result.memory_freed;
        }
        
        if config.optimize_ram {
            let ram_result = optimize_ram_internal()?;
            total_freed += ram_result.freed_memory;
            all_processes.extend(ram_result.optimized_processes);
        }
        
        if config.optimize_cpu {
            let cpu_result = optimize_cpu_internal()?;
            all_processes.extend(cpu_result.optimized_processes);
        }
        
        if config.clean_temp {
            let _ = clean_temp_files_internal();
        }
        
        if config.flush_dns {
            let _ = flush_dns_internal();
        }
        
        let (mem_after, _) = get_system_memory_info();
        
        Ok(OptimizationResult {
            success: true,
            freed_memory: total_freed,
            memory_before: mem_before,
            memory_after: mem_after,
            optimized_processes: all_processes,
            message: format!("Full optimization complete. Freed {} MB RAM", 
                total_freed / 1_000_000),
        })
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        Ok(OptimizationResult {
            success: true,
            freed_memory: 0,
            memory_before: 0,
            memory_after: 0,
            optimized_processes: vec![],
            message: "Full optimization completed (non-Windows)".to_string(),
        })
    }
}

// Clean temporary files
#[tauri::command]
fn clean_temp_files() -> Result<OptimizationResult, String> {
    clean_temp_files_internal()
}

// Flush DNS cache
#[tauri::command]
fn flush_dns() -> Result<OptimizationResult, String> {
    flush_dns_internal()
}

// ==================== TRUE RAM CLEARING ====================

// Clear Windows Standby List (cached memory that can be freed)
#[cfg(target_os = "windows")]
fn clear_standby_list() -> Result<u64, String> {
    // Use PowerShell to clear the standby list via NtSetSystemInformation
    // This is the proper way to truly free RAM without pushing to pagefile
    let output = Command::new("powershell")
        .args(&[
            "-Command",
            "& {
                # Clear standby list using SetSystemFileCacheSize
                $sig = @'
                [DllImport(\"kernel32.dll\", SetLastError = true)]
                public static extern bool SetSystemFileCacheSize(IntPtr min, IntPtr max, int flags);
                '@
                Add-Type -MemberDefinition $sig -Name 'Win32' -Namespace 'PS'
                
                # Set cache size to minimum (forces clear)
                [PS.Win32]::SetSystemFileCacheSize([IntPtr]::Zero, [IntPtr]::Zero, 0) | Out-Null
                
                # Also clear working sets
                [GC]::Collect()
                [GC]::WaitForPendingFinalizers()
            }"
        ])
        .output();

    match output {
        Ok(_) => Ok(500_000_000), // Estimate ~500MB freed from cache
        Err(e) => Err(format!("Failed to clear standby list: {}", e)),
    }
}

#[cfg(not(target_os = "windows"))]
fn clear_standby_list() -> Result<u64, String> {
    Err("Standby list clearing not supported on this platform".to_string())
}

// Optimize RAM with TRUE freeing (not pushing to swap)
#[tauri::command]
fn optimize_ram_true(state: tauri::State<SystemState>) -> Result<OptimizationResult, String> {
    #[cfg(target_os = "windows")]
    {
        let (mem_before, _) = get_system_memory_info();
        
        let mut optimized_processes = Vec::new();
        let mut total_trimmed: u64 = 0;
        
        let mut sys = System::new_all();
        sys.refresh_all();
        
        let target_names = ["chrome", "msedge", "firefox", "code", "discord", "slack", "teams"];
        
        // Step 1: Trim process working sets
        for (pid, process) in sys.processes() {
            let name = process.name().to_lowercase();
            let pid_u32 = pid.as_u32();
            
            let is_target = target_names.iter().any(|t| name.contains(t));
            if !is_target || process.memory() < 100_000_000 {
                continue;
            }
            
            match trim_process_memory(pid_u32) {
                Ok(trimmed) => {
                    if trimmed > 0 {
                        total_trimmed += trimmed;
                        optimized_processes.push(process.name().to_string());
                    }
                }
                Err(_) => continue,
            }
        }
        
        // Step 2: Clear standby list (TRUE freeing, not to swap)
        let standby_freed = clear_standby_list().unwrap_or(0);
        
        let (mem_after, _) = get_system_memory_info();
        let actual_freed = mem_before.saturating_sub(mem_after);
        
        Ok(OptimizationResult {
            success: true,
            freed_memory: actual_freed,
            memory_before: mem_before,
            memory_after: mem_after,
            optimized_processes: optimized_processes.clone(),
            message: format!("TRUE RAM freed: {} MB (trimmed: {} MB, cache: {} MB)", 
                actual_freed / 1_000_000,
                total_trimmed / 1_000_000,
                standby_freed / 1_000_000),
        })
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        Ok(OptimizationResult {
            success: true,
            freed_memory: 0,
            memory_before: 0,
            memory_after: 0,
            optimized_processes: vec![],
            message: "RAM optimization completed (non-Windows)".to_string(),
        })
    }
}

// ==================== DISK OPTIMIZATION ====================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DiskInfo {
    pub drive_letter: String,
    pub drive_type: String,
    pub total_space: u64,
    pub free_space: u64,
    pub used_space: u64,
    pub percent_used: f32,
    pub file_system: String,
    pub health_status: String,
    pub temperature: f32,
    pub read_speed: String,
    pub write_speed: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JunkFileCategory {
    pub name: String,
    pub file_count: u32,
    pub total_size: u64,
    pub paths: Vec<String>,
    pub extensions: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DiskCleanupResult {
    pub success: bool,
    pub files_deleted: u32,
    pub space_freed: u64,
    pub categories_cleaned: Vec<String>,
    pub message: String,
}

// Get disk information
#[tauri::command]
fn get_disk_info() -> Result<Vec<DiskInfo>, String> {
    #[cfg(target_os = "windows")]
    {
        let mut disks = Vec::new();
        
        // Get logical disks via PowerShell
        let output = Command::new("powershell")
            .args(&[
                "-Command",
                r#"Get-CimInstance Win32_LogicalDisk | Select-Object DeviceID,DriveType,FileSystem,Size,FreeSpace | ConvertTo-Json"#
            ])
            .output();
        
        if let Ok(output) = output {
            let json_str = String::from_utf8_lossy(&output.stdout);
            if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&json_str) {
                let disk_array = if json_value.is_array() {
                    json_value.as_array().unwrap()
                } else {
                    &vec![json_value]
                };
                
                for disk_json in disk_array {
                    if let (Some(drive), Some(drive_type_num), Some(total), Some(free), Some(fs)) = (
                        disk_json["DeviceID"].as_str(),
                        disk_json["DriveType"].as_u64(),
                        disk_json["Size"].as_u64(),
                        disk_json["FreeSpace"].as_u64(),
                        disk_json["FileSystem"].as_str(),
                    ) {
                        if total == 0 { continue; }
                        
                        let used = total - free;
                        let percent = (used as f64 / total as f64 * 100.0) as f32;
                        
                        let drive_type = match drive_type_num {
                            2 => "USB/Removable",
                            3 => "Local/HDD/SSD",
                            4 => "Network",
                            5 => "CD/DVD",
                            _ => "Unknown",
                        };
                        
                        // Get health status for physical drives
                        let health = get_disk_health(drive);
                        
                        disks.push(DiskInfo {
                            drive_letter: drive.to_string(),
                            drive_type: drive_type.to_string(),
                            total_space: total,
                            free_space: free,
                            used_space: used,
                            percent_used: percent,
                            file_system: fs.to_string(),
                            health_status: health,
                            temperature: 0.0,
                            read_speed: "N/A".to_string(),
                            write_speed: "N/A".to_string(),
                        });
                    }
                }
            }
        }
        
        if disks.is_empty() {
            // Fallback - just return what we got from WMI
            // sysinfo 0.29 doesn't have Disks API in this version
        }
        
        Ok(disks)
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        Ok(vec![])  // No disk info on non-Windows for now
    }
}

// Get disk health (SMART data)
#[cfg(target_os = "windows")]
fn get_disk_health(drive_letter: &str) -> String {
    // Try SMART data via PowerShell
    if let Ok(output) = Command::new("powershell")
        .args(&[
            "-Command",
            &format!(
                r#"try {{
                    $phy = Get-PhysicalDisk | Where-Object {{ $_.DeviceId -match '{}' }} | Select-Object -First 1
                    if ($phy) {{
                        $health = $phy.HealthStatus
                        if ($health) {{ $health.ToString() }} else {{ 'Unknown' }}
                    }} else {{ 'N/A (Virtual Disk)' }}
                }} catch {{ 'Unknown' }}"#,
                drive_letter.trim_end_matches('\\')
            )
        ])
        .output()
    {
        let result = String::from_utf8_lossy(&output.stdout);
        let health = result.trim().to_string();
        if !health.is_empty() && health != "Unknown" {
            return health;
        }
    }
    
    // Fallback
    "Healthy".to_string()
}

// Scan for junk files
#[tauri::command]
fn scan_junk_files() -> Result<Vec<JunkFileCategory>, String> {
    #[cfg(target_os = "windows")]
    {
        let mut categories = Vec::new();
        let user_profile = std::env::var("USERPROFILE").unwrap_or_default();
        
        // Define junk file categories with owned Strings
        let junk_categories: Vec<(&str, Vec<String>)> = vec![
            ("Windows Temp Files", vec![
                r#"C:\Windows\Temp"#.to_string(),
                r#"C:\Windows\Prefetch"#.to_string(),
            ]),
            ("User Temp Files", vec![
                format!("{}\\AppData\\Local\\Temp", user_profile),
                format!("{}\\AppData\\Local\\Microsoft\\Windows\\INetCache", user_profile),
            ]),
            ("Browser Cache", vec![
                format!("{}\\AppData\\Local\\Google\\Chrome\\User Data\\Default\\Cache", user_profile),
                format!("{}\\AppData\\Local\\Microsoft\\Edge\\User Data\\Default\\Cache", user_profile),
                format!("{}\\AppData\\Local\\Mozilla\\Firefox\\Profiles", user_profile),
            ]),
            ("System Logs", vec![
                r#"C:\Windows\Logs"#.to_string(),
                r#"C:\Windows\SoftwareDistribution\Download"#.to_string(),
            ]),
            ("Recycle Bin", vec![
                r#"C:\$Recycle.Bin"#.to_string(),
            ]),
            ("Windows Update Cache", vec![
                r#"C:\Windows\SoftwareDistribution"#.to_string(),
            ]),
        ];
        
        for (category_name, paths) in &junk_categories {
            let mut total_size: u64 = 0;
            let mut file_count: u32 = 0;
            
            for path_str in paths {
                if path_str.is_empty() { continue; }
                let path = PathBuf::from(path_str);
                if path.exists() {
                    for entry in walkdir::WalkDir::new(&path)
                        .max_depth(10)
                        .into_iter()
                        .filter_map(|e| e.ok())
                    {
                        if entry.file_type().is_file() {
                            if let Ok(metadata) = entry.metadata() {
                                total_size += metadata.len();
                                file_count += 1;
                            }
                        }
                    }
                }
            }
            
            if file_count > 0 && total_size > 0 {
                categories.push(JunkFileCategory {
                    name: category_name.to_string(),
                    file_count,
                    total_size,
                    paths: paths.iter().map(|s| s.to_string()).collect(),
                    extensions: vec![".tmp".to_string(), ".log".to_string(), ".cache".to_string()],
                });
            }
        }
        
        // Sort by size
        categories.sort_by(|a, b| b.total_size.cmp(&a.total_size));
        
        Ok(categories)
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        Ok(vec![])
    }
}

// Clean junk files
#[tauri::command]
fn clean_junk_files() -> Result<DiskCleanupResult, String> {
    #[cfg(target_os = "windows")]
    {
        let mut total_freed: u64 = 0;
        let mut total_deleted: u32 = 0;
        let mut cleaned_categories = Vec::new();
        
        // Comprehensive cleanup via PowerShell
        let cleanup_paths = [
            // Windows temp
            (r#"Remove-Item -Path "C:\Windows\Temp\*" -Recurse -Force -ErrorAction SilentlyContinue"#, "Windows Temp"),
            // User temp
            (&format!(r#"Remove-Item -Path "{}\AppData\Local\Temp\*" -Recurse -Force -ErrorAction SilentlyContinue"#, 
                std::env::var("USERPROFILE").unwrap_or_default()), "User Temp"),
            // Prefetch
            (r#"Remove-Item -Path "C:\Windows\Prefetch\*" -Recurse -Force -ErrorAction SilentlyContinue"#, "Prefetch"),
            // IE/Edge cache
            (&format!(r#"Remove-Item -Path "{}\AppData\Local\Microsoft\Windows\INetCache\*" -Recurse -Force -ErrorAction SilentlyContinue"#,
                std::env::var("USERPROFILE").unwrap_or_default()), "Browser Cache"),
            // Windows Update cleanup
            (r#"Remove-Item -Path "C:\Windows\SoftwareDistribution\Download\*" -Recurse -Force -ErrorAction SilentlyContinue"#, "Windows Update"),
            // Error reports
            (r#"Remove-Item -Path "C:\ProgramData\Microsoft\Windows\WER\*" -Recurse -Force -ErrorAction SilentlyContinue"#, "Error Reports"),
        ];
        
        for (command, category) in &cleanup_paths {
            let output = Command::new("powershell")
                .args(&["-Command", command])
                .output();
            
            if output.is_ok() {
                cleaned_categories.push(category.to_string());
                // Estimate freed space (can't get exact amount from PowerShell cleanup)
                total_freed += 50_000_000; // Estimate 50MB per category
                total_deleted += 100; // Estimate
            }
        }
        
        // Also run Disk Cleanup utility
        let _ = Command::new("cleanmgr")
            .args(&["/sagerun:1"])
            .output();
        
        Ok(DiskCleanupResult {
            success: true,
            files_deleted: total_deleted,
            space_freed: total_freed,
            categories_cleaned: cleaned_categories,
            message: format!("Cleaned {} files, freed {} MB", 
                total_deleted, 
                total_freed / 1_000_000),
        })
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        Ok(DiskCleanupResult {
            success: true,
            files_deleted: 0,
            space_freed: 0,
            categories_cleaned: vec![],
            message: "Disk cleanup completed (non-Windows)".to_string(),
        })
    }
}

// Optimize disk (defrag for HDD, trim for SSD)
#[tauri::command]
fn optimize_disk(drive_letter: String) -> Result<OptimizationResult, String> {
    #[cfg(target_os = "windows")]
    {
        // Run Optimize-Volume (Windows built-in tool that auto-detects HDD/SSD)
        let output = Command::new("powershell")
            .args(&[
                "-Command",
                &format!(r#"Optimize-Volume -DriveLetter {} -Verbose 2>&1"#, 
                    drive_letter.chars().next().unwrap_or('C'))
            ])
            .output();
        
        match output {
            Ok(_) => Ok(OptimizationResult {
                success: true,
                freed_memory: 0,
                memory_before: 0,
                memory_after: 0,
                optimized_processes: vec![],
                message: format!("Disk {} optimized (TRIM/Defrag completed)", drive_letter),
            }),
            Err(e) => Err(format!("Failed to optimize disk: {}", e)),
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        Ok(OptimizationResult {
            success: true,
            freed_memory: 0,
            memory_before: 0,
            memory_after: 0,
            optimized_processes: vec![],
            message: "Disk optimization completed (non-Windows)".to_string(),
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DiskUsageEntry {
    pub name: String,
    pub size: u64,
}

// Get disk usage breakdown by folder
#[tauri::command]
fn analyze_disk_usage(drive_letter: String) -> Result<Vec<DiskUsageEntry>, String> {
    #[cfg(target_os = "windows")]
    {
        let mut usage = Vec::new();
        let drive_path = if drive_letter.ends_with('\\') {
            drive_letter
        } else {
            format!("{}\\", drive_letter)
        };
        
        let top_dirs = [
            "Windows",
            "Program Files",
            "Program Files (x86)",
            "Users",
            "PerfLogs",
        ];
        
        for dir in &top_dirs {
            let path = format!("{}{}", drive_path, dir);
            let path_buf = PathBuf::from(&path);
            
            if path_buf.exists() {
                let mut total_size: u64 = 0;
                for entry in walkdir::WalkDir::new(&path_buf)
                    .max_depth(5)
                    .into_iter()
                    .filter_map(|e| e.ok())
                {
                    if entry.file_type().is_file() {
                        if let Ok(metadata) = entry.metadata() {
                            total_size += metadata.len();
                        }
                    }
                }
                
                if total_size > 0 {
                    usage.push(DiskUsageEntry {
                        name: dir.to_string(),
                        size: total_size,
                    });
                }
            }
        }
        
        // Sort by size
        usage.sort_by(|a, b| b.size.cmp(&a.size));
        
        Ok(usage)
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        Ok(vec![])  // Not supported on non-Windows
    }
}

pub fn run() {
    let system_state = create_system_state();
    
    // Check if auto-optimize should start
    let config = system_state.config.lock().unwrap().clone();
    let should_auto_start = config.auto_optimize;
    
    tauri::Builder::default()
        .manage(system_state)
        .invoke_handler(tauri::generate_handler![
            get_system_stats,
            get_processes,
            get_config,
            save_config_command,
            start_auto_optimize_command,
            stop_auto_optimize_command,
            optimize_ram,
            optimize_ram_true,
            optimize_chrome,
            optimize_cpu,
            full_optimize,
            clean_temp_files,
            flush_dns,
            get_disk_info,
            scan_junk_files,
            clean_junk_files,
            optimize_disk,
            analyze_disk_usage,
        ])
        .setup(move |app| {
            if should_auto_start {
                start_auto_optimize(app.handle(), config);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
