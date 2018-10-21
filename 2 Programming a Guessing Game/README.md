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
