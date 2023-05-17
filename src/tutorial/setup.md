# 프로젝트 준비

아직 컴퓨터에 [러스트를 설치]하지 않았다면 설치하세요. (몇 분 정도만 걸립니다)
이어서, 터미널을 열고 애플리케이션 코드를 위치할 디렉토리로 이동하세요.

[러스트를 설치]: https://www.rust-lang.org/tools/install

프로그래밍 프로젝트를 보관할 디렉토리에서
`cargo new grrs`를 실행해 프로젝트를 시작하세요.
`grrs` 디렉토리가 새롭게 만들어졌다면,
일반적인 러스트 프로젝트 파일을 확인할 수 있을 것입니다:

- `Cargo.toml`: 프로젝트의 메타데이터와 프로젝트에서 사용할 디펜던시/외부 라이브러리 목록을 담고있는 파일.
- `src/main.rs`: 프로그램 바이너리(main)의 엔트리포인트 파일.

`grrs` 디렉토리에서 `cargo run`을 실행했을 때
"Hello World"가 출력된다면 모든 준비를 마친 것입니다.

## 이렇게 보여야 합니다

```console
$ cargo new grrs
     Created binary (application) `grrs` package
$ cd grrs/
$ cargo run
   Compiling grrs v0.1.0 (/Users/pascal/code/grrs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.70s
     Running `target/debug/grrs`
Hello, world!
```
