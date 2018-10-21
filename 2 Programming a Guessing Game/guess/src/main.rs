extern crate rand; // 외부 crate를 불러옴.

use std::io; // io 라이브러리를 스코프에 가져옴.
use std::cmp::Ordering; // Ordering Enum을 가져옴.
use rand::Rng; // 랜덤 라이브러리를 호출

// main은 프로그램의 엔트리 포인트로 사용된다.
fn main() {
    println!("Guess The Number!");

    // 현재 스레드에 로컬인 랜덤 변수를 생성. 1~100까지
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Secret Number is {}", secret_number);

    // 올바른 답을 입력할때까지 루프
    loop {
        println!("Please input Your Guess");
        // mutable변수 guess 선언.
        // 기본 스트링 타입인 String의 인스턴스를 생성.
        // :: 연산자는 뒤따르는 함수가 앞선 타입에 관련된 것이라는것의 의미.
        let mut guess = String::new();

        // import된 io의 stdin함수를 호출.
        // &mut guess는 &로 레퍼런스를 넘기고, mut로 mutable하게 변수를 넘긴다는 것을 의미한다.
        io::stdin()
            .read_line(&mut guess)
            // expect는 에러가 발생하면 에러를 잡아 리턴하는 메소드이다.
            .expect("Failed To Read Line");

        // {} 안에 뒤이어 오는 변수들을 넣는다. 여러개의 {} 에 여러개의 변수를 넣을 수도 있다.
        println!("You Guessed {}", guess);

        // 처음 guess는 string이기 때문에 숫자인 secret_number와 비교 불가능.
        // 따라서 u32(32-bit integer)로 변환해준다.
        // 숫자 변환에 실패했을때 에러 핸들링도 추가.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        /*
            패턴 매칭
            guess.cmp가 리턴하는 결과에 따라서 그에 맞는 식을 실행한다.
        */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too Small"),
            Ordering::Greater => println!("too Big"),
            Ordering::Equal => {
                println!("you Win!");
                break;
            }
        };
    }
}
