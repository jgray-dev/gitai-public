// main.rs
//cargo build --release
//sudo mv target/release/gitai /usr/local/bin/gitai



use clap::Parser;
use std::env;
use std::path::PathBuf;

mod ai;
mod config;
mod git;
mod anthropic;

#[derive(Parser)]
#[clap(version = "1.0", author = "LI:jackson--gray")]
struct Opts {
    /// Path to the Git repository
    #[clap(short, long, default_value = ".")]
    repo_path: String,
    // Add more command line options as needed
}

fn main() {
    let _opts: Opts = Opts::parse();

    // Get the current working directory
    let current_dir = env::current_dir().expect("Failed to get current directory");

    // Use the current directory as the repository path
    let repo_path = current_dir.to_str().expect("Failed to convert current directory to string");

    let config = config::load_config();
    let repo_info = git::extract_repo_info(repo_path);
    let commit_message = ai::generate_commit_message(&repo_info, &config);

    // git::perform_commit(repo_path, &commit_message);
    println!("{}", commit_message);
}