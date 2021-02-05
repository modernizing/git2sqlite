#[macro_use]
extern crate lazy_static;

use std::env;
use std::path::Path;
use std::time::Instant;

use cli_option::ConvertOptions;

use crate::git_command::get_commit_message;
use crate::git_log_parser::GitMessageParser;

pub mod git_command;
pub mod git_log_parser;
pub mod coco_commit;
pub mod cli_option;

pub fn analysis(local_path: &Path, options: ConvertOptions) {
    let messages = get_commit_message(Some(format!("{}", local_path.display())));
    GitMessageParser::parse(messages.as_str(), options);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut path = ".";
    if args.len() > 1 {
        path = args[1].as_str();
    }

    let mut options = ConvertOptions::default();
    if args.len() > 2 {
        if args[2].as_str() == "--with-changes" {
            options.with_changes = true;
        }
    }

    let expand_path = shellexpand::tilde(path);

    let start = Instant::now();
    println!("start process: {}, options: {:?}", expand_path, options);
    process(&*expand_path, options);

    println!("finish process in {:?}ms", start.elapsed().as_millis());
}

fn process(local: &str, options: ConvertOptions) {
    analysis(Path::new(local), options);
}

#[cfg(test)]
mod test {
    use crate::process;
    use crate::cli_option::ConvertOptions;

    #[ignore]
    #[test]
    pub fn should_process_local_code() {
        process(".", ConvertOptions::default());
    }

    #[test]
    #[ignore]
    pub fn should_parse_coco() {
        process("/Users/fdhuang/clone/mir", ConvertOptions::default());
    }
}