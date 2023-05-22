# 더 나은 에러 보고

우리는 모두 에러가 발생할 것이라는 사실을 받아들일 수밖에 없습니다.
다른 언어들과 다르게, 러스트를 사용할 때는 이 현실을 무시하기가 쉽지 않습니다.
러스트에는 예외가 없으며, 모든 발생 가능한 에러 상태는 보통 함수의 반환 타입으로 표현됩니다.

## Result

[`read_to_string`]과 같은 함수는 문자열을 반환하지 않습니다.
대신, `String`이나 에러 타입 중 하나를 담은 [`Result`]를 반환합니다.
(여기서 에러 타입은 [`std::io::Error`])

[`read_to_string`]: https://doc.rust-lang.org/1.39.0/std/fs/fn.read_to_string.html
[`Result`]: https://doc.rust-lang.org/1.39.0/std/result/index.html
[`std::io::Error`]: https://doc.rust-lang.org/1.39.0/std/io/type.Result.html

어떤 타입이 들어있는지 어떻게 알 수 있을까요?
`Result`는 `enum`이기 때문에,
`match`를 이용해 확인할 수 있습니다.

```rust,no_run
let result = std::fs::read_to_string("test.txt");
match result {
    Ok(content) => { println!("File content: {}", content); }
    Err(error) => { println!("Oh noes: {}", error); }
}
```

<aside>

