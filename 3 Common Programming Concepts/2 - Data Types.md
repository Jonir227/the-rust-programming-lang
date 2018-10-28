# Data Types

Rust 의 모든 값은 러스트가 어떻게 값을 처리해야 하는 지 알려주는 data type 을 가지고 있다.

이번 장에서는 두 가지 데이터 타입에 대해서 알아볼 것이다.

- Scalar
- Compound

러스트는 정적 타입 언어이다. 즉 컴파일 시점에서 모든 값에 대해서 타입을 알고있어야 한다는 의미이다. 컴파일러는 개발자가 어떤 타입을 사용할지, 어떻게 사용하는지에 따라서 타입 체킹을 개발자에게 넘겨줄 수 있다.

예를들어, `String`타입에서 `parse`를 이용하여 숫자로 변경하는 챕터 2 의 예제를 보자.

```rust
let guess: u32 = "42".parse().expect("Not a Number");
```

만약, 타입을 지정하지 않는다면, Rust 는 에러를 뱉는다. 컴파일러가 사용자가 어떤 타입을 사용하고 싶어하는지 단서가 부족하기 때문이다.

# Scalar Types

_scalar_ type 은 하나의 값을 나타낸다. Rust 는 네가지 원시적인 scalar 타입을 가지고 있다. Integers, floating-point numbers, Booleans, characters 들이 그것들이다.

## Integer Types

정수는 소수가 없는 숫자이다. 챕터 2 에서 정수인 `u32`타입을 사용했었다. 이 타입은 32 비트의 unsigned Integer 를 말한다.

[Integer Types in Rust](https://www.notion.so/bb47d9f304cf46d89d3b1b0b9fb13e2e)

변수는 양의 수인지, 정수인지 정확한 사이즈로 지정이 가능하다. Signed 와 Unsigned 는 음인지 양인지를 결정할 수 있다.

각각의 숫자는 표현가능한 비트수를 나타낸다. -(2n - 1) to 2n - 1 까지의 값을 가진다.

추가로, `isize`와 `usize`는 프로그램이 도는 환경에 따라서 결정된다. 이때, 환경은 실행하는 프로그램의 프로세서를 말한다. 32 비트 프로세서라면 32 비트를, 64 비트라면 64 비트를 할당한다.

리터럴로도 숫자를 선언할 수 있는데 그 예는 다음과 같다.

- Decimal ⇒ `98_222`
- Hex ⇒ `0xff`
- Octal ⇒ `0o77`
- Binary ⇒ `0b1111_0000`
- Byte(`u8` Only) ⇒ `b'A'`

인테저의 값이 수용가능한 양을 벗어났을 때에는 Integer Overflow 가 발생한다. 러스트는 이 오버플로우에 대해서 흥미로운 규칙을 가지고 있다. 러스트는 디버그 모드에서 컴파일 할때 이런 종류의 이슈를 찾아내고, 여러분의 프로그램을 panic 상태로 만들 것이다. panic 은 러스트가 프로그램을 에러를 뱉음과 동시에 프로그램이 종료되는 것으로 챕터 9 에서 계속해서 알아볼 것.

릴리즈에서는 Rust 는 오버플로우를 체크하지 않는다. 대신 "two's compliment wrapping"이라는 것을 적용한다. 256 이 0 이 되고, 257 은 1 이 된다.

## Floating-point Types

Rust 는 두가지 부동소수점 숫자를 나타내는 타입이 있다. 바로 `f32`와 `u64`로 각각 32, 64 비트 사이즈를 가진다. 기본 부동소수는 `u64`로 취급되는데, 대부분의 모던 cpu 들이 `f32`와 비슷한 속도를 내고 더 높은 정확도를 가지고 있기 때문이다.

다음은 부동소수 선언의 예시이다.

```rust
fn main() {
  let x = 2.0 // f64
  let y: f32 = 3.0 // f32
}
```

부동소수들은 IEEE-754 표준에 따라 정의되었다. `f32`는 단정밀도이고, `f64`는 배정밀도이다.

## Numeric Operations

러스트는 대부분의 숫자에 대한 기본적인 연산을 지원한다. 덧셈, 뺄셈, 곱샘, 나눗셈, 나머지가 그 예이다. 다음 코드는 `let`구문으로 짜여진 예시들이다.

```rust
fn main() {
  let sum = 5 + 10;
  let diff = 95.5 - 4.3;
  let multiplicaiton = 4 * 30;
  let quotient = 56.7 / 43.2.;
  let reaminder = 43 % 5;
}
```

## The Boolean Value

러스트 역시 부울이 지원된다. 러스트에서는 `bool`키워드로 이것을 표현한다.

```rust
fn main() {
  let t = true;
  let f: bool = false;
}
```

## The Character Type

러스트에서는 문자도 지원한다. 러스트의 `char`타입은 Unicode Scalar Value 로서, ASCII 보다 더 많은 것을 표현할 수 있다. 다음은 그 예시이다.

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

Unicode 는 사람이 생각하는 '글자'와는 조금 다를 수도 있는데 8 장에서 자세히 설명할 예정임.

# Compound Types

컴파운드 타입은 여러 값을 하나의 타입으로 그룹화 할 수 있는 타입을 말한다. 러스트는 두가지 컴파운드 타입을 가지고 있다. ⇒ 튜플 & 배열

## The Tuple Type

튜플은 다양한 데이터 타입을 하나로 묶는 일반적인 형태이다.

다음은 튜플의 예이다. 자바스크립트에서처럼 디스트럭쳐링도 가능하다.

또한, 튜플의 각 원소는 `.`을 통해서 접근이 가능하다.

```rust
fn main() {
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let (x, y, z) = tup;
  println!("the value of y is : {}", y); // 6.4
  let five_hundred = x.0;
  let six_point_four = x.1;
}
```

## The Array Type

여러 값을 하나로 묶는 다른 방법은 배열이다. 튜플과는 다르게, 모든 값들은 같은 타입을 가지고 있어야 한다. Rust 에서의 배열과 다른 언어의 배열과 다른 점은 배열이 고정된 크기를 가지고 있다는 점이다.

다음은 배열의 선언이다.

```rust
fn main() {
  let a = [1, 2, 3, 4, 5];
  // 타입의 선언도 가능하다. 타입의 선언은 [type; number]와 같이 이루어진다.
  let b: [i32; 5] = [1, 2, 3, 4, 5];
  println!("array b have 3rd value {}", b[2]); // []로 배열의 값에 접근할 수 있다.
}
```

배열이 가능하지 않은 값을 접근하려고 한다면 컴파일 타임에 out of bound 에러가 난다.
