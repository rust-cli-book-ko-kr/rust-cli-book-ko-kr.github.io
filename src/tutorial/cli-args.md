# 커맨드라인 인자 파싱하기

우리가 만드는 CLI 도구의 일반적인 호출은 아래와 같이 보일 것입니다:

```console
$ grrs foobar test.txt
```

이때 프로그램은 `test.txt`를 살펴보고,
`foobar`가 포함된 라인을 출력할 것입니다.
그런데 저 두 값을 어떻게 얻을 수 있을까요?

프로그램 이름 뒤에 나오는 텍스트는 보통
"커맨드라인 인자"라고 부르며,
`--this`와 같이 쓰일 때는 특히 "커맨드라인 플래그"라고도 부릅니다.
내부적으로, 운영체제는 이를 문자열의 리스트로 나타냅니다.
거칠게 말해서 이때 인자들은 공백으로 분리됩니다.

주어진 인자와 그 인자를 어떻게 파싱할지에 대해
다양한 방향으로 생각해 볼 수 있습니다.
또한 여러분은 프로그램 사용자에게
어떤 인자를 어떤 형식으로 전달해야 하는지 알려줘야 합니다.

## 인자 얻기

표준 라이브러리에는 주어진 인자의 [이터레이터]를 제공하는
[`std::env::args()`] 함수가 있습니다.
첫 엔트리(`0`번 인덱스)는 프로그램의 이름입니다. (예를 들어 `grrs`.)
이후 인자는 사용자가 입력하는 값이 됩니다.

[`std::env::args()`]: https://doc.rust-lang.org/1.39.0/std/env/fn.args.html
[이터레이터]: https://doc.rust-lang.org/1.39.0/std/iter/index.html

이 방식으로 인자를 그대로 얻기는 매우 쉽습니다 (`src/main.rs` 파일에서 `fn main() {` 다음 부분):

```rust,ignore
{{#include cli-args-struct.rs:10:11}}
```

## 데이터 타입으로서의 CLI 인자

주어진 인자들을 텍스트의 묶음으로 취급하는 대신,
CLI 인자를 프로그램의 입력을 표현하는
임의의 데이터 타입으로 생각해 볼 수 있습니다.

`grrs foobar test.txt`를 봅시다:
여기에는 두 개의 인자가 있습니다.
첫 번째로 `pattern` (찾을 문자열)이 있고,
이어서 `path` (문자열을 찾을 파일)이 있습니다.

이것에 대해 무엇을 더 논의할 수 있을까요?
프로그램을 시작하기 위해서는 두 인자가 모두 필요합니다.
우리가 기본값을 지정한 적이 없으므로,
사용자는 항상 두 값을 제공할 것이라고 예상할 수 있습니다.
더 나아가, 인자의 타입에 대해 말할 수 있습니다:
첫 번째 인자인 패턴은 문자열이 될 것이고,
두 번째 인자는 파일의 경로가 될 것입니다.

러스트에서는 프로그램이 다루는 데이터를 중심으로 프로그램을 구성하는 것이 일반적이므로,
이러한 방식으로 CLI 인자를 처리하는 것이 매우 적합합니다.
이것부터 시작하겠습니다 (`src/main.rs` 파일에서 `fn main() {` 앞 부분):

```rust,ignore
{{#include cli-args-struct.rs:3:7}}
```

위 코드는 `pattern`과 `path` 두 필드에 데이터를 저장하는
새로운 구조체([`struct`])를 정의힙니다.

[`struct`]: https://doc.rust-lang.org/1.39.0/book/ch05-00-structs.html

<aside>

**참고:**
[`PathBuf`]는 [`String`]과 비슷하지만, 크로스 플랫폼에서 동작하는 파일 시스템 경로를 위해 사용합니다.

[`PathBuf`]: https://doc.rust-lang.org/1.39.0/std/path/struct.PathBuf.html
[`String`]: https://doc.rust-lang.org/1.39.0/std/string/struct.String.html

</aside>

