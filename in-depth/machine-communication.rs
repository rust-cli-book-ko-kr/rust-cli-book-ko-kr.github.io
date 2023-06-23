use clap::Parser;
use serde_json::json;

/// 파일에서 패턴을 찾고 해당하는 라인을 보여준다.
#[derive(Parser)]
struct Cli {
    /// 사람이 읽을 수 있는 메시지 대신 JSON 출력
    #[arg(long = "json")]
    json: bool,
}

fn main() {
    let args = Cli::parse();
    if args.json {
        println!(
            "{}",
            json!({
                "type": "message",
                "content": "Hello world",
            })
        );
    } else {
        println!("Hello world");
    }
}
