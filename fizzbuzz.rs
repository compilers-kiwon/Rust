fn main() {
    for i in 1..=100 {
        // 값을 match 로 분기. 튜플을 이용한다.
        match (i%3, i%5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _      => println!("{}", i)
        }
    }
}