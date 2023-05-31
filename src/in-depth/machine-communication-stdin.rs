use clap::{CommandFactory, Parser};
use is_terminal::IsTerminal as _;
use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
    path::PathBuf,
};

/// 파일 또는 stdin의 라인 수를 센다
#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    /// 읽을 파일의 경로, - 를 사용하면 stdin에서 읽음 (tty는 안 됨)
    file: PathBuf,
}

fn main() {
    let args = Cli::parse();

    let word_count;
    let mut file = args.file;

    if file == PathBuf::from("-") {
        if stdin().is_terminal() {
            Cli::command().print_help().unwrap();
            ::std::process::exit(2);
        }

        file = PathBuf::from("<stdin>");
        word_count = words_in_buf_reader(BufReader::new(stdin().lock()));
    } else {
        word_count = words_in_buf_reader(BufReader::new(File::open(&file).unwrap()));
    }

    println!("Words from {}: {}", file.to_string_lossy(), word_count)
}

fn words_in_buf_reader<R: BufRead>(buf_reader: R) -> usize {
    let mut count = 0;
    for line in buf_reader.lines() {
        count += line.unwrap().split(' ').count()
    }
    count
}
