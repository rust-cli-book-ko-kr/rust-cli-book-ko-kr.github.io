# 종료 코드

프로그램이 항상 성공적으로 동작하지는 않습니다.
에러가 발생했을 때 여러분은 필수적인 정보를 올바르게 내보내야 합니다.
[사용자에게 에러에 대해 말해주기](human-communication.html)에 더해서,
대부분의 시스템에서 프로세스가 종료될 때
종료 코드를 내보냅니다.
(0에서 255까지의 정수가 대부분의 플랫폼에서 호환됩니다.)
여러분은 프로그램의 상태에 알맞은 코드를 내보내야 합니다.
예를 들어서, 프로그램이 성공적으로 동작하는
이상적인 상황에서 종료 코드는 `0`이 되어야 합니다.

에러가 발생하면 조금 더 복잡해집니다.
현실에서는 프로그램에 일반적인 문제가 생겼을 때
많은 경우 종료 코드로 `1`을 내보냅니다.
러스트는 프로세스에 패닉이 일어났을 때
`101`을 종료 코드로 사용합니다.
이를 넘어서, 사람들은 자신의 프로그램에서 많은 것을 해왔습니다.

뭘 할 수 있을까요?
BSD 생태계는 종료 코드에 대한 공통의 정의를 모아뒀습니다.
([여기][`sysexits.h`]에서 찾아볼 수 있습니다.)
러스트 라이브러리 [`exitcode`]는 이와 같은 코드를 제공하며,
여러분의 애플리케이션에서 바로 사용할 수 있습니다.
사용 가능한 값을 보려면 API 문서를 참고하세요.

`Cargo.toml`에 `exitcode` 디펜던시를 추가한 뒤에
아래와 같이 사용할 수 있습니다:

```rust,ignore
fn main() {
    // ...실제 작업...
    match result {
        Ok(_) => {
            println!("Done!");
            std::process::exit(exitcode::OK);
        }
        Err(CustomError::CantReadConfig(e)) => {
            eprintln!("Error: {}", e);
            std::process::exit(exitcode::CONFIG);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(exitcode::DATAERR);
        }
    }
}
```


[`exitcode`]: https://crates.io/crates/exitcode
[`sysexits.h`]: https://www.freebsd.org/cgi/man.cgi?query=sysexits&apropos=0&sektion=0&manpath=FreeBSD+11.2-stable&arch=default&format=html
