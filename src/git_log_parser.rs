use std::collections::HashMap;

use regex::{Captures, Regex};
use crate::coco_commit::{CocoCommit, FileChange};
use std::fs::{OpenOptions, File};
use std::io::prelude::*;

lazy_static! {
    static ref COMMIT_INFO: Regex = Regex::new(
        r"(?x)
\[(?P<commit_id>[\d|a-f]{5,12})\]
\s(?P<author>.*?)<(?P<email>.*?)>
\s(?P<date>\d{10})
\s\((?P<parent_hashes>([\d|a-f]{5,12}|\s)*),(?P<tree_hash>[\d|a-f]{5,12})\) # parents hash + tree hash
\s\#(?P<branch>.*)\#    # branch
\s(?P<message>.*) # commit messages"
    )
    .unwrap();
    static ref CHANGES: Regex =
        Regex::new(r"(?P<deleted>[\d-]+)[\t\s]+(?P<added>[\d-]+)[\t\s]+(?P<filename>.*)").unwrap();
    static ref CHANGEMODEL: Regex =
        Regex::new(r"\s(\w{1,6})\s(mode 100(\d){3})?\s?(.*)(\s\(\d{2}%\))?").unwrap();

    // for rename
    // static ref COMPLEXMOVEREGSTR: Regex = Regex::new(r"(.*)\{(.*)\s=>\s(.*)\}(.*)").unwrap();
    // static ref BASICMOVEREGSTR: Regex = Regex::new(r"(.*)\s=>\s(.*)").unwrap();
}

pub struct GitMessageParser {
    count: i32,
    current_commit: CocoCommit,
    current_file_change: Vec<FileChange>,
    current_file_change_map: HashMap<String, FileChange>,
}

impl Default for GitMessageParser {
    fn default() -> Self {
        GitMessageParser {
            count: 1,
            current_commit: Default::default(),
            current_file_change: vec![],
            current_file_change_map: Default::default(),
        }
    }
}

impl GitMessageParser {
    pub fn parse(str: &str) {
        let split = str.split("\n");
        let mut parser = GitMessageParser::default();

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("commits.json")
            .unwrap();

        let _ = writeln!(file, "[");

        for line in split {
            parser.parse_log_by_line(line, &mut file)
        }

        let _ = writeln!(file, "]");
    }

    pub fn parse_log_by_line(&mut self, text: &str, file: &mut File) {
        // COMMIT_ID -> CHANGES -> CHANGE_MODEL -> Push to Commits
        if let Some(captures) = COMMIT_INFO.captures(text) {
            self.current_commit = GitMessageParser::create_commit(&captures)
        } else if let Some(caps) = CHANGES.captures(text) {
            let filename = caps["filename"].to_string();
            let file_change = GitMessageParser::create_file_change(filename.clone(), caps);
            self.current_file_change_map.insert(filename, file_change);
        } else if let Some(caps) = CHANGEMODEL.captures(text) {
            self.update_change_mode(caps)
        } else if self.current_commit.commit_id != "" {
            self.push_to_commits(file);
        }
    }

    fn push_to_commits(&mut self, file: &mut File) {
        for (_filename, change) in &self.current_file_change_map {
            self.current_file_change.push(change.clone());
        }

        self.current_commit.changes = self.current_file_change.clone();

        self.current_file_change_map.clear();
        if self.count == 1 {
            self.write_to_file_first(self.current_commit.clone(), file);
        } else {
            self.write_to_file(self.current_commit.clone(), file);
        }
        self.count = self.count + 1 ;
    }

    fn write_to_file_first(&self, commit: CocoCommit, file: &mut File) {
        let result = serde_json::to_string(&commit).unwrap();
        if let Err(e) = writeln!(file, "{}", format!("{}", result)) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
    fn write_to_file(&self, commit: CocoCommit, file: &mut File) {
        let result = serde_json::to_string(&commit).unwrap();
        if let Err(e) = writeln!(file, "{}", format!(",{}", result)) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    fn update_change_mode(&mut self, caps: Captures) {
        let change_model_index = 4;
        if caps.len() > change_model_index {
            let mode = caps.get(1).unwrap().as_str();
            let file_name = caps.get(4).unwrap().as_str();
            if self.current_file_change_map.get(file_name).is_some() {
                let change = self.current_file_change_map.get_mut(file_name).unwrap();
                change.mode = mode.to_string();
            } else {
                let change = FileChange {
                    added: 0,
                    deleted: 0,
                    file: file_name.to_string(),
                    mode: mode.to_string(),
                };
                self.current_file_change_map
                    .insert(file_name.to_string(), change);
            }
        }
    }

    fn create_file_change(filename: String, caps: Captures) -> FileChange {
        let mut added = 0;
        let mut deleted = 0;

        if let Ok(value) = caps["added"].parse::<i32>() {
            added = value
        }

        if let Ok(value) = caps["deleted"].parse::<i32>() {
            deleted = value
        }

        FileChange {
            added,
            deleted,
            file: filename,
            mode: "".to_string(),
        }
    }

    fn create_commit(captures: &Captures) -> CocoCommit {
        let commit_id = &captures["commit_id"];
        let author = &captures["author"];
        let date_str = &captures["date"];
        let message = &captures["message"];
        let branch = &captures["branch"];

        let mut parent_hashes = vec![];
        if let Some(_) = captures.name("parent_hashes") {
            let hashes = &captures["parent_hashes"];
            if hashes != "" {
                parent_hashes = hashes.split(" ").map(|str| str.to_string()).collect()
            }
        }

        let tree_hash = captures["tree_hash"].to_string();

        let date = date_str.parse::<i64>().unwrap();
        CocoCommit {
            branch: branch.to_string(),
            commit_id: commit_id.to_string(),
            author: author.to_string(),
            committer: "".to_string(),
            date,
            message: message.to_string(),
            changes: vec![],
            parent_hashes,
            tree_hash,
        }
    }
}
