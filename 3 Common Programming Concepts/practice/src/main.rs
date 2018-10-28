fn main() {
    // 피보나치
    let x = fibonacci(8);
    println!("{}", x);
    let k = fibonacci_loop(2);
    println!("{}", k);
    // 온도 변환
    let y = celscius_to_farenheit(36.5);
    println!("{}", y);
}

fn fibonacci(x: i32) -> i32 {
    if x == 0 {
        return 0;
    } else if x == 1 {
        return 1;
    }
    fibonacci(x - 1) + fibonacci(x - 2)
}

fn fibonacci_loop(x: i32) -> i32 {
    let mut ans = 0;
    let mut tmp1 = 0;
    let mut tmp2 = 1;
    if x == 0 {
        return 0;
    }
    if x == 1 {
        return 1;
    }
    for _ in (1..x).rev() {
        ans = tmp1 + tmp2;
        tmp1 = tmp2;
        tmp2 = ans;
        println!("ans: {}, tmp: {}", tmp1, tmp2);
    }
    ans
}

fn celscius_to_farenheit(temperature: f64) -> f64 {
    (temperature * 9.0 / 5.0) + 32.0
}
