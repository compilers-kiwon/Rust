fn main() {
    // Command Line 인수 츼득
    let args = std::env::args();
    let mut total = 0.0;

    // Command Line 인수들을 순서대로 연산
    for (i, s) in args.enumerate() {
        // 0 번째 인수는 명령어이므로 무시
        if i == 0 { continue; }
        // 인수를 숫자로 변환
        let num: f64 = match s.parse() {
            Ok(_v) => _v,
            Err(_) => 0.0,
        };

        total += num;
    }

    println!("{}", total);
}