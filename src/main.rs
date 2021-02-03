#[macro_use]
extern crate lazy_static;

use std::path::Path;

use crate::coco_commit::CocoCommit;
use crate::git_command::get_commit_message;
use crate::git_log_parser::GitMessageParser;

pub mod git_command;
pub mod git_log_parser;
pub mod coco_commit;

pub fn analysis(local_path: &Path) -> Vec<CocoCommit> {
    let messages = get_commit_message(Some(format!("{}", local_path.display())));
    let vec = GitMessageParser::parse(messages.as_str());

    return vec;
}


fn main() {
    let path = Path::new(".");
    let results = analysis(path);
    println!("{:?}", results);
}
