use std::io;

fn main() {
    // 계산용 스택
    let mut stack: Vec<f64> = vec![];
    println!("Input RPN :");
    // 표준 입력으로부터 수식 얻기
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("잘못된 입력입니다!");
    // 수식을 빈 칸을 기준으로 분할하여 배열로 만듦
    let tokens = s.split_whitespace();
    // 배열의 각 요소들을 처리
    for tok in tokens {
        // 앞뒤의 불필요한 공백 제거
        let t = tok.trim();
        // f64 타입으로 파싱
        match t.parse::<f64>() {
            Ok(v) => { stack.push(v); continue; },  // Ok 이면 스택에 저장, 다음 요소를 처리
            Err(_) => 0.0   // Err, 숫자가 아니라면 아래에서 연산자인지 확인
        };

        // 연산을 위하여 숫자 두 개를 pop()
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();

        // 연산자라면 계산 후 결과값을 다시 스택에 저장, 아니라면 panic()
        match t {
            "+" => stack.push(a+b),
            "-" => stack.push(a-b),
            "*" => stack.push(a*b),
            "/" => stack.push(a/b),
            _ => panic!("Unexpected operator {}", t)
        };
    }
    // 스택에 남아있는 마지막 결과갑 출력
    println!("{}", stack.pop().unwrap());
}