# 시그널 다루기

커맨드라인 애플리케이션과 같은 프로세스는
운영체제가 보낸 시그널에 반응해야 합니다.
시그널의 가장 흔한 예시는 아마도, 일반적으로 프로세스를 종료시킬 때
쓰는 <kbd>Ctrl</kbd>+<kbd>C</kbd>일 것 입니다.
러스트 프로그램에서 시그널을 다루기 위해서는
시그널에 반응하는 방법뿐 아니라
시그널을 수신하는 방법에 대해서도 고민해 봐야 합니다.

<aside>

**참고:**
애플리케이션을 우아하게 종료할 필요가 없다면
기본적인 처리 방식도 괜찮습니다.
(즉, 프로그램을 즉시 종료하고, 파일 핸들러와
같은 리소스는 OS가 정리하도록 하는 식입니다.)
그런 경우에는 이 챕터의 내용을 따를 필요가 없습니다!

그러나, 애플리케이션이 종료되기 전에 스스로 리소스를
정리해야 하는 경우에는 이 챕터가 매우 중요합니다!
예를 들어,
애플리케이션이 적절하게 네트워크 커넥션을 닫아야 한다거나,
(종료 전에 다른 프로세스에게 "good bye"를 보내는 동작)
임시 파일을 지워야 한다거나,
시스템 설정을 초기화하는 것과 같은
절차가 필요할 수 있습니다.

</aside>

## 운영체제 간 차이점

유닉스 시스템(리눅스, macOS, FreeBSD 등)에서
프로세스는 [시그널]을 받을 수 있습니다.
프로세스는 시그널을 받아서 프로그램이 정의한 방식으로
시그널을 처리하는 기본적인 방법(OS가 제공)으로
시그널에 반응할 수도 있고,
시그널을 통째로 무시할 수도 있습니다.

[시그널]: https://manpages.ubuntu.com/manpages/bionic/en/man7/signal.7.html

윈도우즈에는 시그널이 없습니다.
대신 [콘솔 핸들러]를 이용하여 이벤트가 발생했을 때
실행되는 콜백을 정의할 수 있습니다.
또한 윈도우즈에는 0으로 나누기, 잘못된 접근, 스택오버플로우 등 모든 종류의
시스템 예외를 처리할 수 있는 [구조적 예외 처리]도 있습니다.

[콘솔 핸들러]: https://learn.microsoft.com/ko-kr/windows/console/console-control-handlers
[구조적 예외 처리]: https://docs.microsoft.com/ko-kr/windows/desktop/debug/structured-exception-handling

## Ctrl+C 다루기

[ctrlc] 크레이트는 이름 그대로의 일을 합니다.
[ctrlc]는 다양한 플랫폼에 대해 사용자가
<kbd>Ctrl</kbd>+<kbd>C</kbd>을 눌렀을 때
프로그램이 반응할 수 있도록 만들어 줍니다.
[ctrlc]의 주요 사용법은 아래와 같습니다:

[ctrlc]: https://crates.io/crates/ctrlc

```rust,ignore
{{#include signals-ctrlc.rs}}
```

물론 이렇게 하면 별 도움이 되지 않습니다.
메시지를 출력할 뿐 프로그램을 종료시키지는 않으니까요.

실제 프로그램의 경우,
시그널 핸들러에서 변수를 설정해
프로그램 곳곳에서 상태를 체크하는 것이 좋습니다.
예를 들어,
시그널 핸들러에서 `Arc<AtomicBool>`
(스레드 간에 공유할 수 있는 불리언 타입)
변수를 설정하면, 루프(hot loop)를 돌거나
스레드를 대기할 때 그 값을 주기적으로
체크하면서 true가 되면 프로그램을
종료하도록 할 수 있습니다.

## 다른 시그널 다루기

[ctrlc] 크레이트는 <kbd>Ctrl</kbd>+<kbd>C</kbd>,
혹은 유닉스 시스템에서 `SIGINT` ("인터럽트" 시그널)라고 불리는
시그널만 다룰 수 있습니다.
더 많은 유닉스 시그널에 반응하기 위해서는
[signal-hook]를 사용해야 합니다.
[signal-hook]의 설계는 [이 블로그 글][signal-hook-post]에
설명되어 있으며, 현재 가장 광범위한 커뮤니티 지원을 받는 라이브러리입니다.

여기 간단한 예시가 있습니다:

```rust,ignore
{{#include signals-hooked.rs}}
```

[signal-hook-post]: https://vorner.github.io/2018/06/28/signal-hook.html

## 채널 사용하기

변수를 설정하고, 프로그램이 그 변수를 체크하도록 만드는 대신
채널을 사용할 수 있습니다:
채널을 만들면 시그널을 수신할 때마다 시그널 핸들러가
채널로 값을 내보내 줍니다.
애플리케이션 코드에서는 한 채널과
다른 채널을 스레드 간의 동기화 지점으로 사용하게 됩니다.
[crossbeam-channel]을 사용하면 아래와 같은 모습이 됩니다:

[crossbeam-channel]: https://crates.io/crates/crossbeam-channel

```rust,ignore
{{#include signals-channels.rs}}
```

## 퓨쳐(futures)와 스트림 사용하기

[tokio]를 사용하고 있다면,
여러분은 이미 비동기 패턴과 이벤트 주도 설계를 적용하여
애플리케이션을 작성하고 있을 확률이 높습니다.
이때는 crossbeam의 채널을 직접 사용하지 않고
[signal-hook]의 `tokio-support` 기능을 사용할 수 있습니다.
이 기능을 이용하면 [signal-hook]의 `Signals` 타입에 대해
[`.into_async()`]를 호출하여 `futures::Stream`을 구현하는
새로운 타입을 얻을 수 있습니다.

[signal-hook]: https://crates.io/crates/signal-hook
[tokio]: https://tokio.rs/
[`.into_async()`]: https://docs.rs/signal-hook/0.1.6/signal_hook/iterator/struct.Signals.html#method.into_async

## 첫 Ctrl+C 시그널을 처리하는 도중 또 다른 Ctrl+C 시그널을 수신했을 때

사용자가 <kbd>Ctrl</kbd>+<kbd>C</kbd>를 누르면
여러분의 프로그램은 몇 초 뒤 종료되거나
진행 상황을 알려줄 것입니다.
만약 그렇지 않으면 사용자는
<kbd>Ctrl</kbd>+<kbd>C</kbd>을 한 번 더 누를 것입니다.
이때 일반적인 동작은 애플리케이션을 즉시 종료하는 것입니다.

