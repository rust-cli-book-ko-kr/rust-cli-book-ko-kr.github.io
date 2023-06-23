# 출력

## "Hello World" 출력하기

```rust
println!("Hello World");
```

쉽습니다. 좋아요, 다음 주제로 넘어가죠.

## `println!` 사용하기

여러분이 출력하고 싶은 모든 것을 `println!` 매크로를
통해 출력할 수 있습니다.
이 매크로는 놀라운 기능을 갖추고 있으며,
특수한 문법도 있습니다.
`println!` 매크로에는 첫 파라미터로 문자열 리터럴을 전달해야 합니다.
이 파라미터는 추가 인자의 값으로 채워질
플레이스홀더(placeholder)를 포함해야 합니다.

예를 들어:

```rust
let x = 42;
println!("My lucky number is {}.", x);
```

위 코드는 아래와 같은 출력을 냅니다:

```console
My lucky number is 42.
```

위 코드에서 문자열에 있는 중괄호(`{}`)는 플레이스홀더 중 하나로,
주어진 값을 사람이 읽을 수 있는 형태로 출력하는 기본 플레이스홀더입니다.
숫자와 문자열에 대해 아주 잘 동작하지만, 모든 타입에 대해 동작하지는 않습니다.
이는 "디버그 표현"이 있는 이유이기도 한데, `{:?}`처럼 플레이스홀더의 괄호를
채워서 사용할 수 있습니다.

예를 들어:

```rust
let xs = vec![1, 2, 3];
println!("The list is: {:?}", xs);
```

위 코드는 아래와 같은 출력을 냅니다:

```console
The list is: [1, 2, 3]
```

만약 자기만의 데이터 타입을 디버깅과 로깅을 위해 출력 가능하게 만들고 싶다면,
대부분의 경우 타입 정의 위에 `#[derive(Debug)]`를 추가하면 됩니다.

<aside>

**참고:**
"사용자 친화적"인 출력을 위해서는 [`Display`] 트레잇을 사용하면 되고,
디버그 출력(사람이 읽을 수 있지만 개발자를 위한 출력)을 위해서는 [`Debug`] 트레잇을 사용하면 됩니다.
`println!`에서 사용할 수 있는 문법에 대해 더 자세히 알아보고 싶다면
[`std::fmt` 모듈에 대한 문서][std::fmt]를 읽어보세요.

[`Display`]: https://doc.rust-lang.org/1.39.0/std/fmt/trait.Display.html
[`Debug`]: https://doc.rust-lang.org/1.39.0/std/fmt/trait.Debug.html
[std::fmt]: https://doc.rust-lang.org/1.39.0/std/fmt/index.html

</aside>

## 에러 출력하기

에러는 `stderr`를 통해 출력해야 합니다.
그래야 사용자나 다른 프로그램이 파이프를 통해
프로그램의 출력을 파일이나 다른 프로그램에 전달하기
쉬워집니다.

<aside>

**참고:**
대부분의 운영체제에서,
프로그램은 `stdout`과 `stderr` 두 개의 출력 스트림을 가지고 있습니다.
`stdout`은 프로그램의 실제 출력의 위해 사용하는 반면,
`stderr`은 `stdout`으로부터 분리된 별도의 에러나 메시지를 위해 사용합니다.
이를 통해 사용자에게 에러를 보여주는 동시에 출력은 파일에 저장되도록 할 수 있고,
또는 출력이 다른 프로그램에 파이프되도록 할 수도 있습니다.

</aside>

러스트에서는 이를 `println!`과 `eprintln!`으로 구현할 수 있으며,
전자는 `stdout`으로 출력하고,
후자는 `stderr`로 출력합니다.

```rust
println!("This is information");
eprintln!("This is an error! :(");
```

<aside>

**주의**:
[이스케이프 코드]를 출력하는 것은 위험할 수 있으며,
사용자의 터미널을 이상한 상태에 빠지게 만들 수 있습니다.
직접 이스케이프 코드를 출력할 때는 항상 조심하세요!

