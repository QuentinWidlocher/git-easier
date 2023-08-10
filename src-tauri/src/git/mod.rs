use git2::{Repository, Status};
use serde::Serialize;
use tauri::api::process::Command;

use crate::AppState;

#[derive(Serialize)]
pub struct File {
    path: String,
    statuses: Vec<String>,
}

fn status_flags_to_list(status: Status) -> Vec<String> {
    let mut flags = Vec::new();

    if status.is_index_new() {
        flags.push("index_new".to_string());
    }

    if status.is_index_modified() {
        flags.push("index_modified".to_string());
    }

    if status.is_index_deleted() {
        flags.push("index_deleted".to_string());
    }

    if status.is_index_renamed() {
        flags.push("index_renamed".to_string());
    }

    if status.is_index_typechange() {
        flags.push("index_typechange".to_string());
    }

    if status.is_wt_new() {
        flags.push("wt_new".to_string());
    }

    if status.is_wt_modified() {
        flags.push("wt_modified".to_string());
    }

    if status.is_wt_deleted() {
        flags.push("wt_deleted".to_string());
    }

    if status.is_wt_renamed() {
        flags.push("wt_renamed".to_string());
    }

    if status.is_wt_typechange() {
        flags.push("wt_typechange".to_string());
    }

    if status.is_ignored() {
        flags.push("ignored".to_string());
    }

    if status.is_conflicted() {
        flags.push("conflicted".to_string());
    }

    flags
}

fn get_current_repo(repo_path: &String) -> Result<Repository, String> {
    Repository::open(repo_path).map_err(|e| e.to_string())
}

fn git<I, S>(args: I, repo_path: &String) -> Result<(), String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    Command::new("git")
        .current_dir(repo_path.into())
        .args(args)
        .output()
        .map(|output| {
            if output.status.success() {
                Ok(())
            } else {
                Err(output.stderr)
            }
        })
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub fn rebase_onto_source_branch(state: tauri::State<AppState>) -> Result<(), String> {
    git(["stash", "save", "--include-untracked"], &state.repo_path)?;
    git(["checkout", &state.source_branch], &state.repo_path)?;
    git(["pull", "origin"], &state.repo_path)?;
    git(["checkout", &state.working_branch], &state.repo_path)?;
    git(["rebase", &state.source_branch], &state.repo_path)?;
    git(
        [
            "push",
            "--force-with-lease",
            "origin",
            &state.working_branch,
        ],
        &state.repo_path,
    )?;
    git(["stash", "apply"], &state.repo_path)?;
    Ok(())
}

#[tauri::command]
pub fn get_changed_files(state: tauri::State<AppState>) -> Result<Vec<File>, String> {
    let repo = get_current_repo(&state.repo_path)?;

    let repo = repo.statuses(None).map_err(|e| e.to_string())?;

    Ok(repo
        .iter()
        .map(|entry| File {
            path: entry.path().unwrap().to_string(),
            statuses: status_flags_to_list(entry.status()),
        })
        .collect())
}

#[tauri::command]
pub fn move_file(
    path: String,
    action: String,
    state: tauri::State<AppState>,
) -> Result<(), String> {
    match action.as_str() {
        "stage" => git(["add", path.as_str()], &state.repo_path),
        "unstage" => git(["restore", "--staged", path.as_str()], &state.repo_path),
        _ => Err("Unknown action".to_string())?,
    }
}

#[tauri::command]
pub fn move_all_files(action: String, state: tauri::State<AppState>) -> Result<(), String> {
    match action.as_str() {
        "stage" => git(["add ."], &state.repo_path),
        "unstage" => git(["restore", "--staged", "."], &state.repo_path),
        _ => Err("Unknown action".to_string())?,
    }
}

#[tauri::command]
pub fn publish(message: Option<String>, state: tauri::State<AppState>) -> Result<(), String> {
    let commit_message = message.unwrap_or(format!(
        "{}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    ));

    git(["commit", "-m", &commit_message], &state.repo_path)?;

    git(["push", "origin", &state.working_branch], &state.repo_path)?;

    Ok(())
}
