# 기계와 소통하기

The power of command-line tools really comes to shine
when you are able to combine them.
This is not a new idea:
In fact, this is a sentence from the [Unix philosophy]:
커맨드라인 도구의 진정한 힘은 여러 도구를
결합할 때 드러납니다.
이는 새로운 사실이 아닙니다.
아래는 [유닉스 철학]에 나오는 문장입니다:

> "모든 프로그램 출력이 아직 잘 알려지지 않은 프로그램이라고 할지라도 다른 프로그램에 대한 입력이 될 수 있게 할 것."

[유닉스 철학]: https://ko.wikipedia.org/wiki/%EC%9C%A0%EB%8B%89%EC%8A%A4_%EC%B2%A0%ED%95%99

프로그램이 이 기대를 충족하면
사용자가 행복해집니다.
이러한 철학을 따르기 위해
우리는 사람들을 위한 보기 좋은 출력뿐만 아니라
다른 프로그램이 필요로 하는 것을 제공해야 합니다.
어떻게 하는지 살펴봅시다.

<aside>

**참고:**
먼저 [CLI 출력 챕터][output]을 읽어보세요.
[CLI 출력 챕터]에서는 터미널에 출력을 내는 방법에 대해 설명합니다.

[output]: ../tutorial/output.html

</aside>

## 누가 출력을 읽나요?

첫 번째 질문은: 출력이 컬러풀한 터미널 앞에 있는 사람을 위한 것인지,
또 다른 프로그램을 위한 것인지 묻는 것입니다.
이 질문에 대답하기 위해
[is-terminal]과 같은 크레이트를 사용할 수 있습니다:

[is-terminal]: https://crates.io/crates/is-terminal

```rust,ignore
use is_terminal::IsTerminal as _;

if std::io::stdout().is_terminal() {
    println!("I'm a terminal");
} else {
    println!("I'm not");
}
```

출력을 읽을 대상에 따라 추가적인 정보를
제공할 수 있습니다.
사람들은 보통 색깔이 있는 출력을 좋아합니다.
예를 들어 임의의 러스트 프로젝트에서
`ls`를 실행하면 아래와 같은 결과를 볼 수 있을 것입니다:

```console
$ ls
CODE_OF_CONDUCT.md   LICENSE-APACHE       examples
CONTRIBUTING.md      LICENSE-MIT          proptest-regressions
Cargo.lock           README.md            src
Cargo.toml           convey_derive        target
```

이 스타일은 사람을 위해 만들어졌기 때문에, 대부분의 설정에서
`src`와 같은 일부 이름을 다른 색상으로 보여줌으로써
`src`가 디렉토리임을 표시합니다.
그러나 이를 파일이나 `cat` 같은 프로그램에 파이프하면
`ls`는 그에 맞는 출력을 내보냅니다.
터미널 윈도우에 알맞은 컬럼 레이아웃을 출력하는 대신
개별 행에 파일 이름을 출력합니다.
또한 여기에는 아무런 색깔도 적용되어 있지 않습니다.

```console
$ ls | cat
CODE_OF_CONDUCT.md
CONTRIBUTING.md
Cargo.lock
Cargo.toml
LICENSE-APACHE
LICENSE-MIT
README.md
convey_derive
examples
proptest-regressions
src
target
```

## 기계를 위한 쉬운 출력 형식

역사적으로,
커맨드라인 도구가 생성하는 출력은 대부분 문자열입니다.
터미널 앞에 앉아 있는 사람이 보통 텍스트를 읽고
의미를 추론할 수 있기 때문에 문자열을 출력해도 문제가 없습니다.
하지만 프로그램에게는 그런 능력이 없습니다.
즉, 어떤 프로그램이 `ls`와 같은 도구의 출력을 이해하려면
프로그래머가 `ls`의 출력을 읽는 파서를 작성해야 합니다.

