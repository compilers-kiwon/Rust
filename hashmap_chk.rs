use std::collections::HashMap;

fn main() {
    // HashMap 생성 & 초기화
    let mut map = HashMap::new();

    map.insert("A",30);
    map.insert("B",50);

    // key 가 존재하지 않는지 확인
    if map.get("D") == None {
        println!("D 는 존재하지 않음");
    } else {
        println!("D = {}", map["D"]);
    }
}