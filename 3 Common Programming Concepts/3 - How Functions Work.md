# How Functions Work

# Functions

함수는 러스트 코드 전체에 널리 퍼져 있다.

- `main`함수는 러스트 프로그램의 엔트리 포인트이다.
- `fn`키워드로 함수를 선언 할 수 있다.
- Rust 는 함수 이름 규칙으로 *snake case*를 사욯한다.

다음은 함수의 선언부, 사용한 부분이다.

```rust
    fn main() {
    	println!("Hello World!");
    	another_function();
    }

    fn another_function() {
    	println!("Another Fucntion");
    }
```

- 함수는 프로그램 어디에 선언되어도 상관 없이 호출할 수 있다.

# Fucntion Parameters

함수는 파라미터를 가지질 수 있도록 정의될 수 있다. 기술적으로는 argments 라고 부지만, 일반적으로는 parameter 라고 많이 부른다.

다음은 파라미터를 가진 함수의 예이다.

```rust
    fn main() {
    	another_function(5);
    }

    fn another_function(x: i32) {
    	println!("The value of x is : {}", x);
    }

    // 여러개를 넘길 수 도 있다.
    fn another_function2(x: i32, y: i32) {
    	println!("the value of x is : {}", x);
    	println!("the value of y is : {}", y);
    }
```

# Function Bodies

함수 바디는 여러 선언문으로 이루어져 있다. 여때까지는 함수 종료 표현이 없는 예제들만을 보았지만, 선언문의 일부로 표현을 본 적 은 있을 것.

러스트는 표현문 기반의 언어이다. 다음부터 선언문과 표현이 어떻게 다른지 알아 볼 것

# Statements and Expressions

- 선언문(Statements) ⇒ 몇가지 동작을 행하고 값을 리턴하지 않는 명령문
- 표현식(Expressions) ⇒ 명령문을 평가하여 값을 리턴하는 식

예를 들어, 변수를 선언하고 값을 할당하는 것은 statement 이다. 다음을 보자.

```rust
    fn main() {
    	let y = 6;
    }
```

하나의 Statement 가 존재하는 main 함수이다.

함수의 선언문도 역시 Statement 이다. Statement 는 값을 리턴하지 않는다. 따라서, let 선언문을 다른 변수에 할당할 수 없다. 다음의 경우에는 에러를 뱉는다.

```rust
    fn main() {
    	let x = (let y = 6); // Statement는 값을 리턴하지 않는다!
    }
```

`let y = 6;`이라는 선언문이 값을 리턴하지 않기 때문에 x 에 변수 할당을 할 수 없다. 루비나 c 와 같은 다른 언어와 다른 점이 이것이다.

표현식은 특정 명령문의 값을 평가한다. 간단한 수학 식을 생각해보자. `5+6`같은 식은 표현식으로 11 을 리턴한다. 표현식은 statement 의한 부분으로서 기능할 수 있다. 다음과 같은 경우 모두 표현식이다.

- 함수의 호출
- RValue
- `{}`로 선언된 블록

```rust
  fn main() {
    let x = 5;

    let y = {
      let x = 3;
      x + 1
    };

    println!("The value of y is : {}", y);
  }
```

위 예제에서

```rust
    {
    	let x = 3;
    	x + 1
    }
```

는 표현식이다. 마지막에 평가되는 4 를 리턴한다.

# Functions with Return Values

함수는 값을 리턴할 수 있다. 리턴하는 값에 이름을 붙이지는 않지만, 화살표 표현식(`->`)으로 타입을 지정할 수 있다.

다음은 그 예제이다.

```rust
    fn five() -> i32 {
    	5 // 세미콜론을 붙이면 표현식이 아니고 선언문이 되기 때문에 값을 리턴하지 않는다.
    }

    fn main() {
    	let x = five();
    	println!("The value of x is : {}", x);
    }
```

여기서 주의해야 할 점은 블록 안에서 평가되는 값이 리턴되기 위해서는 표현식이여야 한다는 점이다. 따라서, 세미콜론을 붙이면 선언문이 되기때문에 리턴되는 식에서는 세미콜론을 붙이면 안된다.

```rust
    fn main() {
    	let x = plus_one(5);
    	println!("The value of x is : {}", x);
    }
    fn plus_one(x: i32) -> i32 {
    	x + 1; // 에러!!
    }
```

위 식의 에러는 "mismatched types"로 나오는데, x + 1 은 선언문으로 값을 리턴하지 않고 지나가고, 결국 함수의 바디 블록은 아무것도 리턴하지 않아 "()"가 리턴되게 된다. 따라서 선언된 i32 와 값이 매칭되지 않으므로, 에러가 나는 것이다.