이는 보통 출력이 파싱하기 쉬운 형식으로
제한되어 있음을 의미합니다.
각 레코드가 개별 라인에 들어가고,
개별 라인에는 탭으로 구분된 내용이 들어가는
TSV(tab-separated values, 탭으로 구분된 값)와 같은 형식은
매우 인기있습니다.
이처럼 텍스트 라인을 기반으로 하는 단순한 형식은
`grep`과 같은 도구가 `ls`와 같은 다른 도구의
출력을 사용할 수 있도록 해줍니다.
`| grep Cargo`는 개별 라인이 `ls`에서 왔는지,
파일에서 왔는지 신경쓰지 않으며, 라인별로
필터링을 수행할 것입니다.

단점은 `ls`가 제공한 모든 디렉토리를 필터링하는
간단한 `grep` 호출을 사용할 수 없다는 점입니다.
이를 위해서는 각 디렉토리 요소에 추가적인 데이터를 더해야 합니다.

## 기계를 위한 JSON 출력

TSV는 정형화된 데이터를 출력하는
간단한 방법입니다. 그러나 출력에 TSV를 사용하려면
다른 프로그램이 해당 출력에 어떤 필드가 있는지(그리고 어떤 순서인지)
미리 알고 있어야 하며, 다른 타입의 메시지를 출력하기도 어렵습니다.
예를 들어, 우리의 프로그램이 메시지를 출력해 다운로드를 기다리고 있음을
다른 프로그램에게 알리고, 이후 다운로드한 데이터에 대해 설명하는
메시지를 출력하고자 하는 경우를 생각해 볼 수 있습니다.
이 경우 두 메시지의 성격은 매우 다르며,
TSV 출력으로 이를 통합해 표현하려면
둘을 구분할 방법을 고안해야 합니다.
마찬가지로 길이가 다른 두 리스트를 출력하고자 할 때도
같은 문제가 발생합니다.

그러나 대부분의 프로그래밍 언어/환경에서
쉽게 파싱 가능한 형식을 선택하는 것은 좋은 생각입니다.
그래서 지난 몇 년 동안 많은 애플리케이션이 데이터를
[JSON] 형식으로 출력하는 기능을 갖췄습니다.
JSON은 거의 모든 언어가 파싱할 수 있는 충분히 간단한
형식이면서도 다양한 상황에 유용하게 사용할 수 있습니다.
JSON은 사람이 읽을 수 있는 텍스트 형식이며,
많은 사람들이 JSON 데이터를 빠르게 파싱하고 직렬화하는
구현체를 개발했습니다.

[JSON]: https://www.json.org/

앞서 우리는 프로그램이 출력하는 "메시지"에 대해
이야기했습니다.
이는 프로그램의 출력에 대해 생각해 보는 좋은 방법입니다.
프로그램은 단지 하나의 데이터 덩어리만 출력하지 않고
실행 중에 다양한 종류의 정보를 출력할 수 있습니다.
JSON을 출력할 때 이러한 접근법을 지원할 수 있는
쉬운 방법 중 하나는 메시지 당 하나의 JSON 문서를 작성하고
새로운 라인에 각 JSON 문서를 넣는 것입니다.
(이 방법을 때로 [Line-delimited JSON][jsonlines]라고 부릅니다.)
이를 통해 일반적인 `println!`을 사용하는 것만큼 간단한 구현이 가능합니다.

[jsonlines]: https://en.wikipedia.org/wiki/JSON_streaming#Line-delimited_JSON

아래는 [serde_json]의 `json!` 매크로를 이용해
러스트 소스코드에서 빠르게 JSON을 작성하는 간단한 예시입니다:

[serde_json]: https://crates.io/crates/serde_json

```rust,ignore
{{#include machine-communication.rs}}
```

출력은 아래와 같습니다:

```console
$ cargo run -q
Hello world
$ cargo run -q -- --json
{"content":"Hello world","type":"message"}
```

(`cargo`를 `-q`와 함께 실행하면 출력을 생략할 수 있습니다.
`--` 뒤의 인수는 프로그램으로 전달됩니다.)

### 실습 예시: ripgrep

[ripgrep]은 grep이나 ag를 대체하는 러스트 프로그램입니다.
기본적으로 아래와 같은 출력을 만들어 냅니다:

[ripgrep]: https://github.com/BurntSushi/ripgrep

```console
$ rg default
src/lib.rs
37:    Output::default()

src/components/span.rs
6:    Span::default()
```

