fn main() {
    let s = "구슬이 서 말이라도 꿰어야 보배";

    // 1 글자씩 표시
    for c in s.chars() {
        print!("[{}]", c);
    }

    // 글자수 세기
    println!("\n글자 수 = {}자", s.chars().count());

    // Vec<char> 로 변환
    let s_chars : Vec<char> = s.chars().collect();
    println!("Vec<char> : {:?}", s_chars);
    for c in s_chars.iter() {
        print!("[{}]", c);
    }
    println!("\n글자 수 = {}자", s_chars.len());
}