use std::collections::HashMap;

fn main() {
    // HashMap 생성 & 초기화
    let mut map = HashMap::new();

    map.insert("A",30);
    map.insert("B",50);

    // key 가 존재여부 확인
    match map.get("A") {
        None => println!("A 는 존재하지 않음"),
        Some(_v) => println!("A = {}", map["A"]),
    }
}