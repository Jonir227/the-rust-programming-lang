# Control Flow

특정 코드를 돌릴지 말지, if 컨디션에 따라서 분기를 나누거나, while 이 참인이상 계속해서 코드를 실행하는 것은 대부분의 프로그래밍 언어에서 기본이다.

Rust 에서 `if`구문과 `loop`구문은 기본적인 제어를 위한 방법이다.

# `if` Expressions

`if`는 상태에 따른 분기처리를 가능하게 해준다.

다음은 `if`의 예시이다.

```rust
fn main() {
  let number = 3;

  if number < 5 {
    println!("condition was true");
  } else {
    println!("condition was false");
  }
}
```

`else`표현을 이용해서 거짓인 조건에 대해서 처리를 할 수도 있따.

`if`의 조건 안에 들어가 있는 것은 무조건 `bool`타입이여야 한다. 다음과 같은 코드는 실행이 되지 않는다는 말이다.

```rust
let number = 3;
if number {
  println!("number was there");
}
```

Rust 는 불린이 아닌 타입을 불린으로 자동 변환하지 않기때문에 무조건 참/거짓의 부울린 값을 넣어야 한다.

# Handling Multiple Conditions with `else if`

물론 else if 구문도 사용가능하다.

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

# Using `if` in a `let` statement

`if`가 Expression 이기 때문에, RValue 로서 사용할 수 있다.

```rust
fn main() {
  let condition = true;
  let number = if condition {
    5
  } else {
    6
  }
  println!("{}", number);
}
```

이때, 표현식으로 사용된 if 블록 안에서 리턴되는 두 값은 같은 타입을 가져야 한다. 다음과 같은 경우라면 에러가 난다.

```rust
fn main() {
  let conition = true;
  let number = if condition {
    5
  } else {
    "six"
  }
}
```

# Repetition with Loops

러스트는 다양한 루프를 제공한다.

- `loop`
- `while`
- `for`

하나씩 보면서 알아보자

## Repeating Code with `loop`

```rust
fn main() {
  loop {
    println!("again!");
  }
}
```

위의 코드는 무한루프로 프린트를 계속해서 하는 코드이다.

## Returning form loops

`loop`의 사용법중 하나는 실패할 수 있는 작업에 대해서 계속해서 다시 시도하는 것이다. 예를들어, 다른 스레드가 작업을 끝냈는지에 대한 확인을 이를 이용해서 할 수 있다. 하지만, 그 값을 다른 코드에 전달하고 싶을 수도 있다. 그럴때에는 `break`를 사용한다.

```rust
fn main() {
  let mut counter = 0;
  let result = loop {
    counter += 1;
    if counter == 10 {
      break counter * 2;
    }
  }
  // result 와 20이 같은지 확인~
  assert_eq!(result, 20);
}
```

## Conditional Loops with `while`

루프 안에 조건으로 루프를 계속해서 할지 안할지 판단하는 패턴은 일반적이다.

Rust 에서는 이런 패턴을 `while`을 이용하여 구현한다.

```rust
fn main() {
  let mut number = 3;
  // 1씩 줄이다가 0이되면 종료한다.
  while number != 0 {
    println!("{}!", number);
    number = number - 1;
  }
  println!("LIFTOFF!!!");
}
```

## Loop Trough a Conllection with `for`

컬렉션을 루프할때 `while`을 사용할 수 있다. 그 예이다.

```rust
fn main() {
  let a= [10, 20, 30, 40, 50];
  let mut index = 0;
  while index < 5 {
    println!("the value is: {}", a[index]);
    index = index + 1;
  }
}
```

하지만 이런 방법은 에러를 발생시키기 너무 쉽다. length 가 정확히 무엇인지 알 수가 없을때에 에러를 발생시키기 때문이다.

다음은 `for`를 이용한 같은 예제이다.

```rust
fn main() {
  let a= [10, 20, 30, 40, 50];
  for element in a.iter() {
    println!("the value is: {}", element);
  }
}
```

다음 코드로, a 를 완전하게 순회함과 동시에 인덱스로 인한 버그 발생확률을 낮췄다.

꼭 itreable 한 객체를 순회할때만 쓰는 것이 아니라, 일반적인 반복 상황에서도 러스트의 스탠다드 라이브러리인 `Range`를 사용하여 순회할 수 있다. 다음은 `rev`를 이용한 `for`구문의 예이다.

```rust
    fn main() {
    	for number in (1..4).rev() {
    		println!("{}!", number);
    	}
    	println!("LIFTOFF!!");
    }
```

# Summary

연습하고 싶다면 다음과 같은 프로그램을 만들어 봐라

- Convert temperature between Fahrenheit and Celsius.
- Generate the nth Fibonacci Number
