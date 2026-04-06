// Prevents additional console window on Windows in release, may be removed
#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

fn main() {
    optilite::run()
}
