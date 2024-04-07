use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;
use tauri::api::path;
use tauri::Manager;

#[tauri::command]
pub fn get_directory_item(directory_path: &str) {}