그런데 `--json` 옵션을 주면 아래와 같이 출력됩니다:

```console
$ rg default --json
{"type":"begin","data":{"path":{"text":"src/lib.rs"}}}
{"type":"match","data":{"path":{"text":"src/lib.rs"},"lines":{"text":"    Output::default()\n"},"line_number":37,"absolute_offset":761,"submatches":[{"match":{"text":"default"},"start":12,"end":19}]}}
{"type":"end","data":{"path":{"text":"src/lib.rs"},"binary_offset":null,"stats":{"elapsed":{"secs":0,"nanos":137622,"human":"0.000138s"},"searches":1,"searches_with_match":1,"bytes_searched":6064,"bytes_printed":256,"matched_lines":1,"matches":1}}}
{"type":"begin","data":{"path":{"text":"src/components/span.rs"}}}
{"type":"match","data":{"path":{"text":"src/components/span.rs"},"lines":{"text":"    Span::default()\n"},"line_number":6,"absolute_offset":117,"submatches":[{"match":{"text":"default"},"start":10,"end":17}]}}
{"type":"end","data":{"path":{"text":"src/components/span.rs"},"binary_offset":null,"stats":{"elapsed":{"secs":0,"nanos":22025,"human":"0.000022s"},"searches":1,"searches_with_match":1,"bytes_searched":5221,"bytes_printed":277,"matched_lines":1,"matches":1}}}
{"data":{"elapsed_total":{"human":"0.006995s","nanos":6994920,"secs":0},"stats":{"bytes_printed":533,"bytes_searched":11285,"elapsed":{"human":"0.000160s","nanos":159647,"secs":0},"matched_lines":2,"matches":2,"searches":2,"searches_with_match":2}},"type":"summary"}
```

보시다시피 각 JSON 문서는 `type` 필드를 포함하는 객체(맵)입니다.
이를 통해 `rg`를 위한 간단한 프론트엔드를 작성할 수 있습니다.
이 프론트엔드는 문서가 주어질 때마다 내용을 읽고,
일치하는 부분(또는 일치하는 파일)을 표시해줍니다.
이 과정은 심지어 ripgrep이 여전히 검색 중일 때도 가능합니다.

<aside>

**참고:**
Visual Studio Code가 코드를 검색할 때 ripgrep을 이렇게 사용합니다.

</aside>

## 파이프된 입력을 다루는 방법

파일의 단어수를 세는 프로그램이 있다고 생각해봅시다:

``` rust,ignore
{{#include machine-communication-wc.rs}}
```

이 프로그램은 파일의 경로르 받아 라인별로 읽고 공백으로 구분된
단어의 개수를 셉니다.

프로그램을 실행하면 파일에 있는 총 단어수가 출력됩니다:

``` console
$ cargo run README.md
Words in README.md: 47
```

이 프로그램이 파이프로 전달받은 파일의 단어수를 세도록 하려면 어떻게 해야 할까요?
러스트 프로그램은 [Stdin 구조체](https://doc.rust-lang.org/std/io/struct.Stdin.html)를
통해 전달받은 데이터를 읽을 수 있습니다. 이 구조체는
표준 라이브러리의 [stdin 함수](https://doc.rust-lang.org/std/io/fn.stdin.html)를
통해 얻을 수 있습니다.
파일의 라인을 읽는 것처럼 stdin의 라인을 읽을 수 있습니다.

아래는 stdin을 통해 파이프된 데이터의 단어수를 세는 프로그램입니다:

``` rust,ignore
{{#include machine-communication-stdin.rs}}
```

만약 텍스트를 파이프로 전달하여 프로그램을 실행할 때는, `-`가
`stdin`으로부터 데이터를 읽어들인다는 것을 의미합니다.
이 프로그램은 단어수를 출력합니다:

``` console
$ echo "hi there friend" | cargo run -- -
Words from stdin: 3
```

이 프로그램은 런타임에 입력된 텍스트가 아닌, 파이프된 입력을 예상하기 때문에
인터랙티브하지 않은 stdin을 요구합니다. 만약 stdin이 tty라면
프로그램은 작동하지 않는 이유를 알려주기 위해 도움말 문서를 출력합니다.
