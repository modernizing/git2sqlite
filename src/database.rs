use rusqlite::{Connection, params};
use crate::coco_commit::CocoCommit;

pub struct Database {
    pub connection: Connection
}

impl Database {
    pub fn new(filename: &str) -> Database {
        let connection = Connection::open(filename).unwrap();
        Database {
            connection
        }
    }

    pub fn create_commit_table(&self) {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS git_commit (
                  commit_id       TEXT PRIMARY KEY,
                  branch          TEXT,
                  author          TEXT,
                  committer       TEXT,
                  date            INT,
                  message         TEXT,
                  parent_hashes   TEXT,
                  tree_hash       TEXT,
                  changes         TEXT,
                  added           INT,
                  deleted         INT,
                  files           TEXT
                  )",
            params![],
        ).unwrap();
    }

    pub fn create_file_change(&self) {
        self.connection.execute(
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
    }

    pub fn insert_commit_with_changes(&self, commit: &CocoCommit) {
        let changes = serde_json::to_string_pretty(&commit.changes).unwrap();
        let parent_hashes = commit.parent_hashes.join(" ");

        self.connection.execute(
            "INSERT INTO git_commit (commit_id, branch, author, date, message, parent_hashes, tree_hash, changes) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![commit.commit_id, commit.branch, commit.author, commit.date, commit.message, parent_hashes, commit.tree_hash, changes],
        ).unwrap();
    }

    pub fn insert_commit(&self, commit: &CocoCommit) {
        let parent_hashes = commit.parent_hashes.join(" ");

        self.connection.execute(
            "INSERT INTO git_commit (commit_id, branch, author, date, message, parent_hashes, tree_hash, added, deleted, files) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![commit.commit_id, commit.branch, commit.author, commit.date, commit.message, parent_hashes, commit.tree_hash, commit.added,commit.deleted, commit.files.join(",")],
        ).unwrap();
    }
}