fn main() {
    let s = "내 자신에 대한 자신감을 잃으면 온 세상이 나의 적이 된다.";
    println!("수정 전:{}", s);

    // 문자열 치환
    // 치환된 문자열을 String 타입(&str->String)
    // replace() method 는 String 과 &str 에 모두 사용 가능
    let s = s.replace("잃으면","가지면");
    let s = s.replace("적이","편이");
    println!("수정 후:{}", s);
}