인자를 앞서 정의한 구조체 형태로 만들기 위해서는 프로그램에 입력된 실제 인자를 얻어야 합니다.
한 가지 방법은 운영체제로부터 얻은 문자열 리스트를 하나씩 파싱하고, 구조체를 직접 만드는 것입니다.
이는 아래와 같이 할 수 있습니다:

```rust,ignore
{{#include cli-args-struct.rs:10:15}}
```

위 코드는 잘 동작하기는 하지만, 별로 편리하지 않습니다.
이 방법으로 `--pattern="foo"`나 `--pattern "foo"`와 같은 요구사항은 어떻게 지원할 수 있을까요?
`--help`는 어떻게 구현해야 할까요?

## Clap으로 CLI 인자 파싱하기

더 좋은 방법은 다양한 라이브러리 중 하나를 사용하는 것입니다.
커맨드라인 인자를 파싱하는 데 가장 인기있는 라이브러리는
[`clap`]입니다.
[`clap`]은 서브 커맨드, [shell completions], 도움말 메시지 등,
여러분이 생각하는 모든 기능을 지원합니다.

[`clap`]: https://docs.rs/clap/
[shell completions]: https://docs.rs/clap_complete/

먼저 `Cargo.toml` 파일의 `[dependencies]` 섹션에
`clap = { version = "4.0", features = ["derive"] }`을 추가해
`clap`을 가져와 봅시다.

이제 우리의 코드에 `use clap::Parser;`를 추가하고,
`struct Cli` 바로 위에 `#[derive(Parser)]`를 작성합니다.
그리고 문서화 주석도 작성해봅시다.

아래와 같이 하면 됩니다 (`src/main.rs` 파일의 `fn main() {` 앞 부분):

```rust,ignore
{{#include cli-args-clap.rs:3:12}}
```

<aside class="node">

**참고:**
필드에 추가할 수 잇는 수많은 커스텀 어트리뷰트가 있습니다.
예를 들어, 어떤 필드를 `-o` 또는 `--output` 뒤에 오는 인자를 위해 사용하고 싶다면
`#[arg(short = 'o', long = "output")]`를 추가하면 됩니다.
더 자세한 정보는 [clap documentation][`clap`]을 확인해보세요.

</aside>

`Cli` 구조체 바로 아래에 `main` 함수가 있습니다.
프로그램이 실행되면, 프로그램은 `main` 함수를 호출하게 됩니다.
함수의 첫 줄은 아래와 같습니다:

```rust,ignore
{{#include cli-args-clap.rs:14:16}}
```

위 코드는 인자를 파싱해 `Cli` 구조체로 변환합니다.

이때 문제가 생기면 어떻게 될까요?
이 지점이 Clap을 사용하는 접근법의 아름다운 부분입니다.
Clap은 어떤 필드가 주어져야 하는지,
그 필드가 어떤 형식으로 주어져야 하는지 알고 있습니다.
Clap은 자동으로 `--help` 메시지를 생성해 줄 뿐만 아니라,
`--output`이 아닌 `--putput`을 입력한 사용자에게
에러를 제공해 줍니다.

<aside class="note">

**참고:**
`parse` 메서드는 `main` 함수에서 사용해야 합니다.
만약 파싱에 실패한다면 `parse` 메서드는 에러나 도움말 메시지를 출력하고
즉시 프로그램을 종료할 것입니다.
다른 곳에서는 `parse` 메서드를 사용하지 마세요!

</aside>

## 마무리

여러분의 코드는 이제 아래와 같아야 합니다:

```rust,ignore
{{#include cli-args-clap.rs}}
```

아무 인자없이 실행하는 경우:

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 10.16s
     Running `target/debug/grrs`
error: The following required arguments were not provided:
    <pattern>
    <path>

USAGE:
    grrs <pattern> <path>

For more information try --help
```

`cargo run`을 실행할 때는 `--` 뒤에 인자를 전달할 수 있습니다.

```console
$ cargo run -- some-pattern some-file
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/grrs some-pattern some-file`
```

보다시피 프로그램은 아무것도 출력하지 않습니다.
이는 오류 없이 프로그램이 종료되었다는 것을 의미합니다.

<aside class="exercise">

**연습:**
이 프로그램이 인자를 출력하도록 만들어 보세요!

</aside>
