fn main() {
    let s = "안녕하세요";

    // 첫번째(idx:0) 글자
    let ch = s.chars().nth(0).unwrap();
    println!("{}", ch); // 안

    // 세번째(idx:2) 글자
    let ch = s.chars().nth(2).unwrap();
    println!("{}", ch); // 하
}