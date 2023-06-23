# grrs의 첫 구현

지난 챕터에서 커맨드라인 인자를 다룬 뒤
우리는 입력 데이터를 얻었고,
실제 프로그램 작성을 시작할 수 있게 되었습니다.
지금은 `main` 함수에 아래 한 줄만 있습니다:

```rust,ignore
{{#include impl-draft.rs:15:15}}
```

이제 입력받은 파일을 열어봅시다.

```rust,ignore
{{#include impl-draft.rs:16:16}}
```

<aside>

**참고:**
여기서 [`.expect`] 메서드가 보이시나요?
이 메서드는 값(여기서는 입력 파일)을 읽을 수 없을 때
프로그램을 즉시 종료하는 단축 함수입니다.
이는 별로 좋은 방법이 아니며,
다음 챕터 [더 나은 에러 보고]에서
어떻게 개선할 수 있을지 살펴보겠습니다.

[`.expect`]: https://doc.rust-lang.org/1.39.0/std/result/enum.Result.html#method.expect
[더 나은 에러 보고]:./errors.html

</aside>

이제 파일의 각 라인을 순회하며
주어진 패턴을 포함하는 라인을 출력해 봅시다:

```rust,ignore
{{#include impl-draft.rs:18:22}}
```

## 마무리

여러분의 코드는 이제 아래와 같아야 합니다:

```rust,ignore
{{#include impl-draft.rs}}
```

`cargo run -- main src/main.rs`으로 잘 동작하는지 확인해 보세요!

<aside class="exercise">

**연습:**
이는 최선의 구현은 아닙니다.
위 코드는 전체 파일을 읽어 메모리에 올리는데,
큰 파일도 통째로 메모리에 올리게 됩니다.
최적화할 수 있는 방법을 찾아보세요!
(`read_to_string()` 대신 [`BufReader`]를 사용하는 방법을 생각해 볼 수 있습니다.)

[`BufReader`]: https://doc.rust-lang.org/1.39.0/std/io/struct.BufReader.html

</aside>
