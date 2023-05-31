# 사람과 소통하기

먼저 [CLI 출력 챕터][output]를 읽을 것을 권장합니다.
[CLI 출력 챕터][output]에서는 터미널에 출력을 어떻게 내는지 설명했다면,
이 챕터에서는 무엇을 출력할지 설명합니다.

[output]: ../tutorial/output.html

## 모든 것이 순조로울 때

모든 것이 순조로울 때도 사용자에게
애플리케이션의 진행 상황을 보여주는 것이 좋습니다.
이때 메시지는 간결하고 유익해야 합니다.
로그에 지나치게 기술적인 용어를 사용하지 마세요.
그리고, 애플리케이션이 충돌(crash)한 것이 아니므로
사용자가 에러를 찾아볼 필요는 없다는 사실을 기억하세요.

커뮤니케이션 스타일이 일관되어야 한다는 점이
가장 중요합니다.
로그를 쉽게 파악할 수 있도록
항상 같은 접두어와 문장 구조를 사용하세요.

애플리케이션의 출력이 지금 프로그램에 무슨 일이 일어나고 있는지,
이 일이 사용자에게 어떤 영향을 미치는지 이야기하도록 하세요.
이를 위해 단계별 타임라인을 보여줄 수도 있고,
오래 걸리는 작업에서는 프로그래스 바와 인디케이터를 보여줄 수도 있습니다.
사용자로 하여금 애플리케이션이 하는 일을 따라갈 수 있게 만들고,
프로그램이 하는 일이 비밀스럽게 느껴지지 않도록 해야 합니다.

## 무슨 일이 일어나는지 말하기 어려울 때

사소한 상태를 알릴 때는 일관성을 유지하는 것이 중요합니다.
많은 로그를 남기면서도 로그 레벨을 엄격히 따르지 않는
애플리케이션은 로그를 남기지 않는 애플리케이션보다
적은 정보를 제공합니다.

따라서 이벤트와 메시지의 중요도를
정의하여 일관된 로그 레벨을 사용하는 것이 중요합니다.
이러한 방식으로 사용자는 `--verbose` 플래그 또는
환경 변수(`RUST_LOG` 등)를 통해 직접 로그 양을 조절할 수 있습니다.

일반적으로 사용하는 `log` 크레이트는
아래와 같은 로그 레벨을 [정의][log-levels]합니다.
(중요도 오름차순)

- trace
- debug
- info
- warning
- error

_info_ 를 기본 로그 레벨로 설정하여 유용한 출력을
제공하는 것이 좋습니다. (더 조용한 출력 스타일을 지향하는
일부 애플리케이션은 기본적으로 경고와 에러만 보여주기도 합니다.)

추가로, 모든 로그 메시지에서 비슷한 접두어와
문장 구조를 사용하는 것은 좋은 생각입니다.
이렇게 하면 `grep`과 같은 도구를 사용해 로그를 쉽게 필터링할 수 있습니다.
메시지에는 필터링된 로그에서 유용한 정보를 얻을 수 있을 정도로
충분한 맥락을 제공하되, *너무* 상세한 정보를 담지는 않아야 합니다.

[log-levels]: https://docs.rs/log/0.4.4/log/enum.Level.html

### 로그 예시

```console
error: could not find `Cargo.toml` in `/home/you/project/`
```

```console
=> Downloading repository index
=> Downloading packages...
```

아래는 [wasm-pack]의 로그 출력입니다:

```console
 [1/7] Adding WASM target...
 [2/7] Compiling to WASM...
 [3/7] Creating a pkg directory...
 [4/7] Writing a package.json...
 > [WARN]: Field `description` is missing from Cargo.toml. It is not necessary, but recommended
 > [WARN]: Field `repository` is missing from Cargo.toml. It is not necessary, but recommended
 > [WARN]: Field `license` is missing from Cargo.toml. It is not necessary, but recommended
 [5/7] Copying over your README...
 > [WARN]: origin crate has no README
 [6/7] Installing WASM-bindgen...
 > [INFO]: wasm-bindgen already installed
 [7/7] Running WASM-bindgen...
 Done in 1 second
```

## 패닉이 일어났을 때

자주 잊히는 측면 중 하나는
프로그램이 충돌할 때도 뭔가가 출력된다는 점입니다.
러스트에서 "충돌"은 대개 "패닉"을 의미합니다.
(즉, "운영체제가 프로세스를 강제로 종료시킨 것"과 다르게
"통제된 충돌"입니다.)
패닉이 발생하면 기본적으로 "패닉 핸들러"가
몇 가지 정보를 콘솔에 출력합니다.

예를 들어,
`cargo new --bin foo`로 새로운 바이너리 프로젝트를
생성하고 `fn main`의 내용을 `panic!("Hello World")`로 고치면
프로그램을 실행했을 때 아래와 같은 결과가 나오게 됩니다:

```console
thread 'main' panicked at 'Hello, world!', src/main.rs:2:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

이 정보는 개발자에게 유용합니다.
(놀랍게도 `main.rs` 파일의 두 번째 줄에서 충돌이 발생했습니다.)
하지만 소스 코드를 볼 수 없는 사용자에게는 그다지
가치 있는 정보가 아닙니다.
사실 사용자 입장에서는 혼란에 더 가깝습니다.
따라서 커스텀 패닉 핸들러를 추가하여
더욱 사용자 친화적인 정보를 제공해야 합니다.

이를 위해 사용할 수 있는 라이브러리 중 하나는 [human-panic]입니다.
[human-panic]을 CLI 프로젝트에 추가하려면
`main` 함수의 시작 부분에서 `setup_panic!()` 매크로를
호출하면 됩니다:

```rust,ignore
use human_panic::setup_panic;

fn main() {
   setup_panic!();

   panic!("Hello world")
}
```

이제 사용자 친화적인 메시지가 출력됩니다.
사용자는 메시지를 읽고 어떻게 해야 하는지 알 수 있습니다:

```console
Well, this is embarrassing.

foo had a problem and crashed. To help us diagnose the problem you can send us a crash report.

We have generated a report file at "/var/folders/n3/dkk459k908lcmkzwcmq0tcv00000gn/T/report-738e1bec-5585-47a4-8158-f1f7227f0168.toml". Submit an issue or email with the subject of "foo Crash Report" and include the report as an attachment.

- Authors: Your Name <your.name@example.com>

We take privacy seriously, and do not perform any automated error collection. In order to improve the software, we rely on people to submit reports.

Thank you kindly!
```

[human-panic]: https://crates.io/crates/human-panic
[wasm-pack]: https://crates.io/crates/wasm-pack