[이스케이프 코드]: https://ko.wikipedia.org/wiki/ANSI_%EC%9D%B4%EC%8A%A4%EC%BC%80%EC%9D%B4%ED%94%84_%EC%BD%94%EB%93%9C

여러분(그리고 여러분의 사용자들)의 더 나은 삶을 위해,
이스케이프 코드를 다룰 때는 `ansi_term`과 같은 크레이트를
사용하는 것이 가장 좋습니다.

</aside>

## 출력 성능에 대한 참고사항

터미널에 뭔가를 출력하는 것은 끔찍하게 느립니다!
만약 루프에서 `println!`을 사용한다면,
빠른 프로그램에서도 쉽게 보틀넥이 될 것입니다.
성능을 높이기 위한 두 가지 대응 방법이 있습니다.

첫 번째는, 터미널을 실제로 "플러시(flush)"하는
쓰기 횟수를 줄이는 방법입니다.
`println!`은 보통 새로운 라인에 내용을 출력하기 위해
시스템에게 매번 터미널을 플러시해달라고 요청합니다.
그럴 필요가 없다면, `stdout`을 [`BufWriter`]에서
다루도록 래핑하면 됩니다. [`BufWriter`]는 기본적으로
최대 8kB까지 버퍼링 할 수 있습니다.
(즉시 출력을 하고 싶을 때는 `BufWriter`의
`.flush()`를 호출하면 됩니다.)

```rust
use std::io::{self, Write};

let stdout = io::stdout(); // 글로벌 stdout 엔티티를 얻는다
let mut handle = io::BufWriter::new(stdout); // 선택사항: 버퍼로 다루도록 감싼다
writeln!(handle, "foo: {}", 42); // 에러가 신경쓰인다면 여기에 `?`를 추가한다
```

두 번째는,
`stdout` (또는 `stderr`)에 대한 락(lock)을 획득하고
`writeln!`을 이용해 직접 출력하는 방법입니다.
이렇게 하면 시스템이 `stdout`을 매번 다시 잠그고 해제하는 것을 방지할 수 있습니다.

```rust
use std::io::{self, Write};

let stdout = io::stdout(); // 글로벌 stdout 엔티티를 얻는다
let mut handle = stdout.lock(); // 락을 얻는다
writeln!(handle, "foo: {}", 42);  // 에러가 신경쓰인다면 여기에 `?`를 추가한다
```

두 방법을 함께 사용할 수도 있습니다.

[`BufWriter`]: https://doc.rust-lang.org/1.39.0/std/io/struct.BufWriter.html

## 프로그래스 바 보여주기

어떤 CLI 애플리케이션은 1초 이내에 실행되기도 하지만,
어떤 애플리케이션은 수 분, 수 시간을 소요하기도 합니다.
시간이 오래 걸리는 프로그램을 작성한다면,
사용자에게 프로그램이 동작하고 있다는 것을 보여주고 싶을 수 있습니다.
이를 위해서는 상태가 업데이트되고 있다는 정보를 사용하기
쉬운 형태로 출력해줘야 합니다.

[indicatif] 크레이트를 사용하면
프로그램에 프로그래스 바와
작은 스피너를 추가할 수 있습니다.
여기 간단한 예시가 있습니다:

```rust,ignore
{{#include output-progressbar.rs:1:9}}
```

더 자세한 정보는
[문서][indicatif docs]와 [예시][indicatif examples]를
참고하세요.


[indicatif]: https://crates.io/crates/indicatif
[indicatif docs]: https://docs.rs/indicatif
[indicatif examples]: https://github.com/console-rs/indicatif/tree/main/examples

## 로그

프로그램에서 무슨 일이 일어나는지 보다 쉽게 이해하기 위해
로그 구문을 추가하고 싶을 수 있습니다.
보통 애플리케이션을 작성할 때 쉽게 로그를 남길 수 있습니다.
로그는 반년 뒤에 프로그램을 다시 실행할 때 대단히 유용해집니다.
한편, 로그를 남기는 것은 메시지의 중요도를 명시하는 것만 빼면
`println!`을 사용하는 것과 같습니다.
주로 사용하는 로그 레벨에는 _error_, _warn_, _info_, _debug_, _trace_ 가 있습니다. (_error_ 는 중요도가 가장 높고, _trace_ 는 가장 낮습니다.)