**참고:**
enum이 무엇이고, 러스트에서 enum이 어떻게 동작하는지 잘 모르겠나요?
그렇다면 [러스트 책의 이 챕터를 읽어보세요](https://doc.rust-lang.org/1.39.0/book/ch06-00-enums.html).

</aside>

## Unwrap

이제 우리는 파일 내용에 접근할 수 있지만,
`match` 블록 이후로 실제 뭔가를 할 수는 없습니다.
이를 위해서는 에러 케이스를 처리해야 합니다.
이때 어려운 부분은 `match` 블록의 모든 분기가 같은 타입을 반환해야 한다는 점입니다.
하지만 간단한 트릭이 있습니다:

```rust,no_run
let result = std::fs::read_to_string("test.txt");
let content = match result {
    Ok(content) => { content },
    Err(error) => { panic!("Can't deal with {}, just exit here", error); }
};
println!("file content: {}", content);
```

match 블록 이후에 `content`를 문자열로 사용할 수 있습니다.
만약 `result`가 에러라면 문자열은 존재하지 않게 되지만,
`result`를 사용하기 전에 프로그램이 종료될 것이기 때문에 문제가 없습니다.

조금 과격해 보이지만, 매우 편리한 방법입니다.
만약 파일을 읽는 프로그램이 파일이 존재하지 않는 경우 아무것도 할 수 없다면,
프로그램 종료는 적합한 전략입니다.
여기에는 `unwrap`이라는 `Result`의 단축 메서드도 있습니다:

```rust,no_run
let content = std::fs::read_to_string("test.txt").unwrap();
```

## 패닉할 필요 없습니다

물론 프로그램 종료가 에러를 다루는 유일한 방법은 아닙니다.
`panic!` 대신 단순히 `return`을 사용할 수 있습니다:

```rust,no_run
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let result = std::fs::read_to_string("test.txt");
let content = match result {
    Ok(content) => { content },
    Err(error) => { return Err(error.into()); }
};
# Ok(())
# }
```

그러나 이렇게 하려면 함수의 반환 타입을 변경해야 합니다.
지금까지의 모든 예시에 실제로는 숨겨진 부분이 있었습니다.
바로 이 코드가 속해 있는 함수 시그니처입니다.
`return`이 있는 앞 예시에서 이것이 매우 중요해집니다.
여기 _전체_ 예시가 있습니다:

```rust,no_run
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = std::fs::read_to_string("test.txt");
    let content = match result {
        Ok(content) => { content },
        Err(error) => { return Err(error.into()); }
    };
    println!("file content: {}", content);
    Ok(())
}
```

우리의 반환 타입은 `Result`입니다!
덕분에 두 번째 match 분기에서 `return Err(error);`을 사용할 수 있습니다.
맨 마지막에 `Ok(())`가 보이시나요?
이는 함수의 기본 반환 값이며, "결과가 정상이고, 내용은 없다"라는 의미입니다.

<aside>

**참고:**
왜 `return Ok(());`라고 쓰지 않았을까요?
`Ok(())`는 쉽게 사용할 수 있으며, 완전히 유효한 문법입니다.
러스트에서는 어떤 블록에서든 마지막 표현식은 블록의 반환 값이며,
`return`을 관례적으로 생략할 수 있습니다.

</aside>

## 물음표

`.unwrap()`을 호출하는 것은 `match`의 에러 분기에서
`panic!`을 사용하는 것과 동일한 일종의 단축어입니다.
또 다른 단축어로는 에러 분기의 `return`을 위한 `?`가 있습니다.

맞아요, 물음표입니다.
`Result` 타입의 값에 이 연산자를 붙일 수 있고,
러스트는 내부적으로 이 연산자를 우리가 작성한
`match`와 매우 비슷한 것으로 확장해줍니다.

한번 해보세요:

```rust,no_run
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("test.txt")?;
    println!("file content: {}", content);
    Ok(())
}
```

정말 간결하죠!

<aside>

**참고:**
여기서 사용만 할 때는 알 필요가 없는 몇 가지 다른 일들이 일어납니다.
예를 들어,
`main` 함수의 에러 타입은 `Box<dyn std::error::Error>`입니다.
하지만 앞서 봤듯이, `read_to_string`은 [`std::io::Error`]를 반환합니다.
이것이 동작하는 이유는, `?`가 에러 타입을 변환하는 코드로 확장되기 때문입니다.

`Box<dyn std::error::Error>`는 흥미로운 타입입니다.
`Box`는 표준 [`Error`][`std::error::Error`] 트레잇을 구현하는 타입이라면 _어떤 타입이든_ 담을 수 있습니다.
즉, 기본적으로 모든 에러를 박스에 담을 수 있습니다.
따라서 `Result`를 반환하는 모든 함수에서 `?`를 사용할 수 있습니다.

[`std::error::Error`]: https://doc.rust-lang.org/1.39.0/std/error/trait.Error.html

</aside>

## 맥락 제공하기

`main` 함수에서 `?`를 사용하여 에러를 받는 것은 괜찮지만,
최선의 방법은 아닙니다.
예를 들어:
`std::fs::read_to_string("test.txt")?`를 실행할 때
`test.txt`가 존재하지 않는다면,
아래와 같은 출력을 보게 될 것입니다:

```text
Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

코드가 파일 이름을 포함하지 않는다면,
어떤 파일이 `NotFound`인지 말해주기가 상당히 어렵습니다.
이를 해결하는 여러 방법이 있습니다.

예를 들어, 우리만의 에러 타입을 만들 수 있습니다.
그리고 커스텀 에러 메시지를 만들면 됩니다:

```rust,ignore
{{#include errors-custom.rs}}
```

이제,
프로그램을 실행하면 우리가 만든 커스텀 에러 메시지가 출력됩니다:

```text
Error: CustomError("Error reading `test.txt`: No such file or directory (os error 2)")
```

그다지 예쁘지는 않지만,
나중에 디버그 출력을 우리 타입에 맞게 적용할 수 있습니다.

이러한 패턴은 실제로 매우 일반적입니다.
그러나 원본 에러를 저장하지 않고 문자열만
보여준다는 문제가 있습니다.
이러한 문제를 해결하기 위해 주로 [`anyhow`] 라이브러리를 사용합니다.
이를 통해 `CustomError` 타입처럼 [`Context`] 트레잇을 이용해 설명을 추가할 수 있습니다.
더불어, 원본 에러를 저장함으로써 에러의 근본 원인을 알 수 있도록 해주는
에러 메시지 "체인"을 제공합니다.

[`anyhow`]: https://docs.rs/anyhow
[`Context`]: https://docs.rs/anyhow/1.0/anyhow/trait.Context.html

먼저 `Cargo.toml` 파일의 `[dependencies]` 섹션에
`anyhow = "1.0"`을 추가하여
`anyhow` 크레이트를 가져옵니다.

전체 예시는 아래와 같습니다:

```rust,ignore
{{#include errors-exit.rs}}
```

실행하면 아래와 같이 에러가 출력됩니다:

```text
Error: could not read file `test.txt`

Caused by:
    No such file or directory (os error 2)
```
