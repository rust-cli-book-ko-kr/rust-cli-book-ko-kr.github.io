#![allow(unused)]

use clap::Parser;

/// 파일에서 패턴을 찾고 패턴을 포함한 라인을 보여준다.
#[derive(Parser)]
struct Cli {
    /// 찾을 패턴
    pattern: String,
    /// 읽을 파일 경로
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
}
