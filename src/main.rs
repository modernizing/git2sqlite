#[macro_use]
extern crate lazy_static;

use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use rusqlite::{Connection, params, Result};

use crate::coco_commit::CocoCommit;
use crate::git_command::get_commit_message;
use crate::git_log_parser::GitMessageParser;

pub mod git_command;
pub mod git_log_parser;
pub mod coco_commit;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

pub fn analysis(local_path: &Path) {
    let messages = get_commit_message(Some(format!("{}", local_path.display())));
    GitMessageParser::parse(messages.as_str());
}

fn main(){
    process("/Users/fdhuang/consultant/devops/cocoj");
}

fn process(local: &str) {
    // let _ = fs::remove_file(Path::new("commits.json"));
    // analysis(Path::new(local));

    let _ = fs::remove_file(Path::new("coco_git.db"));

    let commits = load_commit();
    let _ = save_to_database(commits);
}

fn load_commit() -> Vec<CocoCommit> {
    let file = File::open("commits.json").unwrap();
    let reader = BufReader::new(file);
    let mut commits = vec![];

    for line in reader.lines() {
        let line_str = line.unwrap();
        let mut string = line_str.as_str();
        if string.starts_with(",") {
            string = string.strip_prefix(",").unwrap()
        }

        if string.starts_with("{") {
            let commit: CocoCommit = serde_json::from_str(string).unwrap();
            commits.push(commit)
        }
    }

    commits
}

fn save_to_database(commits: Vec<CocoCommit>)  {
    let conn = Connection::open("coco_git.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS git_commit (
                  commit_id       TEXT PRIMARY KEY,
                  branch          TEXT,
                  author          TEXT,
                  committer       TEXT,
                  date            INT,
                  message         TEXT,
                  parent_hashes   TEXT,
                  tree_hash       TEXT
                  )",
        params![],
    ).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS file_changed (
                  id              INTEGER PRIMARY KEY,
                  commit_id       TEXT,
                  added           INTEGER,
                  deleted         INTEGER,
                  file            TEXT,
                  mode            TEXT
                  )",
        params![],
    ).unwrap();

    println!("commits: {}", commits.len());
    for commit in commits {
        let parent_hashes = commit.parent_hashes.join(" ");
        conn.execute(
            "INSERT INTO git_commit (commit_id, branch, author, date, message, parent_hashes, tree_hash) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![commit.commit_id, commit.branch, commit.author, commit.date, commit.message, parent_hashes, commit.tree_hash],
        ).unwrap();

        for change in commit.changes {
            conn.execute(
                "INSERT INTO file_changed (commit_id, added, deleted, file, mode) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![commit.commit_id, change.added, change.deleted, change.file, change.mode],
            ).unwrap();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::process;

    #[ignore]
    #[test]
    pub fn should_process_local_code() {
        process(".");
    }

    #[test]
    #[ignore]
    pub fn should_parse_coco() {
        process("/Users/fdhuang/consultant/devops/cocoj");
    }
}