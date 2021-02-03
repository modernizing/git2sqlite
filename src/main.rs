#[macro_use]
extern crate lazy_static;

use std::path::Path;

use crate::git_command::get_commit_message;
use crate::git_log_parser::GitMessageParser;
use std::env;

pub mod git_command;
pub mod git_log_parser;
pub mod coco_commit;

pub fn analysis(local_path: &Path) {
    let messages = get_commit_message(Some(format!("{}", local_path.display())));
    GitMessageParser::parse(messages.as_str());
}

fn main(){
    let args: Vec<String> = env::args().collect();
    process(args[0].as_str());
}

fn process(local: &str) {
    analysis(Path::new(local));
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
        process("/Users/fdhuang/clone/mir");
    }
}