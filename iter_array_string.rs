fn main() {
    // 문자열로 이루어진 배열
    let array = [
        "Apple".to_string(),
        "Banana".to_string(),
        "Mango".to_string(),
        "Tomato".to_string()
    ];

    // 배열의 각 요소들을 출력
    for a in array.iter() { // "for a in array" 의 경우, array 의 소유권이 이동하여 loop 가 끝날 때 소멸
                            // 소유권 이동없이 참조만 하기 위해, .iter() 를 사용.
        println!("{}", a);
    }

    println!("len={}", array.len());
}