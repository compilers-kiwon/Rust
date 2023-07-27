fn main() {
    // 변수 s 에 문장을 대입
    // 문자열 리터럴은 모두 &str 이므로 '+' 연산자를 이용해
    // 결합할 수는 없지만, format! 매크로를 이용하면 쉽게
    // 결합할 수 있다.
    let s = format!("{}{}",
                    "There is more happiness in giving ",
                    "than there is in receiving.");
    // 클로저로 검색
    let res = s.find(|c:char| c.to_ascii_uppercase() == 'S');
    match res {
        Some(i) => println!("S={}B", i),
        None => println!("None"),
    };
}