fn fibo(n:i64) -> i64 {
    if n == 1 { return  0; }
    if n == 2 { return  1; }
    return  fibo(n-2)+fibo(n-1);
}

fn input(prompt: &str) -> i64 {
    // 메세지 출력
    println!("{}", prompt);
    // 입력값을 가져옴
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("잘못된 입력입니다.");
    // 공백을 제거하고 숫자 값으로 변환
    return  s.trim().parse().expect("숫자가 아닙니다.");
}

fn main() {
    let n = input("1 부터 몇까지?");

    for i in 1..n+1 {
        println!("fibo({}) = {}",i,fibo(i));
    }
}