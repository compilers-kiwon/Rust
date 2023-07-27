fn main() {
    let s = "제주도의 특산품 중 귤은 겨울에 많이 먹을 수 있다.";

    find_word(s, "귤");     // '귤'을 검색
    find_word(s, "바나나"); // '바나나'를 검색 
}

fn find_word(s: &str, w: &str) {
    match s.find(w) {
        Some(i) => println!("{} = {}B", w, i),
        None => println!("'{}'(이)라는 단어는 없습니다.", w),
    };
}