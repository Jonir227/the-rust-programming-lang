fn main() {
    // 피보나치
    let x = fibonacci(8);
    println!("{}", x);
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

fn celscius_to_farenheit(temperature: f64) -> f64 {
    (temperature * 9.0 / 5.0) + 32.0
}
