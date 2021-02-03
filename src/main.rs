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
    // let path = Path::new(".");
    // let results = analysis(path);
    // println!("{:?}", results);

    let conn = Connection::open("coco_git.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS git_commit (
                  id              INTEGER PRIMARY KEY,
                  branch          TEXT,
                  commit_id       TEXT,
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

    // let me = Person {
    //     id: 0,
    //     name: "Steven".to_string(),
    //     data: None,
    // };
    // conn.execute(
    //     "INSERT INTO person (name, data) VALUES (?1, ?2)",
    //     params![me.name, me.data],
    // )?;
    //
    // let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    // let person_iter = stmt.query_map(params![], |row| {
    //     Ok(Person {
    //         id: row.get(0)?,
    //         name: row.get(1)?,
    //         data: row.get(2)?,
    //     })
    // })?;
    //
    // for person in person_iter {
    //     println!("Found person {:?}", person.unwrap());
    // }
    Ok(())
}
