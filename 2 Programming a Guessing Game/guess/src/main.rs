use std::io; // io 라이브러리를 스코프에 가져옴.

// main은 프로그램의 엔트리 포인트로 사용된다.
fn main() {
    println!("Guess The Number!");
    println!("Please input Your Guess");

    // mutable변수 guess 선언.
    // 기본 스트링 타입인 String의 인스턴스를 생성.
    // :: 연산자는 뒤따르는 함수가 앞선 타입에 관련된 것이라는것의 의미.
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed To Read Line");

    println!("You Guessed {}", guess);
}
