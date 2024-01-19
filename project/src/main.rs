use clap::Parser;
use std::process;
use std::{env::current_dir, io::stdout};

#[derive(Parser, Debug)]
struct Arguments {
    #[clap(subcommand)]
    command: Command,
}
use lib::{
    commit::Commit,
    directory::Directory,
    dot_log::{self, JSON},
};

#[derive(Parser, Debug)]
enum Command {
    #[clap(about = "Initialize a new repo")]
    Init,
    #[clap(about = "Highlight differences between current branch and selected one")]
    Diff { branch: String },
    #[clap(about = "Provide information about current state(current branch, modified files)")]
    Status,
    #[clap(about = "Branch to checkout")]
    Checkout { branch: String },
    #[clap(about = "Merge current branch and selected one")]
    Merge { branch: String },
    #[clap(about = "Commit repository changes with a message")]
    Commit { message: String },
}

fn main() {
    let args = Arguments::parse();

    match args.command {
        Command::Init => match dot_log::DotLog::init(current_dir().unwrap().join(".log")) {
            Ok(_) => {}
            Err(err) => {
                println!("{:?}", err);
            }
        },
        Command::Diff { branch } => {
            let current_branch: String;
            let current_directory = current_dir().expect("Error at getting current path");
            let dot_log = match dot_log::DotLog::is_log_repo(current_directory.join(".log")) {
                Some(repo) => {
                    current_branch = dot_log::DotLog::get_branch(&repo).expect("error at getting current branch!");
                    println!("Current branch: {}", current_branch);
                    repo
                }
                None => {
                    println!("Not a log repo!");
                    process::exit(0);
                }
            };
            let mut objects = dot_log.get_objects().expect("Error at getting objects!");
            let selected_branch_commit_hash = dot_log
                .get_branch_commit_hash(&branch)
                .expect("Error at getting last commit hash from current branch!");
            let ignores = dot_log
                .ignores()
                .expect("Error at getting files to be ignored!");
            let current_branch_tree =
                Directory::new(current_directory.as_path(), &ignores, &mut objects).expect("error at getting current branch tree");
            let selected_branch_commit_entry: Commit = objects
                .read_json(selected_branch_commit_hash)
                .expect("Error at getting commit data");
            let selected_branch_commit_tree: Directory = objects
                .read_json(selected_branch_commit_entry.directory)
                .expect("error at getting selected branch commit tree");
            serde_json::to_writer_pretty(
                stdout(),
                &current_branch_tree.diff(&selected_branch_commit_tree),
            )
            .expect("error at displaying json data");
        }
        Command::Status => {
            let current_branch: String;
            let current_directory = current_dir().expect("Error at getting current path");
            let dot_log = match dot_log::DotLog::is_log_repo(current_directory.join(".log")) {
                Some(repo) => {
                    current_branch = dot_log::DotLog::get_branch(&repo).unwrap();
                    println!("Current branch: {}", current_branch);
                    repo
                }
                None => {
                    println!("Not a log repo!");
                    process::exit(0);
                }
            };
            let mut objects = dot_log.get_objects().expect("Error at getting objects!");
            let commit_hash = dot_log
                .get_branch_commit_hash(&current_branch)
                .expect("Error at getting last commit hash from current branch!");
            let ignores = dot_log
                .ignores()
                .expect("Error at getting files to be ignored!");
            let directory =
                Directory::new(current_directory.as_path(), &ignores, &mut objects).expect("");
            let commit: Commit = objects
                .read_json(commit_hash)
                .expect("Error at getting commit data");
            let commit_directory: Directory = objects.read_json(commit.directory).expect("e");
            serde_json::to_writer_pretty(stdout(), &commit_directory.diff(&directory))
                .expect("error at displaying json data");
        }
        Command::Checkout { branch } => {
            let current_directory = current_dir().expect("Error at getting current path");
            let dot_log = match dot_log::DotLog::is_log_repo(current_directory.join(".log")) {
                Some(repo) => repo,
                None => {
                    println!("Not a log repo!");
                    process::exit(0);
                }
            };

            if dot_log.branch_exists(&branch) {
                let mut objects = dot_log.get_objects().expect("Error at getting objects!");
                let selected_branch_commit_hash = dot_log
                    .get_branch_commit_hash(&branch)
                    .expect("Error at getting last commit hash from current branch!");
                let ignores = dot_log
                    .ignores()
                    .expect("Error at getting files to be ignored!");
                let current_branch_tree =
                    Directory::new(current_directory.as_path(), &ignores, &mut objects).expect("error at getting current branch tree");
                let selected_branch_commit_entry: Commit = objects
                    .read_json(selected_branch_commit_hash)
                    .expect("Error at getting commit data");
                let selected_branch_commit_tree: Directory = objects
                    .read_json(selected_branch_commit_entry.directory)
                    .expect("error at getting selected branch commit tree");
                // serde_json::to_writer_pretty(stdout(), &current_branch_tree.diff(&selected_branch_commit_tree)).expect("msg");
                current_branch_tree.build_branch_working_dir(
                    &selected_branch_commit_tree,
                    current_directory.join(".log"),
                );
                match dot_log.set_branch(&branch) {
                    Ok(_) => {
                        println!("Switched to branch: {}", branch);
                    }

                    Err(err) => {
                        println!(
                            "error at switching to branch {}\npossible reason: {:?}",
                            branch, err
                        )
                    }
                }
            } else {
                match dot_log.create_branch(&branch) {
                    Ok(_) => {
                        println!("Created branch: {}", branch);
                        let mut objects = dot_log.get_objects().expect("Error at getting objects!");
                        let selected_branch_commit_hash = dot_log
                            .get_branch_commit_hash(&branch)
                            .expect("Error at getting last commit hash from current branch!");
                        let ignores = dot_log
                            .ignores()
                            .expect("Error at getting files to be ignored!");
                        let current_branch_tree =
                            Directory::new(current_directory.as_path(), &ignores, &mut objects)
                                .expect("error at getting current branch tree");
                        let selected_branch_commit_entry: Commit = objects
                            .read_json(selected_branch_commit_hash)
                            .expect("Error at getting commit data");
                        let selected_branch_commit_tree: Directory = objects
                            .read_json(selected_branch_commit_entry.directory)
                            .expect("error at getting selected branch commit tree");
                        current_branch_tree.build_branch_working_dir(
                            &selected_branch_commit_tree,
                            current_directory.join(".log"),
                        );
                        match dot_log.set_branch(&branch) {
                            Ok(_) => {
                                println!("Switched to branch: {}", branch);
                            }
                            Err(err) => {
                                println!(
                                    "error at switching to branch {}\npossible reason: {:?}",
                                    branch, err
                                )
                            }
                        }
                    }
                    Err(err) => {
                        println!(
                            "error at creating branch {}\npossible reason: {:?}",
                            branch, err
                        )
                    }
                }
            }
        }
        Command::Merge { branch } => {
            let current_branch: String;
            let current_directory = current_dir().expect("Error at getting current path");
            let dot_log = match dot_log::DotLog::is_log_repo(current_directory.join(".log")) {
                Some(repo) => {
                    current_branch = dot_log::DotLog::get_branch(&repo).unwrap();
                    println!("Current branch: {}", current_branch);
                    repo
                }
                None => {
                    println!("Not a log repo!");
                    process::exit(0);
                }
            };
            let mut objects = dot_log.get_objects().expect("Error at getting objects!");
            let selected_branch_commit_hash = dot_log
                .get_branch_commit_hash(&branch)
                .expect("Error at getting last commit hash from current branch!");
            let ignores = dot_log
                .ignores()
                .expect("Error at getting files to be ignored!");
            let current_branch_tree =
                Directory::new(current_directory.as_path(), &ignores, &mut objects).expect("");
            let selected_branch_commit_entry: Commit = objects
                .read_json(selected_branch_commit_hash)
                .expect("Error at getting commit data");
            let selected_branch_commit_tree: Directory = objects
                .read_json(selected_branch_commit_entry.directory)
                .expect("error at getting selected branch commit tree");
            current_branch_tree
                .merge_branches(&selected_branch_commit_tree, current_directory.join(".log"));
        }
        Command::Commit { message } => {
            let current_branch: String;
            let current_directory = current_dir().expect("Error at getting current path");
            let dot_log = match dot_log::DotLog::is_log_repo(current_directory.join(".log")) {
                Some(repo) => {
                    current_branch = dot_log::DotLog::get_branch(&repo).unwrap();
                    println!("Current branch: {}", current_branch);
                    repo
                }
                None => {
                    println!("Not a log repo!");
                    process::exit(0);
                }
            };
            let mut objects = dot_log.get_objects().expect("Error at getting objects!");
            let last_commit_hash = dot_log
                .get_branch_commit_hash(&current_branch)
                .expect("Error at getting last commit hash from current branch!");
            let ignores = dot_log
                .ignores()
                .expect("Error at getting files to be ignored!");
            let directory =
                Directory::new(current_directory.as_path(), &ignores, &mut objects).expect("");
            let new_commit_blob = objects.insert_json(&directory).expect("");
            let commit = Commit {
                directory: new_commit_blob,
                message,
                previous: vec![last_commit_hash].into_iter().collect(),
            };
            let new_commit_hash = objects.insert_json(&commit).expect("");
            dot_log
                .set_branch_commit_hash(&current_branch, new_commit_hash)
                .expect("error at setting hash for the branch to point");
        }
    }
}
