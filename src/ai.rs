// ai.rs

use crate::config::Config;
use crate::git::RepoInfo;
use git2::Repository;
use std::process::Command;
use crate::anthropic::anthropic;

pub fn generate_commit_message(repo_info: &RepoInfo, _config: &Config) -> String {
    // Open the Git repository
    match Repository::open(&repo_info.repo_path) {
        Ok(repo) => {
            let mut changed_files = Vec::new();
            let mut diff_options = git2::DiffOptions::new();
            diff_options.include_untracked(true);
            diff_options.recurse_untracked_dirs(true);
            let diff = repo
                .diff_index_to_workdir(None, Some(&mut diff_options))
                .expect("Failed to compute diff");

            diff.foreach(
                &mut |delta, _| {
                    let file_path = delta.new_file().path().unwrap().to_str().unwrap().to_string();
                    changed_files.push(file_path);
                    true
                },
                None,
                None,
                None,
            )
                .expect("Failed to iterate over diff");

            println!("------------------ START OF CHANGES ------------------");
            for (index, file_path) in changed_files.iter().enumerate() {
                let output = Command::new("git")
                    .arg("diff")
                    .arg(file_path)
                    .output()
                    .expect("Failed to execute git diff command");
                let diff_output = String::from_utf8_lossy(&output.stdout);
                let claude = anthropic(diff_output);
                println!("Changes in file: {}", file_path);
                println!("CLAUDE: {}", claude);
                let add_output = Command::new("git")
                    .arg("add")
                    .arg(file_path)
                    .output()
                    .expect("Failed to execute git add command");
                if !add_output.status.success() {
                    eprintln!("Failed to stage changes for file: {}", file_path);
                    continue;
                }
                let commit_message = format!("{}", claude);
                let commit_output = Command::new("git")
                    .arg("commit")
                    .arg("-m")
                    .arg(commit_message)
                    .output()
                    .expect("Failed to execute git commit command");
                if !commit_output.status.success() {
                    eprintln!("Failed to commit changes for file: {}", file_path);
                    continue;
                }
                if index < changed_files.len() - 1 {
                    println!("------------------ NEXT CHANGE ------------------");
                }
            }
            String::from("------------------ END OF CHANGES ------------------")
        }
        Err(_) => {
            eprintln!("Failed to open Git repository. The current directory may not be a git repo.");
            String::from("Error: Not a git repository.")
        }
    }
}