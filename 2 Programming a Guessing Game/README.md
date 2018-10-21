# Programming a Guessing Game

러스트를 이용해 숫자맞추기 게임을 만드는 예제.

## 라이브러리 가져오기

```rust
use [lib];
```

## 프로그램의 엔트리 포인트

```rust
fn main() {}
```

함수 선언은 `fn` 키워드를 사용한다.

## 콘솔 출력

```rust
println!()
```

첫 번째 인자로 출력할 문자열을 인자로 넘기고, 그 다음 인자부터는 인자로 받은 문자열에 {} 부분에 대입하는 변수들이다.

## 변수

rust 는 `let`키워드로 변수를 생성한다. 주의해야할 점은 생성되는 모든 변수는 immutable 하다는 것이다. mutable 한 변수는 `mut`키워드를 붙여서 생성해주어야 한다.

```rust
let a = 1; // immutable
let mut b = 2; // mutable
```

## Result Type

rust 는 standard library 에 `Result`라는 타입이 존재한다. 다양한 형태의 Result 가 존재하고 예제에서는 `io::Result`의 타입이 사용된다.

- `Result`는 `enum`을 가지고 있다. `enum`은 고정된 값을 가지고 있으며 `Result`의 경우에는 `Ok`와 `Err`두가지로 이루어져 있다.
- `Result`는 에러 핸들링에 사용된다. `Result`가 가지고 있는 `expect`로 이를 통제하는것이 가능한데, `Ok`의 경우에는 `Result`로 성공적으로 리턴된 값을 반환하고, 그렇지 않다면 `expect`에 정의된 문자열이 표시된다.

## 패키지 추가

Rust 에서 패키지는 크레이트라고 불린다.

`Cargo.toml`파일에 원하는 크레이트 이름과 버전을 작성하고 `cargo build`를 돌리면 된다. 이후, 호환되는 범위 안에서 패키지를 업데이트 하려면 `cargo update`를 사용.