애플리케이션에 간단한 로그를 남기기 위해서는
[log] 크레이트 (로그 레벨의 이름을 딴 매크로 포함)와 로그 출력을
작성할 때 유용한 어댑터가 필요합니다.
로그 어댑터는 매우 유연하게 상용할 수 있습니다.
예를 들어, 어댑터를 이용해 터미널이 아닌 [syslog]에 로그를 남길 수도 있고,
아니면 중앙 로그 서버에 로그를 남길 수도 있습니다.

[syslog]: https://ko.wikipedia.org/wiki/%EC%8B%9C%EC%8A%A4%EB%A1%9C%EA%B7%B8

우리는 CLI 애플리케이션을 작성하는 데만 집중하고 있으므로,
당장 사용하기 쉬운 어댑터는 [env_logger]입니다.
[env_logger]를 사용하면 애플리케이션의 어느 부분에 어떤 레벨의
로그를 남길지 환경 변수를 통해 명시할 수 있기 때문에 이를 "env" 로거라고 합니다.
[env_logger]는 로그 메시지 앞에 타임스탬프와
로그를 남긴 모듈의 이름을 붙입니다.
라이브러리도 `log`를 사용할 수 있기 때문에
로그 출력을 쉽게 구성할 수 있습니다.

[log]: https://crates.io/crates/log
[env_logger]: https://crates.io/crates/env_logger

여기 간단한 예시가 있습니다:

```rust,ignore
{{#include output-log.rs}}
```

리눅스나 macOS에서 위 코드를 `src/bin/output-log.rs` 파일로 작성했다면,
아래와 같이 실행할 수 있습니다:
```console
$ env RUST_LOG=info cargo run --bin output-log
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/output-log`
[2018-11-30T20:25:52Z INFO  output_log] starting up
[2018-11-30T20:25:52Z WARN  output_log] oops, nothing implemented!
```

윈도우즈 파워셸에서는 아래와 같이 실행할 수 있습니다:
```console
$ $env:RUST_LOG="info"
$ cargo run --bin output-log
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/output-log.exe`
[2018-11-30T20:25:52Z INFO  output_log] starting up
[2018-11-30T20:25:52Z WARN  output_log] oops, nothing implemented!
```

윈도우즈 CMD에서는 아래와 같이 실행합니다:
```console
$ set RUST_LOG=info
$ cargo run --bin output-log
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/output-log.exe`
[2018-11-30T20:25:52Z INFO  output_log] starting up
[2018-11-30T20:25:52Z WARN  output_log] oops, nothing implemented!
```

`RUST_LOG`는 로그 설정에
사용하는 환경 변수의 이름입니다.
`env_logger`에는 빌더가 있기 때문에
프로그래밍적으로 로그를 설정할 수도 있으며,
가령 기본적으로 _info_ 레벨 메시지가 출력됩니다.

이외에도 많은 로그 어댑터가 있으며,
`log`를 대체하거나 확장할 수 있는 어댑터들이 있습니다.
만약 애플리케이션에 많은 양의 로그가 필요할 것 같다면
다른 것들을 검토해보고 사용자의 삶의 질을 높여주세요.

<aside>

**팁:**
경험적으로, 가볍게 쓸만한 수준의 CLI 프로그램도 향후 수년간 사용되곤 합니다.
(특히 임시 방편으로 프로그램을 만든 경우.)
만약 애플리케이션이 동작하지 않아서 다른 사용자(가령 미래의 여러분)가 그 원인을 찾아야 할 때,
`--verbose`를 전달해 추가적인 로그 출력을 확인할 수 있다면 디버깅에 수 분내지는 수 시간을 절약할 수 있습니다.
[clap-verbosity-flag] 크레이트는 `clap`을 사용해 프로젝트에 쉽게 `--verbose`를 추가할 수 있도록 해줍니다.

[clap-verbosity-flag]: https://crates.io/crates/clap-verbosity-flag

</aside>
