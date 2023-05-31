# 설정 파일 사용하기

설정을 다루는 것은 짜증 날 수도 있습니다.
특히 다양한 운영체제를 지원해야 하는 경우
각자의 단기, 장기 보관 파일 저장 위치를
고려해야 하므로 더욱 그렇습니다.

여기엔 여러 해결 방안이 있는데,
일부는 다른 것들보다 더욱 로우 레벨의 해결책이기도 합니다.

이때 사용하기 가장 쉬운 크레이트는 [`confy`]입니다.
[`confy`]는 여러분의 애플리케이션 이름을 묻고
`struct`(`Serialize`, `Deserialize`를 derive)를 통해
설정 레이아웃을 명시하도록 합니다.
이렇게만 하면 나머지는 [`confy`]가 찾아냅니다!

```rust,ignore
#[derive(Debug, Serialize, Deserialize)]
struct MyConfig {
    name: String,
    comfy: bool,
    foo: i64,
}

fn main() -> Result<(), io::Error> {
    let cfg: MyConfig = confy::load("my_app")?;
    println!("{:#?}", cfg);
    Ok(())
}
```

물론 설정 가능성(configurability)을 포기해야 하지만,
[`confy`]는 정말 사용하기 쉽습니다.
여러분이 간단한 설정만을 원한다면
[`confy`] 크레이트가 바로 여러분을 위한 것일 수 있습니다.

[`confy`]: https://docs.rs/confy/0.3.1/confy/

## 설정 환경

<aside class="todo">

**TODO**

1. 기존 크레이트를 평가한다.
2. Cli-args + 다양한 설정 + 환경 변수
3. 모든 경우에 [`configure`]를 쓸 수 있는가? 더 나은 래퍼가 있는가?

</aside>

[`configure`]: https://docs.rs/configure/0.1.1/configure/
