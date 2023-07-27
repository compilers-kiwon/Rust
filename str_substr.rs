fn main() {
    let s = "지혜는 무기보다 가치가 있다.";

    // 앞의 2 글자 가져오기
    let sub_0_2: String = s.chars().take(2).collect();
    println!("앞부터 2글자: {}", sub_0_2);

    // "무기" 가져오기(5번째 & 6번째, ' ' 공백도 한 글자로 취급)
    let s_vec: Vec<char> = s.chars().collect();
    let sub_4_2 = &s_vec[4..6];
    let sub_s: String = sub_4_2.into_iter().collect();
    println!("5-6 번째 글자: {}", sub_s);
}