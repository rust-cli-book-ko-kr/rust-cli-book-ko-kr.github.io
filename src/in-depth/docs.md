# CLI 앱을 위한 문서 렌더링하기

CLI를 위한 문서는 보통
명령의 `--help` 섹션이나
매뉴얼(`man`) 페이지로 구성됩니다.

[`clap`](https://crates.io/crates/clap)을 사용하면
[`clap_mangen`](https://crates.io/crates/clap_mangen) 크레이트를 통해
둘 다 자동으로 생성할 수 있습니다.

```rust,ignore
#[derive(Parser)]
pub struct Head {
    /// 로드할 파일
    pub file: PathBuf,
    /// 출력할 라인 개수
    #[arg(short = "n", default_value = "5")]
    pub count: usize,
}
```

두 번째로, 컴파일 타임에 코드에 있는
앱의 정의로부터 매뉴얼 파일을 생성하려면 `
`build.rs`를 사용해야 합니다.

바이너리 패키징 방식 등 고려해야 할 사항이 있지만,
지금은 `src` 폴더 옆에 `man` 을 두도록 하겠습니다.

```rust,ignore
use clap::CommandFactory;

#[path="src/cli.rs"]
mod cli;

fn main() -> std::io::Result<()> {
    let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").ok_or_else(|| std::io::ErrorKind::NotFound)?);
    let cmd = cli::Head::command();

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write(out_dir.join("head.1"), buffer)?;

    Ok(())
}
```

이제 애플리케이션을 컴파일하면 프로젝트 디렉토리에
`head.1` 파일이 만들어집니다.

`man`에서 해당 파일을 열면 공짜 문서에 감탄할 것입니다.
