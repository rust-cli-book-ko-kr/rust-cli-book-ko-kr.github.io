use clap::Parser;
use std::path::PathBuf;

/// 파일의 라인수를 센다
#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    /// 읽을 파일의 경로
    file: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let mut word_count = 0;
    let file = args.file;

    for line in std::fs::read_to_string(&file).unwrap().lines() {
        word_count += line.split(' ').count();
    }

    println!("Words in {}: {}", file.to_str().unwrap(), word_count)
}
