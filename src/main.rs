#[macro_use]
extern crate lazy_static;

use std::path::Path;

use crate::coco_commit::CocoCommit;
use crate::git_command::get_commit_message;
use crate::git_log_parser::GitMessageParser;

pub mod git_command;
pub mod git_log_parser;
pub mod coco_commit;

use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

pub fn analysis(local_path: &Path) -> Vec<CocoCommit> {
    let messages = get_commit_message(Some(format!("{}", local_path.display())));
    let vec = GitMessageParser::parse(messages.as_str());

    return vec;
}

fn main() -> Result<()> {
    let path = Path::new(".");
    let commits = analysis(path);

    let conn = Connection::open("coco_git.db")?;
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
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS file_changed (
                  id              INTEGER PRIMARY KEY,
                  commit_id       INTEGER,
                  added           INTEGER,
                  deleted         INTEGER,
                  file            TEXT,
                  mode            TEXT,
                  FOREIGN KEY (commit_id) REFERENCES git_commit
                  )",
        params![],
    )?;

    for commit in commits {
        let parent_hashes = commit.parent_hashes.join(" ");
        conn.execute(
            "INSERT INTO git_commit (commit_id, branch, author, date, message, parent_hashes, tree_hash) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![commit.commit_id, commit.branch, commit.author, commit.date, commit.message, parent_hashes, commit.tree_hash],
        )?;

        for change in commit.changes {
            conn.execute(
                "INSERT INTO file_changed (commit_id, added, deleted, file, mode) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![commit.commit_id, change.added, change.deleted, change.file, change.mode],
            )?;
        }
    }

    Ok(())
}
