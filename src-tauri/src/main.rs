// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod git;

use std::path::PathBuf;

use git::get_changed_files;
use serde_json::json;
use tauri::{Manager, Wry};
use tauri_plugin_store::{with_store, StoreCollection};

use crate::git::{move_all_files, move_file, publish, rebase_onto_source_branch};

#[derive(Debug)]
pub struct AppState {
    repo_path: String,
    source_branch: String,
    working_branch: String,
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let stores = app.state::<StoreCollection<Wry>>();
            let path = PathBuf::from(".settings.dat");
            let empty_json_string = &json!("");

            let new_state = with_store(app.handle(), stores, path, |store| {
                let repo_path = store.get("repo-path").unwrap_or(empty_json_string);
                let working_branch = store.get("working-branch").unwrap_or(empty_json_string);
                let source_branch = store.get("source-branch").unwrap_or(empty_json_string);

                Ok(AppState {
                    repo_path: repo_path.as_str().unwrap_or("").to_string(),
                    working_branch: working_branch.as_str().unwrap_or("").to_string(),
                    source_branch: source_branch.as_str().unwrap_or("").to_string(),
                })
            })?;

            app.manage(new_state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_changed_files,
            move_file,
            move_all_files,
            publish,
            rebase_onto_source_branch,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
